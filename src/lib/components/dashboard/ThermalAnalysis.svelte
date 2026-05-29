<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";
  const lastReset = formatUptime(dynamicSystemState.uptime);

  $: averageTemperature = dynamicSystemState.components.length
    ? dynamicSystemState.components.reduce(
        (sum, sensor) => sum + sensor.temperature,
        0,
      ) / dynamicSystemState.components.length
    : 0;
</script>

<WidgetCard title="Thermal Analysis Matrix" class="p-6">
  <div class="space-y-6">
    <div
      class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"
    >
      <div class="space-y-3">
        <p class="text-xs uppercase tracking-[0.28em] text-cyan-400/80">
          Telemetry System
        </p>
        <h1 class="text-3xl font-black tracking-tight text-white">
          CORE MONITOR V1.0
        </h1>
        <p class="text-sm text-zinc-500">
          Thermal analysis grid com status em tempo real e monitoramento dos
          sensores.
        </p>
      </div>

      <div class="flex flex-wrap gap-2">
        <span
          class="rounded-full border border-cyan-600/40 bg-cyan-500/10 px-3 py-1 text-[10px] uppercase tracking-[0.3em] text-cyan-300"
        >
          Real-time stream
        </span>
        <span
          class="rounded-full border border-zinc-800 bg-zinc-950 px-3 py-1 text-[10px] uppercase tracking-[0.3em] text-zinc-400"
        >
          Last reset: {lastReset}
        </span>
      </div>
    </div>

    <div class="grid gap-4 sm:grid-cols-2">
      <div class="rounded-3xl border border-zinc-800 bg-[#0c1018] p-5">
        <p class="text-xs uppercase tracking-[0.3em] text-zinc-500">
          CPU Thermal Grid
        </p>
        <div class="mt-5 flex items-baseline gap-3">
          <h2 class="text-5xl font-black text-fuchsia-400">
            {averageTemperature.toFixed(0)}
          </h2>
          <span class="text-2xl font-semibold text-zinc-500">°C</span>
        </div>
        <p class="mt-3 text-sm text-zinc-500">
          Temperatura média dos núcleos e distribuição do calor.
        </p>
      </div>

      <div class="rounded-3xl border border-zinc-800 bg-[#0c1018] p-5">
        <p class="text-xs uppercase tracking-[0.3em] text-zinc-500">
          Active Sensors
        </p>
        <p class="mt-4 text-5xl font-black text-cyan-300">
          {dynamicSystemState.components.length}
        </p>
        <p class="mt-2 text-sm text-zinc-500">
          Sensores ativos detectados no sistema.
        </p>
      </div>
    </div>
  </div>
</WidgetCard>
