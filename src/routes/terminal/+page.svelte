<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, isTauri } from "@tauri-apps/api/core";

  type TerminalCapabilities = {
    available: boolean;
    platform: string;
    message: string;
    commands: string[];
  };

  type TerminalCommandResult = {
    command: string;
    stdout: string;
    stderr: string;
    success: boolean;
    exitCode: number | null;
    truncated: boolean;
  };

  type EntryKind = "command" | "output" | "error" | "system";

  type TerminalEntry = {
    id: number;
    kind: EntryKind;
    text: string;
  };

  const fallbackCommands = ["help", "clear", "platform"];

  let capabilities = $state<TerminalCapabilities>({
    available: false,
    platform: "web",
    message: "Verificando os recursos disponíveis…",
    commands: fallbackCommands,
  });
  let entries = $state<TerminalEntry[]>([
    {
      id: 0,
      kind: "system",
      text: "Thermins Diagnostic Console — modo de leitura",
    },
  ]);
  let input = $state("");
  let loading = $state(true);
  let running = $state(false);
  let history = $state<string[]>([]);
  let historyIndex = $state(0);
  let nextEntryId = 1;
  let outputElement: HTMLDivElement | undefined = $state();
  let inputElement: HTMLInputElement | undefined = $state();

  const prompt = $derived(`thermins@${capabilities.platform}:~$`);
  const suggestedCommands = $derived(
    capabilities.commands.filter(
      (command) => !["help", "clear", "platform", "pwd"].includes(command),
    ),
  );

  onMount(() => {
    void loadCapabilities();
  });

  async function loadCapabilities() {
    if (!isTauri()) {
      capabilities = {
        available: false,
        platform: "browser",
        message:
          "Abra o aplicativo Tauri para executar diagnósticos nativos. `help`, `platform` e `clear` continuam disponíveis.",
        commands: fallbackCommands,
      };
      appendEntry("system", capabilities.message);
      loading = false;
      return;
    }

    try {
      capabilities = await invoke<TerminalCapabilities>(
        "get_terminal_capabilities",
      );
      appendEntry("system", capabilities.message);
    } catch (error) {
      capabilities = {
        available: false,
        platform: "tauri",
        message: `Não foi possível inicializar o console: ${errorMessage(error)}`,
        commands: fallbackCommands,
      };
      appendEntry("error", capabilities.message);
    } finally {
      loading = false;
      scrollToEnd();
      inputElement?.focus();
    }
  }

  async function submitCommand(event: SubmitEvent) {
    event.preventDefault();
    await executeCommand(input);
  }

  async function executeCommand(rawCommand: string) {
    const command = rawCommand.trim();
    if (!command || loading || running) return;

    input = "";
    history = [...history.filter((item) => item !== command), command];
    historyIndex = history.length;
    appendEntry("command", `${prompt} ${command}`);

    const normalized = command.replace(/\s+/g, " ").toLowerCase();

    if (normalized === "clear") {
      entries = [];
      inputElement?.focus();
      return;
    }

    if (normalized === "help") {
      appendEntry(
        "output",
        `Comandos disponíveis em ${capabilities.platform}:\n${capabilities.commands.join("\n")}`,
      );
      scrollToEnd();
      inputElement?.focus();
      return;
    }

    if (normalized === "platform") {
      appendEntry(
        "output",
        `plataforma: ${capabilities.platform}\nexecução nativa: ${capabilities.available ? "disponível (restrita)" : "indisponível"}`,
      );
      scrollToEnd();
      inputElement?.focus();
      return;
    }

    if (!capabilities.available) {
      appendEntry("error", capabilities.message);
      scrollToEnd();
      inputElement?.focus();
      return;
    }

    running = true;

    try {
      const result = await invoke<TerminalCommandResult>(
        "run_terminal_command",
        { command },
      );

      if (result.stdout) appendEntry("output", result.stdout.trimEnd());
      if (result.stderr) appendEntry("error", result.stderr.trimEnd());

      if (!result.success) {
        appendEntry(
          "error",
          `O processo terminou com código ${result.exitCode ?? "desconhecido"}.`,
        );
      }

      if (result.truncated) {
        appendEntry(
          "system",
          "A saída foi limitada a 128 KiB para preservar a responsividade.",
        );
      }

      if (!result.stdout && !result.stderr && result.success) {
        appendEntry("system", "Comando concluído sem saída.");
      }
    } catch (error) {
      appendEntry("error", errorMessage(error));
    } finally {
      running = false;
      scrollToEnd();
      inputElement?.focus();
    }
  }

  function handleInputKeydown(event: KeyboardEvent) {
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "l") {
      event.preventDefault();
      entries = [];
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (historyIndex > 0) {
        historyIndex -= 1;
        input = history[historyIndex] ?? "";
      }
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (historyIndex < history.length - 1) {
        historyIndex += 1;
        input = history[historyIndex] ?? "";
      } else {
        historyIndex = history.length;
        input = "";
      }
    }
  }

  function appendEntry(kind: EntryKind, text: string) {
    entries = [...entries, { id: nextEntryId++, kind, text }];
  }

  function scrollToEnd() {
    requestAnimationFrame(() => {
      if (outputElement) outputElement.scrollTop = outputElement.scrollHeight;
    });
  }

  function errorMessage(error: unknown) {
    if (error instanceof Error) return error.message;
    return String(error);
  }
