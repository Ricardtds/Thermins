<script lang="ts">
  import SystemIcon from "$lib/components/ui/SystemIcon.svelte";
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

  const GB = 1024 * 1024 * 1024;
  const taskColors = ["#00d1ff", "#bd00ff", "#ff00e5"];

  $: topProcesses = dynamicSystemState.processes.slice(0, 3);
  $: networkThroughput = dynamicSystemState.networks.reduce(
    (total, network) => total + network.received + network.transmitted,
    0,
  );
</script>

<div class="min-h-full bg-[#111317] text-white">
  <main class="mx-auto max-w-[1600px] space-y-4 p-3 sm:p-4 lg:p-6">
    <!-- CPU Quick Stats -->
    <section
      class="bg-[#1a1c20] border border-white/5 rounded-2xl p-5 relative overflow-hidden group"
    >
      <div
        class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity"
      >
        <SystemIcon name="memory" size={64} />
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
          <SystemIcon name="memory" size={16} class="text-[#bd00ff]" />
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
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <div class="bg-[#1a1c20] border border-white/5 rounded-2xl p-4">
        <div class="flex justify-between items-start mb-3">
          <p class="text-[9px] uppercase tracking-widest text-white/40">
            MEM ALLOC
          </p>
          <SystemIcon name="database" size={16} class="text-[#00d1ff]" />
        </div>
        <div class="grid grid-cols-3 gap-2">
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
            class="h-full bg-[#00d1ff] transition-all"
            style="width: {Math.min(
              100,
              Math.max(0, dynamicSystemState.memory.usagePercent),
            )}%"
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

      <div class="rounded-2xl border border-white/5 bg-[#1a1c20] p-4">
        <div class="space-y-3">
          {#each dynamicSystemState.disks as disk (disk.mountPoint)}
            {@const usedSpace = disk.totalSpace - disk.availableSpace}
            {@const usagePercent = disk.totalSpace > 0
              ? Math.min(
                  100,
                  Math.max(0, (usedSpace / disk.totalSpace) * 100),
                )
              : 0}
            <article class="rounded-xl border border-white/5 bg-black/15 p-3">
                <div class="mb-3 flex items-start justify-between gap-3">
                  <p
                    class="min-w-0 break-all text-[9px] uppercase tracking-widest text-white/40"
                  >
                    STORAGE {disk.name} · {disk.mountPoint}
                  </p>
                  <SystemIcon
                    name="hard_drive"
                    size={16}
                    class="shrink-0 text-[#bd00ff]"
                  />
                </div>
                <h3 class="mb-3 text-xl font-bold">
                  {(disk.totalSpace / GB).toFixed(2)}<span
                    class="ml-1 text-xs text-white/40">GB</span
                  >
                </h3>
                <div
                  class="mb-2 h-1.5 w-full overflow-hidden rounded-full bg-white/5"
                >
                  <div
                    class="h-full bg-[#bd00ff] transition-all"
                    style="width: {usagePercent}%"
                  ></div>
                </div>
                <p class="text-[9px] font-mono text-white/40">
                  FREE: {(disk.availableSpace / GB).toFixed(1)} GB · {disk.filesystem}
                </p>
            </article>
          {:else}
            <p class="py-6 text-center text-xs text-white/40">
              Waiting for storage telemetry…
            </p>
          {/each}
        </div>
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

      <div class="overflow-x-auto pb-2">
        <div
          class="relative flex h-32 items-end gap-2 px-2"
          style={`min-width: ${Math.max(320, dynamicSystemState.cpu.cores.length * 48)}px`}
        >
          {#each dynamicSystemState.cpu.cores as core (core.name)}
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
          {:else}
            <p class="m-auto text-xs text-white/40">
              Waiting for CPU telemetry…
            </p>
          {/each}
        </div>
      </div>

      <div class="flex justify-between mt-4 text-[9px] font-mono text-white/40">
        <span>{dynamicSystemState.cpu.cores.length} LOGICAL CORES</span>
        <span class="text-[#bd00ff] font-bold">LIVE</span>
        <span>{dynamicSystemState.refreshRate || 1}s SAMPLE</span>
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
        <SystemIcon name="filter" size={16} class="text-white/40" />
      </div>

      <div class="space-y-3">
        {#each topProcesses as task, index (task.id)}
          <div
            class="flex justify-between items-center p-3 bg-white/2 rounded-xl border border-white/5"
          >
            <div class="flex items-center gap-3">
              <span style={`color: ${taskColors[index]}`}>
                <SystemIcon name="memory" size={16} />
              </span>
              <div>
                <p class="text-xs font-bold text-white/90">{task.name}</p>
                <p
                  class="text-[9px] font-mono text-white/40 uppercase tracking-tighter"
                >
                  PID: {task.id}
                </p>
              </div>
            </div>
            <div class="text-right">
              <p
                class="text-xs font-mono font-bold"
                style={`color: ${taskColors[index]}`}
              >
                {task.cpuUsage.toFixed(1)}%
              </p>
            </div>
          </div>
        {:else}
          <p class="py-8 text-center text-xs text-white/40">
            Waiting for process telemetry…
          </p>
        {/each}
      </div>

      <a
        href="/processes"
        data-sveltekit-preload-data="hover"
        class="mt-4 block w-full rounded-xl py-3 text-center text-[10px] font-bold uppercase tracking-widest text-[#bd00ff] transition-colors hover:bg-[#bd00ff]/5"
      >
        VIEW_ALL_PROCESSES
      </a>
    </section>

    <!-- Micro Stats -->
    <div class="grid grid-cols-2 gap-4">
      <div
        class="bg-[#1a1c20] border border-white/5 rounded-xl p-4 flex items-center gap-3"
      >
        <div
          class="w-10 h-10 bg-white/5 rounded-lg flex items-center justify-center"
        >
          <SystemIcon name="wifi" size={24} class="text-[#00d1ff]" />
        </div>
        <div>
          <p class="text-[8px] text-white/40 uppercase tracking-widest">
            NETWORK
          </p>
          <p class="text-xs font-bold">{networkThroughput.toFixed(2)} Mb/s</p>
        </div>
      </div>
      <div
        class="bg-[#1a1c20] border border-white/5 rounded-xl p-4 flex items-center gap-3"
      >
        <div
          class="w-10 h-10 bg-white/5 rounded-lg flex items-center justify-center"
        >
          <SystemIcon name="thermostat" size={24} class="text-[#00d1ff]" />
        </div>
        <div>
          <p class="text-[8px] text-white/40 uppercase tracking-widest">
            NODE_TEMP
          </p>
          <p class="text-xs font-bold">{averageTemperature.toFixed(1)} °C</p>
        </div>
      </div>
    </div>
  </main>
</div>
