<script lang="ts">
  import {
    dynamicSystemState,
    staticSystemState,
  } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";

  export let toggleSidebar: () => void = () => {};
</script>

<!-- <header
  class="flex h-16 items-center justify-between border-b border-zinc-800 bg-[#0d0d12] px-4 sm:px-6"
>
  <div class="flex min-w-0 flex-1 items-center gap-6">
    <button
      class="rounded-md border border-zinc-800 bg-zinc-900 px-3 py-2 text-white transition hover:border-cyan-400"
      on:click={toggleSidebar}
      aria-label="Abrir menu"
    >
      ☰
    </button>

    <div
      class="flex items-center gap-2 text-xs uppercase tracking-[0.25em] text-cyan-300"
    >
      <div class="h-2 w-2 rounded-full bg-cyan-400"></div>

      <span>
        {formatUptime(dynamicSystemState.uptime)}
      </span>
    </div>

    <div
      class="hidden min-w-0 flex-1 items-center gap-4 text-sm text-zinc-400 md:flex"
    >
      <span class="truncate">
        {staticSystemState.host.hostName}
      </span>

      <span class="text-zinc-600">•</span>

      <span>
        Kernel {staticSystemState.host.kernelVersion}
      </span>

      <span class="text-zinc-600">•</span>

      <span>
        {staticSystemState.host.name}
        {staticSystemState.host.osVersion}
      </span>
    </div>
  </div>

  <div class="flex items-center gap-2">
    <button
      class="flex h-10 w-10 items-center justify-center rounded-md border border-zinc-800 bg-zinc-900 text-zinc-400 transition hover:border-cyan-400 hover:text-cyan-300"
    >
      ⚙
    </button>

    <button
      class="flex h-10 w-10 items-center justify-center rounded-md border border-zinc-800 bg-zinc-900 text-zinc-400 transition hover:border-cyan-400 hover:text-cyan-300"
    >
      ⌁
    </button>

    <div
      class="flex h-10 w-10 items-center justify-center rounded-md border border-zinc-700 bg-zinc-900 text-xs font-medium text-cyan-300"
    >
      RT
    </div>
  </div>
</header> -->

<!-- Header -->
<header
  class="flex flex-col justify-between items-center p-1 border-b border-white/10 bg-[#111317]/80 backdrop-blur-md sticky top-0 z-50"
>
  <div class="flex w-full items-center gap-3 justify-between">
    <div
      class="w-12 h-12 bg-gradient-to-br from-[#00d1ff] to-[#bd00ff] rounded-full flex items-center justify-center shadow-[0_0_10px_rgba(189,0,255,0.4)]"
    >
      <img src="/Thermins.png" />
      <!-- <span class="font-bold text-xs italic">Thermins</span> -->
    </div>
    <div class="flex flex-col justify-center items-center">
      <div class="flex items-center justify-between w-full">
        <h1 class="text-sm font-bold tracking-tighter text-white/90">
          Thermins
        </h1>
        {staticSystemState.host.name} -
        {staticSystemState.host.osVersion}
      </div>
      <p class="text-[10px] text-[#00d1ff] font-mono tracking-widest uppercase">
        {staticSystemState.host.hostName}
      </p>
      <p class="text-[10px] text-[#00d1ff] font-mono tracking-widest uppercase">
        KERNEL: {staticSystemState.host.kernelVersion}
      </p>
    </div>
    <button class="p-2 hover:bg-white/5 rounded-lg transition-colors">
      <span class="material-symbols-outlined text-white/60"
        >settings_input_component</span
      >
    </button>
  </div>
</header>
