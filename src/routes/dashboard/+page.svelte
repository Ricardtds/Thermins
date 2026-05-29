<!-- <script lang="ts">
  import CpuChart from "$lib/components/dashboard/CpuChart.svelte";
  import EventTable from "$lib/components/dashboard/EventTable.svelte";
  import HardwareStatus from "$lib/components/dashboard/HardwareStatus.svelte";
  import NetworkWidget from "$lib/components/dashboard/NetworkWidget.svelte";
  import RamWidget from "$lib/components/dashboard/RamWidget.svelte";
  import StorageWidget from "$lib/components/dashboard/StorageWidget.svelte";
</script>

<section
  class="
    grid
    grid-cols-1
    gap-4
    bg-[#0b0c10]
    p-4

    md:grid-cols-2
    xl:grid-cols-12
  "
>
  <CpuChart />

  <RamWidget />

  <StorageWidget />

  <NetworkWidget />

  <EventTable />

  <HardwareStatus />
</section> -->

<script>
  import { onMount } from "svelte";
  import {
    dynamicSystemState,
    staticSystemState,
  } from "$lib/stores/system.svelte";

  $: averageTemperature = dynamicSystemState.components.length
    ? dynamicSystemState.components.reduce(
        (sum, sensor) => sum + sensor.temperature,
        0,
      ) / dynamicSystemState.components.length
    : 0;

  // Props para simular dados dinâmicos do backend Rust
  export let nodeName = "US-EAST-01A";
  export let cpuLoad = 79.0;
  export let activeCores = 12;
  export let temp = 64;
  export let memAlloc = 12.8;
  export let storageUsed = 1.2;
  export let networkThroughput = "1.2 GB/S";
  const GB = 1024 * 1024 * 1024;

  let chartData = [10, 25, 45, 30, 60, 40, 85, 20, 35, 50];
</script>

<div
  class="min-h-screen bg-[#111317] text-white font-['Space_Grotesk'] pb-20 select-none"
>
  <main class="p-4 space-y-4">
    <!-- CPU Quick Stats -->
    <section
      class="bg-[#1a1c20] border border-white/5 rounded-2xl p-5 relative overflow-hidden group"
    >
      <div
        class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity"
      >
        <span class="material-symbols-outlined text-6xl">memory</span>
      </div>
      <div class="flex justify-between items-end mb-4">
        <div>
          <p class="text-[10px] uppercase tracking-[0.2em] text-white/40 mb-1">
            CPU LOAD
          </p>
          <h2 class="text-4xl font-bold tracking-tighter">
            {dynamicSystemState.cpu.usage.toFixed(2)}<span
              class="text-lg text-white/40 ml-1">%</span
            >
          </h2>
        </div>
        <div class="text-right">
          <span class="material-symbols-outlined text-[#bd00ff] text-sm"
            >memory</span
          >
        </div>
      </div>
      <div class="h-2 w-full bg-white/5 rounded-full overflow-hidden mb-3">
        <div
          class="h-full bg-gradient-to-r from-[#00d1ff] via-[#bd00ff] to-[#ff00e5] transition-all"
          style="width: {dynamicSystemState.cpu.usage}%"
        ></div>
      </div>
      <div class="flex justify-between text-[11px] font-mono text-white/60">
        <span>{dynamicSystemState.cpu.cores.length} CORES ACTIVE</span>
        <span>TEMP: {averageTemperature.toFixed(0)}°C</span>
      </div>
    </section>

    <!-- Mem & Storage Grid -->
    <div class="grid grid-cols-2 gap-4">
      <div class="bg-[#1a1c20] border border-white/5 rounded-2xl p-4">
        <div class="flex justify-between items-start mb-3">
          <p class="text-[9px] uppercase tracking-widest text-white/40">
            MEM ALLOC
          </p>
          <span class="material-symbols-outlined text-xs text-[#00d1ff]"
            >database</span
          >
        </div>
        <div class="flex justify-between">
          <div class="text-center">
            <span class="text-xs text-white/90 ml-1">total</span>
            <h3 class="text-xl font-bold mb-3">
              {(staticSystemState.memory.total / GB).toFixed(1)}
              <span class="text-xs text-white/40 ml-1">GB</span>
            </h3>
          </div>
          <div class="text-center">
            <span class="text-xs text-white/90 ml-1">Used</span>
            <h3 class="text-xl font-bold mb-3">
              {(dynamicSystemState.memory.used / GB).toFixed(1)}
              <span class="text-xs text-white/40 ml-1">GB</span>
            </h3>
          </div>
          <div class="text-center">
            <span class="text-xs text-white/90 ml-1">Free</span>
            <h3 class="text-xl font-bold mb-3">
              {(dynamicSystemState.memory.freeMemory / GB).toFixed(1)}
              <span class="text-xs text-white/40 ml-1">GB</span>
            </h3>
          </div>
        </div>
        <div class="h-1.5 w-full bg-white/5 rounded-full overflow-hidden mb-2">
          <div
            class="h-full bg-[#00d1ff]"
            style="width: {dynamicSystemState.memory.usagePercent}%"
          ></div>
        </div>
        <div class="flex gap-3">
          <p class="text-[9px] text-white/40 font-mono">
            SWAP: {(dynamicSystemState.memory.usedSwap / GB).toFixed(1)}GB
          </p>
          <p class="text-[9px] text-white/40 font-mono">
            Total: {(staticSystemState.memory.totalSwap / GB).toFixed(1)} GB
          </p>
        </div>
      </div>

      <div class="bg-[#1a1c20] border border-white/5 rounded-2xl p-4">
        {#each dynamicSystemState.disks as disk}
          {@const staticDisk = staticSystemState.disks.find(
            (b) => b.name === disk.name,
          )}
          {#if staticDisk}
            <div class="flex justify-between items-start mb-3">
              <p class="text-[9px] uppercase tracking-widest text-white/40">
                STORAGE {disk.name}
                {staticDisk.filesystem}
              </p>
              <span class="material-symbols-outlined text-xs text-[#bd00ff]"
                >hard_drive</span
              >
            </div>
            <h3 class="text-xl font-bold mb-3">
              {(staticDisk.totalSpace / GB).toFixed(2)}<span
                class="text-xs text-white/40 ml-1">GB</span
              >
            </h3>
            <div
              class="h-1.5 w-full bg-white/5 rounded-full overflow-hidden mb-2"
            >
              <div class="h-full bg-[#bd00ff]" style="width: 60%"></div>
            </div>
            <p class="text-[9px] text-white/40 font-mono">
              FREE: {(disk.availableSpace / GB).toFixed(1)} GB
            </p>
          {/if}
        {/each}
      </div>
    </div>

    <!-- Real-time Activity Stream -->
    <section class="bg-[#1a1c20] border border-white/5 rounded-2xl p-5">
      <div class="flex justify-between items-center mb-6">
        <h3
          class="text-[10px] uppercase tracking-[0.3em] font-bold text-white/90 flex items-center gap-2"
        >
          <span class="w-2 h-2 rounded-full bg-[#bd00ff] animate-pulse"></span>
          CORE_ACTIVITY_STREAM
        </h3>
        <span class="text-[9px] font-mono text-white/20">LIVE</span>
      </div>

      <div class="h-32 flex items-end gap-2 px-2 relative group">
        {#each dynamicSystemState.cpu.cores as core}
          <div class="flex flex-1 flex-col items-center gap-1">
            <!-- BAR CONTAINER -->
            <div class="h-24 w-full flex items-end justify-center">
              <!-- BAR FILL -->
              <div
                class="w-full bg-gradient-to-t from-[#bd00ff]/20 to-[#bd00ff]/80 rounded-t-sm transition-all duration-300"
                style={`height: ${core.usage}%`}
              >
                <div class="text-[10px] text-white font-mono text-center">
                  {core.usage.toFixed()}%
                </div>
              </div>
            </div>

            <!-- NAME -->
            <span class="text-[10px] text-zinc-400 font-mono">
              {core.name}
            </span>
          </div>
        {/each}

        <!-- OVERLAY -->
        <div
          class="absolute inset-0 opacity-80 flex items-center justify-center pointer-events-none"
        >
          <!-- <div
            class="bg-[#111317]/80 backdrop-blur-md border border-[#bd00ff]/30 px-3 py-1 rounded text-[10px] font-mono text-[#bd00ff] shadow-xl"
          >
            PEAK_DETECTED: 92.4%
          </div> -->
        </div>
      </div>

      <div class="flex justify-between mt-4 text-[9px] font-mono text-white/40">
        <span>04:12:00</span>
        <span class="text-[#bd00ff] font-bold">SYSTEM_STABLE</span>
        <span>04:13:00</span>
      </div>
    </section>

    <!-- Top Tasks -->
    <section class="bg-[#1a1c20] border border-white/5 rounded-2xl p-5">
      <div class="flex justify-between items-center mb-4">
        <h3
          class="text-[10px] uppercase tracking-[0.2em] font-bold text-white/90"
        >
          TOP_TASKS
        </h3>
        <span class="material-symbols-outlined text-white/40 text-sm"
          >filter_list</span
        >
      </div>

      <div class="space-y-3">
        {#each [{ name: "k_worker/0:1", pid: 1042, load: 12.4, color: "#00d1ff", icon: "sync" }, { name: "system_d_sec", pid: 882, load: 8.1, color: "#bd00ff", icon: "shield" }, { name: "node_exporter", pid: 219, load: 4.2, color: "#ff00e5", icon: "cloud" }] as task}
          <div
            class="flex justify-between items-center p-3 bg-white/2 rounded-xl border border-white/5"
          >
            <div class="flex items-center gap-3">
              <span
                class="material-symbols-outlined text-sm"
                style="color: {task.color}">{task.icon}</span
              >
              <div>
                <p class="text-xs font-bold text-white/90">{task.name}</p>
                <p
                  class="text-[9px] font-mono text-white/40 uppercase tracking-tighter"
                >
                  PID: {task.pid}
                </p>
              </div>
            </div>
            <div class="text-right">
              <p
                class="text-xs font-mono font-bold"
                style="color: {task.color}"
              >
                {task.load}%
              </p>
            </div>
          </div>
        {/each}
      </div>

      <button
        class="w-full mt-4 py-3 text-[10px] font-bold uppercase tracking-widest text-[#bd00ff] hover:bg-[#bd00ff]/5 rounded-xl transition-colors"
      >
        VIEW_ALL_PROCESSES
      </button>
    </section>

    <!-- Micro Stats -->
    <div class="grid grid-cols-2 gap-4">
      <div
        class="bg-[#1a1c20] border border-white/5 rounded-xl p-4 flex items-center gap-3"
      >
        <div
          class="w-10 h-10 bg-white/5 rounded-lg flex items-center justify-center"
        >
          <span class="material-symbols-outlined text-[#00d1ff]">wifi</span>
        </div>
        <div>
          <p class="text-[8px] text-white/40 uppercase tracking-widest">
            NETWORK
          </p>
          <p class="text-xs font-bold">{networkThroughput}</p>
        </div>
      </div>
      <div
        class="bg-[#1a1c20] border border-white/5 rounded-xl p-4 flex items-center gap-3"
      >
        <div
          class="w-10 h-10 bg-white/5 rounded-lg flex items-center justify-center"
        >
          <span class="material-symbols-outlined text-[#00d1ff]"
            >thermostat</span
          >
        </div>
        <div>
          <p class="text-[8px] text-white/40 uppercase tracking-widest">
            NODE_TEMP
          </p>
          <p class="text-xs font-bold">42.8 °C</p>
        </div>
      </div>
    </div>
  </main>
</div>
