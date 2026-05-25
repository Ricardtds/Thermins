<script lang="ts">
  import { systemState } from "$lib/stores/system.svelte";

  const recentEvents = [
    {
      title: "Kernel event detected",
      time: "2 minutes ago",
    },
    {
      title: "CPU temperature stabilized",
      time: "5 minutes ago",
    },
    {
      title: "Storage scan completed",
      time: "12 minutes ago",
    },
  ];

  const hardwareStatus = [
    {
      label: "CPU",
      status: "Operational",
    },
    {
      label: "GPU",
      status: "Operational",
    },
    {
      label: "Storage",
      status: "Healthy",
    },
    {
      label: "Network",
      status: "Stable",
    },
  ];

  const ramUsedGb = $derived.by(() => {
    if (!systemState) return 0;

    return systemState.memory.used / 1024 / 1024 / 1024;
  });

  const ramTotalGb = $derived.by(() => {
    if (!systemState) return 0;

    return systemState.memory.total / 1024 / 1024 / 1024;
  });
</script>

{#if systemState}
  <section class="space-y-6">
    <!-- HEADER -->
    <div>
      <h1 class="text-4xl font-bold tracking-tight">Dashboard</h1>

      <p class="mt-2 text-zinc-400">Real-time system telemetry overview</p>
    </div>

    <!-- METRIC CARDS -->
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
      <!-- CPU -->
      <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
        <div class="mb-5">
          <h2 class="text-xl font-semibold">CPU Information</h2>

          <p class="text-sm text-zinc-500">Processor details</p>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <p class="text-sm text-zinc-500">Brand</p>

            <p class="mt-1 font-medium">
              {systemState.cpu.info.brand}
            </p>
          </div>

          <div>
            <p class="text-sm text-zinc-500">Vendor</p>

            <p class="mt-1 font-medium">
              {systemState.cpu.info.vendor_id}
            </p>
          </div>

          <div>
            <p class="text-sm text-zinc-500">Physical Cores</p>

            <p class="mt-1 font-medium">
              {systemState.cpu.info.physical_cores}
            </p>
          </div>

          <div>
            <p class="text-sm text-zinc-500">Logical Threads</p>

            <p class="mt-1 font-medium">
              {systemState.cpu.cores.length}
            </p>
          </div>
        </div>
      </div>
      <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
        <p class="text-sm text-zinc-400">CPU Load</p>

        <h2 class="mt-3 text-4xl font-bold">
          {systemState.cpu.usage.toFixed(2)}%
        </h2>

        <div class="mt-4 h-2 overflow-hidden rounded-full bg-zinc-800">
          <div
            class="h-full rounded-full bg-cyan-400 transition-all duration-300"
            style={`width: ${systemState.cpu.usage}%`}
          ></div>
        </div>
      </div>

      <!-- RAM -->
      <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
        <p class="text-sm text-zinc-400">RAM Usage</p>

        <h2 class="mt-3 text-4xl font-bold">
          {ramUsedGb.toFixed(1)} GB
        </h2>

        <p class="mt-1 text-sm text-zinc-500">
          / {ramTotalGb.toFixed(1)} GB
        </p>

        <div class="mt-4 h-2 overflow-hidden rounded-full bg-zinc-800">
          <div
            class="h-full rounded-full bg-emerald-400 transition-all duration-300"
            style={`width: ${systemState.memory.usage_percent}%`}
          />
        </div>
      </div>

      <!-- STORAGE -->
      <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
        <p class="text-sm text-zinc-400">Storage Usage</p>

        <h2 class="mt-3 text-4xl font-bold">72%</h2>

        <div class="mt-4 h-2 overflow-hidden rounded-full bg-zinc-800">
          <div class="h-full w-[72%] rounded-full bg-orange-400" />
        </div>
      </div>

      <!-- NETWORK -->
      <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
        <p class="text-sm text-zinc-400">Network Traffic</p>

        <h2 class="mt-3 text-4xl font-bold">2.4 MB/s</h2>

        <div class="mt-4 flex items-center gap-2 text-sm text-zinc-500">
          <span>↓ 1.8 MB/s</span>
          <span>↑ 0.6 MB/s</span>
        </div>
      </div>
    </div>

    <!-- MAIN GRID -->
    <div class="grid grid-cols-1 gap-4 xl:grid-cols-3">
      <!-- CPU CHART -->
      <div
        class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5 xl:col-span-2"
      >
        <div class="mb-5 flex items-center justify-between">
          <div>
            <h2 class="text-xl font-semibold">CPU Activity</h2>

            <p class="text-sm text-zinc-500">Real-time usage monitoring</p>
          </div>

          <div
            class="rounded-full border border-cyan-500/30 bg-cyan-500/10 px-3 py-1 text-sm text-cyan-300"
          >
            Live
          </div>
        </div>

        <!-- CORES -->
        <div class="grid grid-cols-2 gap-4 xl:grid-cols-4">
          {#each systemState.cpu.cores as core}
            <div class="rounded-xl bg-zinc-800 p-4">
              <div class="flex items-center justify-between">
                <p class="text-sm text-zinc-400">
                  {core.name}
                </p>

                <p class="text-xs text-zinc-500">
                  {core.frequency} MHz
                </p>
              </div>

              <h3 class="mt-3 text-2xl font-bold">
                {core.usage.toFixed(1)}%
              </h3>

              <div class="mt-4 h-2 overflow-hidden rounded-full bg-zinc-700">
                <div
                  class="h-full rounded-full bg-cyan-400 transition-all duration-300"
                  style={`width: ${core.usage}%`}
                />
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- EVENTS -->
      <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
        <div class="mb-5">
          <h2 class="text-xl font-semibold">Recent Events</h2>

          <p class="text-sm text-zinc-500">Latest system activity</p>
        </div>

        <div class="space-y-3">
          {#each recentEvents as event}
            <div class="rounded-xl bg-zinc-800 p-4">
              <p class="font-medium">
                {event.title}
              </p>

              <p class="mt-1 text-sm text-zinc-500">
                {event.time}
              </p>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- HARDWARE STATUS -->
    <div class="rounded-2xl border border-zinc-800 bg-zinc-900 p-5">
      <div class="mb-5">
        <h2 class="text-xl font-semibold">Hardware Status</h2>

        <p class="text-sm text-zinc-500">Current subsystem state</p>
      </div>

      <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
        {#each hardwareStatus as item}
          <div class="rounded-xl bg-zinc-800 p-4">
            <div class="flex items-center justify-between">
              <p class="text-sm text-zinc-400">
                {item.label}
              </p>

              <div class="h-3 w-3 rounded-full bg-emerald-400" />
            </div>

            <p class="mt-4 text-lg font-semibold">
              {item.status}
            </p>
          </div>
        {/each}
      </div>
    </div>
  </section>
{/if}
oi
