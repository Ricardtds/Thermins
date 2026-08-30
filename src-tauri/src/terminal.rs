use serde::Serialize;
use std::{
    io::Read,
    process::Stdio,
    thread,
    time::{Duration, Instant},
};

const MAX_COMMAND_LENGTH: usize = 128;
const MAX_OUTPUT_BYTES: usize = 128 * 1024;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalCapabilities {
    available: bool,
    platform: &'static str,
    message: &'static str,
    commands: Vec<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalCommandResult {
    command: String,
    stdout: String,
    stderr: String,
    success: bool,
    exit_code: Option<i32>,
    truncated: bool,
}

#[derive(Clone, Copy)]
struct CommandSpec {
    program: &'static str,
    args: &'static [&'static str],
}

#[tauri::command]
pub fn get_terminal_capabilities() -> TerminalCapabilities {
    TerminalCapabilities {
        available: !is_mobile(),
        platform: platform_name(),
        message: platform_message(),
        commands: supported_commands(),
    }
}

/// Runs a deliberately small set of read-only diagnostic commands.
///
/// The user input is never forwarded to a shell. It only selects an exact
/// allow-listed command whose executable and arguments are defined below.
#[tauri::command]
pub async fn run_terminal_command(command: String) -> Result<TerminalCommandResult, String> {
    if command.len() > MAX_COMMAND_LENGTH {
        return Err(format!(
            "O comando excede o limite de {MAX_COMMAND_LENGTH} caracteres."
        ));
    }

    let normalized = normalize_command(&command);
    if normalized.is_empty() {
        return Err("Digite um comando.".to_owned());
    }

    if let Some(result) = run_builtin(&normalized) {
        return Ok(result);
    }

    if is_mobile() {
        return Err(
            "A execução de processos do sistema não está disponível no sandbox do Android/iOS."
                .to_owned(),
        );
    }

    tauri::async_runtime::spawn_blocking(move || run_desktop_command(&normalized))
        .await
        .map_err(|error| format!("Falha interna ao executar o comando: {error}"))?
}

fn run_builtin(command: &str) -> Option<TerminalCommandResult> {
    let output = match command {
        "help" => supported_commands().join("  \n"),
        "platform" => format!(
            "plataforma: {}\narquitetura: {}\nexecução nativa: {}",
            platform_name(),
            std::env::consts::ARCH,
            if is_mobile() {
                "indisponível no sandbox móvel"
            } else {
                "disponível (modo diagnóstico restrito)"
            }
        ),
        "pwd" if !is_mobile() => match std::env::current_dir() {
            Ok(path) => path.display().to_string(),
            Err(error) => format!("Não foi possível obter o diretório atual: {error}"),
        },
        "clear" => String::new(),
        _ => return None,
    };

    Some(TerminalCommandResult {
        command: command.to_owned(),
        stdout: output,
        stderr: String::new(),
        success: true,
        exit_code: Some(0),
        truncated: false,
    })
}

