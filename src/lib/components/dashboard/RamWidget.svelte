<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";

  import { systemState } from "$lib/stores/system.svelte";

  const usedGb = $derived.by(() => {
    return systemState.memory.used / 1024 / 1024 / 1024;
  });

  const totalGb = $derived.by(() => {
    return systemState.memory.total / 1024 / 1024 / 1024;
  });

  const memoryBars = $derived.by(() => {
    return Math.floor(systemState.memory.usage_percent / 6.25);
  });

  const usagePercent = $derived.by(() => {
    return systemState.memory.usage_percent.toFixed(1);
  });
</script>

<div class="col-span-12 xl:col-span-4">
  <WidgetCard title="RAM UTILIZATION">
    <!-- HEADER -->
    <div class="flex items-start justify-between">
      <div>
        <p
          class="
            text-xs
            uppercase
            tracking-[0.25em]
            text-zinc-500
          "
        >
          Memory Pool
        </p>

        <h2
          class="
            mt-3
            text-5xl
            font-black
            text-fuchsia-300
          "
        >
          {usagePercent}%
        </h2>
      </div>

      <div class="text-right">
        <p
          class="
            text-xs
            uppercase
            tracking-[0.25em]
            text-zinc-500
          "
        >
          Usage
        </p>

        <p
          class="
            mt-3
            text-2xl
            font-bold
            text-zinc-100
          "
        >
          {usedGb.toFixed(2)} GB
        </p>

        <p class="mt-1 text-sm text-zinc-500">
          / {totalGb.toFixed(0)} GB
        </p>
      </div>
    </div>

    <!-- MEMORY GRID -->
    <div class="mt-8">
      <div
        class="
          grid
          grid-cols-8
          gap-2
        "
      >
        {#each Array(32) as _, index}
          <div
            class={`
              h-8
              border
              border-zinc-800
              transition-all
              duration-300

              ${
                index < memoryBars * 2
                  ? "bg-fuchsia-400 shadow-[0_0_12px_rgba(232,121,249,0.5)]"
                  : "bg-zinc-900"
              }
            `}
          />
        {/each}
      </div>
    </div>

    <!-- MEMORY STATS -->
    <div class="mt-8 grid grid-cols-2 gap-4">
      <div
        class="
          border
          border-zinc-800
          bg-black/30
          p-4
        "
      >
        <p
          class="
            text-xs
            uppercase
            tracking-[0.25em]
            text-zinc-500
          "
        >
          Used
        </p>

        <p
          class="
            mt-4
            text-4xl
            font-bold
            text-zinc-100
          "
        >
          {usedGb.toFixed(2)}
        </p>

        <p class="mt-1 text-sm text-zinc-500">Gigabytes</p>
      </div>

      <div
        class="
          border
          border-zinc-800
          bg-black/30
          p-4
        "
      >
        <p
          class="
            text-xs
            uppercase
            tracking-[0.25em]
            text-zinc-500
          "
        >
          Available
        </p>

        <p
          class="
            mt-4
            text-4xl
            font-bold
            text-cyan-300
          "
        >
          {(totalGb - usedGb).toFixed(1)}
        </p>

        <p class="mt-1 text-sm text-zinc-500">Gigabytes</p>
      </div>
    </div>

    <!-- FOOTER -->
    <div
      class="
        mt-8
        flex
        items-center
        justify-between
        border-t
        border-zinc-800
        pt-4
        text-xs
        uppercase
        tracking-[0.25em]
        text-zinc-500
      "
    >
      <span> Real-Time Memory Telemetry </span>

      <span> ACTIVE </span>
    </div>
  </WidgetCard>
</div>