</script>

<svelte:head>
  <title>Terminal | Thermins</title>
</svelte:head>

<section class="flex h-full min-h-0 flex-col bg-[#07090f] p-3 sm:p-5 lg:p-8">
  <div class="mx-auto flex min-h-0 w-full max-w-7xl flex-1 flex-col overflow-hidden border border-zinc-800 bg-[#0b0d12] shadow-[0_0_40px_rgba(0,209,255,0.04)]">
    <header class="flex flex-wrap items-center justify-between gap-3 border-b border-zinc-800 bg-[#111317] px-3 py-3 sm:px-5">
      <div class="min-w-0">
        <div class="flex items-center gap-2">
          <span
            class={`h-2.5 w-2.5 rounded-full ${capabilities.available ? "bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.65)]" : "bg-amber-400"}`}
            aria-hidden="true"
          ></span>
          <h1 class="truncate text-xs font-bold uppercase tracking-[0.22em] text-zinc-100 sm:text-sm">
            Diagnostic Terminal
          </h1>
        </div>
        <p class="mt-1 truncate text-[10px] uppercase tracking-wider text-zinc-500 sm:text-xs">
          {capabilities.platform} · read-only allowlist
        </p>
      </div>

      <span
        class={`border px-2 py-1 text-[9px] font-bold uppercase tracking-[0.18em] sm:text-[10px] ${capabilities.available ? "border-emerald-500/30 bg-emerald-500/10 text-emerald-300" : "border-amber-500/30 bg-amber-500/10 text-amber-300"}`}
      >
        {loading
          ? "checking"
          : capabilities.available
            ? "desktop ready"
            : "local commands"}
      </span>
    </header>

    {#if suggestedCommands.length > 0}
      <div class="flex shrink-0 gap-2 overflow-x-auto border-b border-zinc-800 px-3 py-2 sm:px-5" aria-label="Comandos sugeridos">
        {#each suggestedCommands as command}
          <button
            type="button"
            class="shrink-0 border border-zinc-800 bg-zinc-950 px-2.5 py-1.5 text-[10px] text-zinc-400 transition hover:border-cyan-500/60 hover:text-cyan-300 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-400 disabled:cursor-not-allowed disabled:opacity-40"
            disabled={loading || running}
            onclick={() => void executeCommand(command)}
          >
            {command}
          </button>
        {/each}
      </div>
    {/if}

    <div
      bind:this={outputElement}
      class="min-h-0 flex-1 overflow-auto p-3 font-mono sm:p-5"
      role="log"
      aria-live="polite"
      aria-label="Saída do terminal"
    >
      {#if entries.length === 0}
        <p class="text-xs text-zinc-600">Console limpo. Digite <span class="text-cyan-400">help</span> para começar.</p>
      {/if}

      {#each entries as entry (entry.id)}
        <pre
          class={`mb-2 min-w-max whitespace-pre-wrap break-words text-[11px] leading-5 sm:text-xs ${
            entry.kind === "command"
              ? "text-cyan-300"
              : entry.kind === "error"
                ? "text-rose-300"
                : entry.kind === "system"
                  ? "text-amber-200/80"
                  : "text-zinc-300"
          }`}>{entry.text}</pre>
      {/each}

      {#if running}
        <div class="flex items-center gap-2 text-[11px] text-zinc-500" aria-label="Executando comando">
          <span class="h-2 w-2 animate-pulse rounded-full bg-cyan-400"></span>
          executando diagnóstico…
        </div>
      {/if}
    </div>

    <form
      class="shrink-0 border-t border-zinc-800 bg-black/40 p-3 sm:p-4"
      onsubmit={submitCommand}
    >
      <label for="terminal-command" class="sr-only">Comando de diagnóstico</label>
      <div class="flex min-w-0 items-center gap-2 border border-zinc-700 bg-black px-3 transition focus-within:border-cyan-500/70 sm:gap-3">
        <span class="hidden shrink-0 text-[11px] text-cyan-400 sm:inline">{prompt}</span>
        <span class="shrink-0 text-cyan-400 sm:hidden" aria-hidden="true">$</span>
        <input
          bind:this={inputElement}
          bind:value={input}
          id="terminal-command"
          type="text"
          class="min-w-0 flex-1 bg-transparent py-3 font-mono text-xs text-zinc-100 outline-none placeholder:text-zinc-700 sm:text-sm"
          placeholder={loading ? "Inicializando…" : "Digite help"}
          disabled={loading || running}
          maxlength="128"
          autocomplete="off"
          autocapitalize="none"
          spellcheck="false"
          onkeydown={handleInputKeydown}
        />
        <button
          type="submit"
          class="shrink-0 border-l border-zinc-800 px-2 py-1 text-[10px] font-bold uppercase tracking-wider text-cyan-300 transition hover:text-white focus-visible:outline-2 focus-visible:outline-cyan-400 disabled:cursor-not-allowed disabled:text-zinc-700 sm:px-3"
          disabled={loading || running || !input.trim()}
        >
          Run
        </button>
      </div>
      <p class="mt-2 text-[9px] leading-4 text-zinc-600 sm:text-[10px]">
        ↑/↓ histórico · Ctrl/⌘+L limpa · sem shell, pipes ou redirecionamentos
      </p>
    </form>
  </div>
</section>