fn run_desktop_command(command: &str) -> Result<TerminalCommandResult, String> {
    let spec = command_spec(command).ok_or_else(|| {
        format!(
            "Comando não permitido. Use `help` para ver os comandos de diagnóstico disponíveis em {}.",
            platform_name()
        )
    })?;

    let program = resolve_program(spec.program)?;
    let mut process = std::process::Command::new(&program);
    process
        .args(spec.args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Prevent a transient console window when running the packaged app on Windows.
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        process.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = process
        .spawn()
        .map_err(|error| format!("Não foi possível iniciar `{}`: {error}", spec.program))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Não foi possível capturar a saída do comando.".to_owned())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Não foi possível capturar os erros do comando.".to_owned())?;

    // Drain both pipes concurrently to prevent a verbose process from blocking
    // on a full OS pipe while retaining at most MAX_OUTPUT_BYTES per stream.
    let stdout_reader = thread::spawn(move || read_limited(stdout));
    let stderr_reader = thread::spawn(move || read_limited(stderr));
    let deadline = Instant::now() + COMMAND_TIMEOUT;

    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("Falha ao aguardar o comando: {error}"))?
        {
            break status;
        }

        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();

            return Err(format!(
                "O comando excedeu o limite de {} segundos e foi interrompido.",
                COMMAND_TIMEOUT.as_secs()
            ));
        }

        thread::sleep(Duration::from_millis(20));
    };

    let (stdout, stdout_truncated) = stdout_reader
        .join()
        .map_err(|_| "Falha interna ao processar a saída do comando.".to_owned())?
        .map_err(|error| format!("Falha ao ler a saída do comando: {error}"))?;
    let (stderr, stderr_truncated) = stderr_reader
        .join()
        .map_err(|_| "Falha interna ao processar os erros do comando.".to_owned())?
        .map_err(|error| format!("Falha ao ler os erros do comando: {error}"))?;

    Ok(TerminalCommandResult {
        command: command.to_owned(),
        stdout,
        stderr,
        success: status.success(),
        exit_code: status.code(),
        truncated: stdout_truncated || stderr_truncated,
    })
}

fn normalize_command(command: &str) -> String {
    command
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn read_limited<R: Read>(mut reader: R) -> std::io::Result<(String, bool)> {
    let mut retained = Vec::with_capacity(MAX_OUTPUT_BYTES);
    let mut buffer = [0_u8; 8 * 1024];
    let mut truncated = false;

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        let remaining = MAX_OUTPUT_BYTES.saturating_sub(retained.len());
        let keep = remaining.min(read);
        retained.extend_from_slice(&buffer[..keep]);
        truncated |= keep < read;
    }

    Ok((String::from_utf8_lossy(&retained).into_owned(), truncated))
}

#[cfg(target_os = "linux")]
fn command_spec(command: &str) -> Option<CommandSpec> {
    match command {
        "whoami" => Some(CommandSpec {
            program: "whoami",
            args: &[],
        }),
        "hostname" => Some(CommandSpec {
            program: "hostname",
            args: &[],
        }),
        "date" => Some(CommandSpec {
            program: "date",
            args: &["--iso-8601=seconds"],
        }),
        "uptime" => Some(CommandSpec {
            program: "uptime",
            args: &["-p"],
        }),
        "uname" | "uname -a" => Some(CommandSpec {
            program: "uname",
            args: &["-a"],
        }),
        "df" | "df -h" => Some(CommandSpec {
            program: "df",
            args: &["-h"],
        }),
        "free" | "free -h" => Some(CommandSpec {
            program: "free",
            args: &["-h"],
        }),
        "ps" | "ps aux" => Some(CommandSpec {
            program: "ps",
            args: &["-eo", "pid,pcpu,pmem,comm", "--sort=-pcpu"],
        }),
        "ip" | "ip addr" | "ip address" => Some(CommandSpec {
            program: "ip",
            args: &["-brief", "address"],
        }),
        _ => None,
    }
}

#[cfg(target_os = "macos")]
fn command_spec(command: &str) -> Option<CommandSpec> {
    match command {
        "whoami" => Some(CommandSpec {
            program: "whoami",
            args: &[],
        }),
        "hostname" => Some(CommandSpec {
            program: "hostname",
            args: &[],
        }),
        "date" => Some(CommandSpec {
            program: "date",
            args: &["+%Y-%m-%dT%H:%M:%S%z"],
        }),
        "uptime" => Some(CommandSpec {
            program: "uptime",
            args: &[],
        }),
        "uname" | "uname -a" => Some(CommandSpec {
            program: "uname",
            args: &["-a"],
        }),
        "df" | "df -h" => Some(CommandSpec {
            program: "df",
            args: &["-h"],
        }),
        "ps" | "ps aux" => Some(CommandSpec {
            program: "ps",
            args: &["-Ao", "pid,%cpu,%mem,comm", "-r"],
        }),
        "ifconfig" => Some(CommandSpec {
            program: "ifconfig",
            args: &[],
        }),
        _ => None,
    }
}

