<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";

  import { cpuHistory } from "$lib/stores/history.svelte";

  import {
    dynamicSystemState,
    staticSystemState,
  } from "$lib/stores/system.svelte";

  const width = 900;
  const height = 260;

  const points = $derived.by(() => {
    if (cpuHistory.length === 0) return "";

    return cpuHistory
      .map((value, index) => {
        const x = (index / Math.max(cpuHistory.length - 1, 1)) * width;

        const y = height - (value / 100) * height;

        return `${x},${y}`;
      })
      .join(" ");
  });
</script>

<div class="col-span-12 xl:col-span-8">
  <WidgetCard title="REAL-TIME CPU LOAD (%)">
    <div class="mb-6 flex items-center justify-between">
      <div>
        <p class="text-xs uppercase tracking-widest text-zinc-500">Processor</p>

        <h2 class="mt-2 text-3xl font-bold text-zinc-100">
          {staticSystemState.cpu.brand || "Processor"}
        </h2>
      </div>

      <div class="text-right">
        <p class="text-xs uppercase tracking-widest text-zinc-500">
          Current Usage
        </p>

        <h2 class="mt-2 text-5xl font-bold text-cyan-300">
          {dynamicSystemState.cpu.usage.toFixed(1)}%
        </h2>
      </div>
    </div>

    <!-- CHART -->
    <div
      class="
        relative
        h-[260px]
        overflow-hidden
        border
        border-zinc-800
        bg-black
      "
    >
      <!-- GRID -->
      <div
        class="
          absolute
          inset-0
          bg-[linear-gradient(to_right,rgba(255,255,255,0.03)_1px,transparent_1px),linear-gradient(to_bottom,rgba(255,255,255,0.03)_1px,transparent_1px)]
          bg-[size:40px_40px]
        "
      ></div>

      <!-- SVG -->
      <svg
        viewBox={`0 0 ${width} ${height}`}
        class="absolute inset-0 h-full w-full"
        preserveAspectRatio="none"
      >
        <!-- AREA -->
        <polygon
          points={`0,${height} ${points} ${width},${height}`}
          fill="rgba(34,211,238,0.12)"
        />

        <!-- LINE -->
        <polyline
          {points}
          fill="none"
          stroke="#22d3ee"
          stroke-width="3"
          stroke-linejoin="round"
          stroke-linecap="round"
        />
      </svg>

      <!-- SCANLINE -->
      <div
        class="
          pointer-events-none
          absolute
          inset-0
          bg-[linear-gradient(to_bottom,transparent_0%,rgba(255,255,255,0.03)_50%,transparent_100%)]
          animate-pulse
        "
      ></div>
    </div>

    <!-- FOOTER -->
    <div
      class="
        mt-6
        flex
        items-center
        justify-between
        text-xs
        uppercase
        tracking-widest
        text-zinc-500
      "
    >
      <span> Refresh Rate: 1s </span>

      <span>
        Threads:
        {dynamicSystemState.cpu.cores.length}
      </span>

      <span>
        Vendor:
        {staticSystemState.cpu.vendorId || "Unknown"}
      </span>
    </div>
  </WidgetCard>
</div>
