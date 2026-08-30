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

  function getTemperatureStatus(temp: number) {
    if (temp >= 85) return "Critical";
    if (temp >= 70) return "High";
    if (temp >= 55) return "Elevated";
    return "Normal";
  }

  function getScaleMaximum(
    temperature: number,
    maxTemperature: number,
    critical: number,
  ) {
    return Math.max(100, temperature, maxTemperature, critical);
  }

  function formatOptionalTemperature(value: number) {
    return value > 0 ? `${value.toFixed(0)}°` : "--";
  }
</script>

<WidgetCard title="Core Thermal Grid" class="min-w-0">
  {#if dynamicSystemState.components.length === 0}
    <div
      class="flex min-h-44 items-center justify-center rounded-2xl border border-dashed border-zinc-800 bg-[#0b1018] p-5 text-center"
      role="status"
    >
      <div class="max-w-sm">
        <p class="text-sm font-semibold text-zinc-300">
          No thermal sensors available
        </p>
        <p class="mt-2 text-xs leading-relaxed text-zinc-500">
          The grid will populate automatically when the operating system
          reports component temperatures.
        </p>
      </div>
    </div>
  {:else}
    <ul
      class="grid min-w-0 gap-3 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
      aria-label="Thermal sensor grid"
    >
      {#each dynamicSystemState.components as sensor}
        {@const scaleMaximum = getScaleMaximum(
          sensor.temperature,
          sensor.maxTemperature,
          sensor.critical,
        )}
        <li
          class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0b1018] p-3 transition-colors hover:border-cyan-400/40 sm:rounded-3xl sm:p-4"
          aria-label={`${sensor.label || "Unnamed sensor"}: ${sensor.temperature.toFixed(0)} degrees Celsius, ${getTemperatureStatus(sensor.temperature)}`}
        >
          <div class="flex min-w-0 items-start justify-between gap-3">
            <div class="min-w-0">
              <p
                class="break-words text-[10px] uppercase leading-relaxed tracking-[0.18em] text-zinc-500 sm:tracking-[0.25em]"
              >
                {sensor.label || "Unnamed sensor"}
              </p>
              <h3 class="mt-2 break-all text-sm font-bold text-white sm:text-base">
                {sensor.id || "ID unavailable"}
              </h3>
            </div>
            <div
              class={`shrink-0 text-2xl font-black tabular-nums sm:text-3xl ${getTemperatureColor(sensor.temperature)}`}
              aria-hidden="true"
            >
              {sensor.temperature.toFixed(0)}°C
            </div>
          </div>

          <div class="mt-4 sm:mt-5">
            <div
              class="flex min-w-0 items-center justify-between gap-3 text-[10px] uppercase tracking-[0.16em] text-zinc-500 sm:tracking-[0.22em]"
            >
              <span>{getTemperatureStatus(sensor.temperature)}</span>
              <span class="shrink-0 tabular-nums">
                Scale {scaleMaximum.toFixed(0)}°C
              </span>
            </div>
            <div
              class="mt-3 h-2 overflow-hidden rounded-full border border-zinc-800 bg-zinc-950"
              role="progressbar"
              aria-label={`Current temperature for ${sensor.label || "sensor"}`}
              aria-valuemin="0"
              aria-valuemax={scaleMaximum}
              aria-valuenow={Math.max(0, sensor.temperature)}
              aria-valuetext={`${sensor.temperature.toFixed(0)} degrees Celsius`}
            >
              <div
                class={`h-full transition-[width] duration-300 motion-reduce:transition-none ${getBarColor(sensor.temperature)}`}
                style={`width: ${Math.min(100, Math.max(0, (sensor.temperature * 100) / scaleMaximum))}%`}
              ></div>
            </div>
          </div>

          <dl
            class="mt-4 grid grid-cols-3 gap-2 text-center text-[9px] uppercase tracking-[0.12em] text-zinc-500 sm:mt-5 sm:text-[10px] sm:tracking-[0.18em]"
          >
            <div
              class="min-w-0 rounded-xl border border-zinc-800 bg-[#090b12] p-2 sm:rounded-2xl"
            >
              <dt>Curr</dt>
              <dd
                class="mt-1 text-sm font-bold tabular-nums text-white normal-case"
              >
                {sensor.temperature.toFixed(0)}°
              </dd>
            </div>
            <div
              class="min-w-0 rounded-xl border border-zinc-800 bg-[#090b12] p-2 sm:rounded-2xl"
            >
              <dt>Max</dt>
              <dd
                class="mt-1 text-sm font-bold tabular-nums text-orange-400 normal-case"
              >
                {formatOptionalTemperature(sensor.maxTemperature)}
              </dd>
            </div>
            <div
              class="min-w-0 rounded-xl border border-zinc-800 bg-[#090b12] p-2 sm:rounded-2xl"
            >
              <dt>Crit</dt>
              <dd
                class="mt-1 text-sm font-bold tabular-nums text-red-400 normal-case"
              >
                {formatOptionalTemperature(sensor.critical)}
              </dd>
            </div>
          </dl>
        </li>
      {/each}
    </ul>
  {/if}
</WidgetCard>
