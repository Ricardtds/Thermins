<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";

  $: lastReset = formatUptime(dynamicSystemState.uptime);
  $: averageTemperature = dynamicSystemState.components.length
    ? dynamicSystemState.components.reduce(
        (sum, sensor) => sum + sensor.temperature,
        0,
      ) / dynamicSystemState.components.length
    : 0;
</script>

<WidgetCard title="Thermal Analysis Matrix" class="h-full min-w-0">
  <div class="min-w-0 space-y-5 sm:space-y-6">
    <div
      class="flex min-w-0 flex-col gap-4 xl:flex-row xl:items-end xl:justify-between"
    >
      <div class="min-w-0 space-y-3">
        <p
          class="text-[10px] uppercase tracking-[0.2em] text-cyan-400/80 sm:text-xs sm:tracking-[0.28em]"
        >
          Telemetry System
        </p>
        <h3
          class="break-words text-2xl font-black tracking-tight text-white sm:text-3xl"
        >
          CORE MONITOR V1.0
        </h3>
        <p class="max-w-prose text-sm leading-relaxed text-zinc-500">
          Thermal analysis grid com status em tempo real e monitoramento dos
          sensores.
        </p>
      </div>

      <div class="flex min-w-0 flex-wrap gap-2">
        <span
          class="rounded-full border border-cyan-600/40 bg-cyan-500/10 px-3 py-1 text-[10px] uppercase tracking-[0.18em] text-cyan-300 sm:tracking-[0.25em]"
        >
          Real-time stream
        </span>
        <span
          class="max-w-full break-words rounded-full border border-zinc-800 bg-zinc-950 px-3 py-1 text-[10px] uppercase tracking-[0.18em] text-zinc-400 sm:tracking-[0.25em]"
        >
          Last reset: {lastReset}
        </span>
      </div>
    </div>

    <div class="grid min-w-0 gap-3 sm:grid-cols-2 sm:gap-4">
      <div
        class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0c1018] p-4 sm:rounded-3xl sm:p-5"
        aria-label={dynamicSystemState.components.length
          ? `Average sensor temperature: ${averageTemperature.toFixed(0)} degrees Celsius`
          : "Average sensor temperature unavailable"}
      >
        <p
          class="text-[10px] uppercase tracking-[0.2em] text-zinc-500 sm:text-xs sm:tracking-[0.25em]"
        >
          CPU Thermal Grid
        </p>
        <div class="mt-4 flex min-w-0 items-baseline gap-2 sm:mt-5 sm:gap-3">
          <span
            class="min-w-0 text-4xl font-black tabular-nums text-fuchsia-400 sm:text-5xl"
            aria-hidden="true"
          >
            {dynamicSystemState.components.length
              ? averageTemperature.toFixed(0)
              : "--"}
          </span>
          <span
            class="shrink-0 text-xl font-semibold text-zinc-500 sm:text-2xl"
            aria-hidden="true">°C</span
          >
        </div>
        <p class="mt-3 text-sm leading-relaxed text-zinc-500">
          Temperatura média dos núcleos e distribuição do calor.
        </p>
      </div>

      <div
        class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0c1018] p-4 sm:rounded-3xl sm:p-5"
      >
        <p
          class="text-[10px] uppercase tracking-[0.2em] text-zinc-500 sm:text-xs sm:tracking-[0.25em]"
        >
          Active Sensors
        </p>
        <p
          class="mt-4 text-4xl font-black tabular-nums text-cyan-300 sm:text-5xl"
        >
          {dynamicSystemState.components.length}
        </p>
        <p class="mt-2 text-sm leading-relaxed text-zinc-500">
          Sensores ativos detectados no sistema.
        </p>
      </div>
    </div>
  </div>
</WidgetCard>
