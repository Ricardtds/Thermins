<script lang="ts">
  import {
    dynamicSystemState,
    staticSystemState,
    telemetryState,
  } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";

  export let toggleSidebar: () => void = () => {};

  const statusClasses = {
    connecting: "bg-amber-400",
    connected: "bg-emerald-400",
    unavailable: "bg-red-400",
  } as const;
</script>

<header
  class="app-topbar z-30 flex min-h-16 shrink-0 items-center justify-between gap-3 border-b border-white/10 bg-[#111317]/90 px-3 py-2 backdrop-blur-md sm:px-5"
>
  <div class="flex min-w-0 items-center gap-3">
    <button
      type="button"
      class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg border border-zinc-800 bg-zinc-900 text-lg text-zinc-200 transition hover:border-cyan-400 hover:text-cyan-300 lg:hidden"
      onclick={toggleSidebar}
      aria-label="Abrir menu"
    >
      ☰
    </button>

    <div
      class="hidden h-11 w-11 shrink-0 items-center justify-center overflow-hidden rounded-full bg-gradient-to-br from-[#00d1ff] to-[#bd00ff] shadow-[0_0_10px_rgba(189,0,255,0.35)] sm:flex"
    >
      <img src="/Thermins.png" alt="" class="h-full w-full object-cover" />
    </div>

    <div class="min-w-0">
      <p class="truncate text-xs font-bold tracking-wide text-zinc-100">
        {staticSystemState.host.hostName || "Local host"}
      </p>
      <p class="truncate text-[10px] uppercase tracking-[0.14em] text-cyan-300/80">
        {staticSystemState.host.name || "System"}
        {#if staticSystemState.host.osVersion}
          · {staticSystemState.host.osVersion}
        {/if}
        {#if staticSystemState.host.kernelVersion}
          <span class="hidden md:inline">
            · Kernel {staticSystemState.host.kernelVersion}
          </span>
        {/if}
      </p>
    </div>
  </div>

  <div class="flex shrink-0 items-center gap-2 sm:gap-3">
    <div
      class="hidden text-right text-[10px] uppercase tracking-[0.16em] text-zinc-500 sm:block"
    >
      <p>Uptime</p>
      <p class="text-zinc-300">{formatUptime(dynamicSystemState.uptime)}</p>
    </div>

    <div
      class="flex min-h-10 items-center gap-2 rounded-full border border-zinc-800 bg-black/40 px-3 text-[10px] uppercase tracking-[0.12em] text-zinc-300"
      title={telemetryState.message}
      aria-label={telemetryState.message}
    >
      <span
        class={`h-2 w-2 rounded-full ${statusClasses[telemetryState.status]} ${
          telemetryState.status === "connecting" ? "animate-pulse" : ""
        }`}
      ></span>
      <span class="hidden sm:inline">
        {telemetryState.status === "connected"
          ? "Live"
          : telemetryState.status === "connecting"
            ? "Connecting"
            : "Offline"}
      </span>
    </div>
  </div>
</header>
