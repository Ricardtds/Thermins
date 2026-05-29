<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";

  function getTemperatureColor(temp: number) {
    if (temp >= 85) return "text-red-400";
    if (temp >= 70) return "text-orange-400";
    if (temp >= 55) return "text-yellow-300";
    return "text-cyan-300";
  }

  function getBarColor(temp: number) {
    if (temp >= 85) return "bg-red-500";
    if (temp >= 70) return "bg-orange-400";
    if (temp >= 55) return "bg-yellow-300";
    return "bg-cyan-400";
  }
</script>

<WidgetCard title="Core Thermal Grid" class="p-6">
  <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
    {#each dynamicSystemState.components as sensor}
      <div
        class="rounded-3xl border border-zinc-800 bg-[#0b1018] p-4 transition hover:border-cyan-400/40"
      >
        <div class="flex items-start justify-between gap-3">
          <div>
            <p class="text-[10px] uppercase tracking-[0.3em] text-zinc-500">
              {sensor.label}
            </p>
            <h3 class="mt-2 text-lg font-bold text-white">{sensor.id}</h3>
          </div>
          <div
            class={`text-3xl font-black ${getTemperatureColor(sensor.temperature)}`}
          >
            {sensor.temperature.toFixed(0)}°
          </div>
        </div>

        <div class="mt-5">
          <div
            class="flex items-center justify-between text-[10px] uppercase tracking-[0.25em] text-zinc-500"
          >
            <span>Thermal</span>
            <span>{sensor.maxTemperature.toFixed(0)}°</span>
          </div>
          <div
            class="mt-3 h-2 overflow-hidden rounded-full bg-zinc-950 border border-zinc-800"
          >
            <div
              class={`h-full ${getBarColor(sensor.temperature)}`}
              style={`width: ${Math.min(sensor.temperature, 100)}%`}
            ></div>
          </div>
        </div>

        <div
          class="mt-5 grid grid-cols-3 gap-2 text-center text-[10px] uppercase tracking-[0.25em] text-zinc-500"
        >
          <div class="rounded-2xl border border-zinc-800 bg-[#090b12] p-2">
            <p>Curr</p>
            <p class="mt-1 text-sm font-bold text-white">
              {sensor.temperature.toFixed(0)}°
            </p>
          </div>
          <div class="rounded-2xl border border-zinc-800 bg-[#090b12] p-2">
            <p>Max</p>
            <p class="mt-1 text-sm font-bold text-orange-400">
              {sensor.maxTemperature.toFixed(0)}°
            </p>
          </div>
          <div class="rounded-2xl border border-zinc-800 bg-[#090b12] p-2">
            <p>Crit</p>
            <p class="mt-1 text-sm font-bold text-red-400">
              {sensor.critical.toFixed(0)}°
            </p>
          </div>
        </div>
      </div>
    {/each}
  </div>
</WidgetCard>
