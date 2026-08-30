<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";

  function getTemperatureStatus(temperature: number) {
    if (temperature >= 85) return "Critical";
    if (temperature >= 70) return "High";
    if (temperature >= 55) return "Elevated";
    return "Normal";
  }

  function getStatusClass(temperature: number) {
    if (temperature >= 85) return "border-red-500/40 bg-red-500/10 text-red-300";
    if (temperature >= 70)
      return "border-orange-500/40 bg-orange-500/10 text-orange-300";
    if (temperature >= 55)
      return "border-yellow-500/40 bg-yellow-500/10 text-yellow-200";
    return "border-cyan-500/40 bg-cyan-500/10 text-cyan-300";
  }

  function formatOptionalTemperature(value: number) {
    return value > 0 ? `${value.toFixed(0)}°C` : "--";
  }
</script>

<WidgetCard title="Sensor Log" class="h-full min-w-0">
  <div class="min-w-0 space-y-4">
    <div
      class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0b1018] p-3 sm:rounded-3xl sm:p-4"
    >
      <div
        class="flex min-w-0 flex-col gap-2 text-[10px] uppercase tracking-[0.18em] text-zinc-500 sm:flex-row sm:items-center sm:justify-between sm:text-xs sm:tracking-[0.22em]"
      >
        <span>
          Sampling:
          {dynamicSystemState.refreshRate > 0
            ? `Every ${dynamicSystemState.refreshRate}s`
            : "Waiting"}
        </span>
        <span>Active Sensors: {dynamicSystemState.components.length}</span>
      </div>

      {#if dynamicSystemState.components.length === 0}
        <div
          class="mt-4 flex min-h-36 items-center justify-center rounded-2xl border border-dashed border-zinc-800 p-5 text-center"
          role="status"
        >
          <div class="max-w-xs">
            <p class="text-sm font-semibold text-zinc-300">
              Waiting for sensor data
            </p>
            <p class="mt-2 text-xs leading-relaxed text-zinc-500">
              Temperature readings will be listed after telemetry connects.
            </p>
          </div>
        </div>
      {:else}
        <ul class="mt-4 grid gap-3 sm:hidden" aria-label="Sensor readings">
          {#each dynamicSystemState.components as component}
            <li class="min-w-0 rounded-2xl border border-zinc-800 bg-[#090d14] p-3">
              <div class="flex min-w-0 items-start justify-between gap-3">
                <div class="min-w-0">
                  <p class="break-words text-sm font-semibold text-white">
                    {component.label || "Unnamed sensor"}
                  </p>
                  <p
                    class="mt-1 break-all text-[10px] uppercase tracking-[0.16em] text-zinc-600"
                  >
                    {component.id || "ID unavailable"}
                  </p>
                </div>
                <span
                  class={`shrink-0 rounded-full border px-2 py-1 text-[9px] uppercase tracking-[0.12em] ${getStatusClass(component.temperature)}`}
                >
                  {getTemperatureStatus(component.temperature)}
                </span>
              </div>

              <dl class="mt-3 grid grid-cols-3 gap-2 text-center">
                <div class="min-w-0 rounded-xl bg-black/30 p-2">
                  <dt
                    class="text-[9px] uppercase tracking-[0.12em] text-zinc-500"
                  >
                    Current
                  </dt>
                  <dd
                    class="mt-1 text-sm font-bold tabular-nums text-cyan-300"
                  >
                    {component.temperature.toFixed(0)}°C
                  </dd>
                </div>
                <div class="min-w-0 rounded-xl bg-black/30 p-2">
                  <dt
                    class="text-[9px] uppercase tracking-[0.12em] text-zinc-500"
                  >
                    Peak
                  </dt>
                  <dd
                    class="mt-1 text-sm font-bold tabular-nums text-orange-300"
                  >
                    {formatOptionalTemperature(component.maxTemperature)}
                  </dd>
                </div>
                <div class="min-w-0 rounded-xl bg-black/30 p-2">
                  <dt
                    class="text-[9px] uppercase tracking-[0.12em] text-zinc-500"
                  >
                    Critical
                  </dt>
                  <dd
                    class="mt-1 text-sm font-bold tabular-nums text-red-300"
                  >
                    {formatOptionalTemperature(component.critical)}
                  </dd>
                </div>
              </dl>
            </li>
          {/each}
        </ul>

        <!-- svelte-ignore a11y_no_noninteractive_tabindex (overflow region must be keyboard-scrollable) -->
        <div
          class="mt-4 hidden max-w-full overflow-x-auto overscroll-x-contain rounded-xl border border-zinc-800 sm:block"
          role="region"
          aria-label="Scrollable sensor readings"
          tabindex="0"
        >
          <table class="w-full min-w-[28rem] text-left text-xs text-zinc-300">
            <caption class="sr-only">
              Current, peak, and status values for every detected temperature
              sensor.
            </caption>
            <thead
              class="border-b border-zinc-800 bg-[#0d121b] text-[10px] uppercase tracking-[0.16em] text-zinc-500"
            >
              <tr>
                <th class="px-3 py-3" scope="col">Module</th>
                <th class="px-3 py-3 text-right" scope="col">Current</th>
                <th class="px-3 py-3 text-right" scope="col">Peak</th>
                <th class="px-3 py-3 text-right" scope="col">Status</th>
              </tr>
            </thead>
            <tbody>
              {#each dynamicSystemState.components as component}
                <tr
                  class="border-b border-zinc-900 transition-colors last:border-b-0 hover:bg-cyan-500/5"
                >
                  <th class="min-w-0 px-3 py-3 font-normal" scope="row">
                    <span
                      class="block max-w-48 truncate text-sm text-white"
                      title={component.label}
                    >
                      {component.label || "Unnamed sensor"}
                    </span>
                    <span
                      class="mt-1 block max-w-48 truncate text-[10px] uppercase tracking-[0.12em] text-zinc-600"
                      title={component.id}
                    >
                      {component.id || "ID unavailable"}
                    </span>
                  </th>
                  <td
                    class="px-3 py-3 text-right font-semibold tabular-nums text-cyan-300"
                  >
                    {component.temperature.toFixed(0)}°C
                  </td>
                  <td
                    class="px-3 py-3 text-right tabular-nums text-orange-300"
                  >
                  {formatOptionalTemperature(component.maxTemperature)}
                  </td>
                  <td class="px-3 py-3 text-right">
                    <span
                      class={`inline-flex rounded-full border px-2 py-1 text-[9px] uppercase tracking-[0.1em] ${getStatusClass(component.temperature)}`}
                    >
                      {getTemperatureStatus(component.temperature)}
                    </span>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
</WidgetCard>
