# Thermins

Monitor local de recursos do sistema construído com Tauri 2, SvelteKit 2,
Svelte 5 e Rust. A interface se adapta a janelas desktop e telas móveis e o
coletor publica amostras de telemetria a cada dois segundos.

## Recursos atuais

- dashboard com CPU, memória, volumes, rede, temperatura e processos;
- lista pesquisável de processos;
- sensores térmicos e bateria com estados vazios para hardware indisponível;
- console de diagnóstico com comandos somente de leitura;
- navegação lateral no desktop e navegação inferior no mobile;
- fallback explícito quando a interface é aberta sem o runtime Tauri.

## Desenvolvimento

Requisitos: Node.js compatível com o toolchain do SvelteKit, pnpm, Rust e os
pré-requisitos de sistema do Tauri 2.

```bash
pnpm install
pnpm tauri dev
```

Verificações usadas antes de uma entrega:

```bash
pnpm check
pnpm build
cd src-tauri
cargo test --lib
cargo clippy --lib -- -D warnings
```

## Arquitetura

O backend coleta os dados com `sysinfo` e publica o evento
`system_snapshot`. O frontend mantém um snapshot estático (metadados de host,
CPU, memória e discos) e outro dinâmico. Discos são correlacionados pelo ponto
de montagem, não pelo nome, pois nomes podem se repetir.

O “Terminal” é intencionalmente um console de diagnóstico, não um shell. A
entrada seleciona comandos exatos de uma allowlist por sistema operacional;
ela nunca é interpolada em `sh`, `cmd` ou PowerShell. A saída também é limitada
para proteger a responsividade da aplicação.

## Plataformas e limitações

- Linux, macOS e Windows: telemetria local e comandos diagnósticos nativos.
- Android/iOS: a interface é responsiva, mas o sandbox móvel não permite um
  shell geral. O console mantém apenas comandos internos seguros.
- Android/iOS: a integração de bateria desktop é desativada. No Android ela não
  oferece backend e, no iOS, depende de IOKit privado e poderia causar rejeição
  na App Store. Um adapter nativo aprovado deve substituí-la.
- Sensores, diretórios de processos e algumas métricas dependem do que cada
  sistema operacional expõe ao aplicativo.

Para monitorar **um PC a partir do celular**, o passo arquitetural recomendado
é separar o projeto em um agente desktop e um cliente móvel, conectados por um
canal autenticado. Um aplicativo móvel isolado não consegue ler diretamente os
recursos de outra máquina.

## Roadmap recomendado

1. Persistência de histórico em SQLite, seleção de período e exportação CSV.
2. Alertas configuráveis de CPU, memória, disco, bateria e temperatura, com
   notificações nativas e intervalo antirruído.
3. Agente remoto desktop com pareamento por QR code, TLS e revogação de
   dispositivos para o cliente mobile.
4. Pausa/redução automática da coleta quando a janela estiver em segundo plano
   e taxa de atualização configurável.
5. Detalhes de processo e ação de encerrar processo com confirmação, política
   de permissões e trilha de auditoria.
6. Saúde SMART de discos, detecção de hot-plug e agrupamento entre disco físico
   e volumes montados.
7. Ícone de bandeja, inicialização com o sistema e alertas em background.
8. Testes de componentes, contratos de serialização Rust/TypeScript e CI para
   Linux, Windows, macOS e targets móveis.