#[cfg(target_os = "windows")]
fn command_spec(command: &str) -> Option<CommandSpec> {
    match command {
        "whoami" => Some(CommandSpec {
            program: "whoami.exe",
            args: &[],
        }),
        "hostname" => Some(CommandSpec {
            program: "hostname.exe",
            args: &[],
        }),
        "systeminfo" => Some(CommandSpec {
            program: "systeminfo.exe",
            args: &[],
        }),
        "tasklist" | "ps" => Some(CommandSpec {
            program: "tasklist.exe",
            args: &[],
        }),
        "ipconfig" => Some(CommandSpec {
            program: "ipconfig.exe",
            args: &[],
        }),
        _ => None,
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn command_spec(_command: &str) -> Option<CommandSpec> {
    None
}

#[cfg(unix)]
fn resolve_program(program: &str) -> Result<std::path::PathBuf, String> {
    const SYSTEM_PATHS: [&str; 4] = ["/usr/bin", "/bin", "/usr/sbin", "/sbin"];

    SYSTEM_PATHS
        .iter()
        .map(|directory| std::path::Path::new(directory).join(program))
        .find(|path| path.is_file())
        .ok_or_else(|| format!("O utilitário de sistema `{program}` não foi encontrado."))
}

#[cfg(target_os = "windows")]
fn resolve_program(program: &str) -> Result<std::path::PathBuf, String> {
    let windows_directory = std::env::var_os("SystemRoot")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Windows"));
    let path = windows_directory.join("System32").join(program);

    if path.is_file() {
        Ok(path)
    } else {
        Err(format!(
            "O utilitário de sistema `{program}` não foi encontrado."
        ))
    }
}

fn is_mobile() -> bool {
    cfg!(any(target_os = "android", target_os = "ios"))
}

fn platform_name() -> &'static str {
    std::env::consts::OS
}

fn platform_message() -> &'static str {
    if is_mobile() {
        "Android e iOS não expõem um shell geral aos aplicativos. Os comandos locais continuam disponíveis."
    } else {
        "Console diagnóstico ativo. Somente comandos de leitura previamente autorizados são executados."
    }
}

#[cfg(target_os = "linux")]
fn supported_commands() -> Vec<&'static str> {
    vec![
        "help",
        "clear",
        "platform",
        "pwd",
        "whoami",
        "hostname",
        "date",
        "uptime",
        "uname -a",
        "df -h",
        "free -h",
        "ps aux",
        "ip address",
    ]
}

#[cfg(target_os = "macos")]
fn supported_commands() -> Vec<&'static str> {
    vec![
        "help", "clear", "platform", "pwd", "whoami", "hostname", "date", "uptime", "uname -a",
        "df -h", "ps aux", "ifconfig",
    ]
}

#[cfg(target_os = "windows")]
fn supported_commands() -> Vec<&'static str> {
    vec![
        "help",
        "clear",
        "platform",
        "pwd",
        "whoami",
        "hostname",
        "systeminfo",
        "tasklist",
        "ipconfig",
    ]
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn supported_commands() -> Vec<&'static str> {
    vec!["help", "clear", "platform"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_whitespace_and_case() {
        assert_eq!(normalize_command("  UNAME    -A "), "uname -a");
    }

    #[test]
    fn rejects_shell_syntax_and_unlisted_commands() {
        assert!(command_spec("uname; rm -rf /tmp/example").is_none());
        assert!(command_spec("sh -c whoami").is_none());
        assert!(command_spec("env").is_none());
    }

    #[test]
    fn truncates_large_output() {
        let bytes = vec![b'x'; MAX_OUTPUT_BYTES + 1];
        let (output, truncated) = read_limited(std::io::Cursor::new(bytes)).unwrap();

        assert!(truncated);
        assert_eq!(output.len(), MAX_OUTPUT_BYTES);
    }
}
