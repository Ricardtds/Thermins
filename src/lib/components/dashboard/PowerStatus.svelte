<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import {
    dynamicSystemState,
    staticSystemState,
  } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";

  function getChargePercentage(energy: number, fullCapacity: number) {
    if (!Number.isFinite(energy) || !Number.isFinite(fullCapacity)) return 0;
    if (fullCapacity <= 0) return 0;

    return Math.min(100, Math.max(0, (energy * 100) / fullCapacity));
  }

  function formatPower(value: number) {
    return Number.isFinite(value) ? `${value.toFixed(1)} W` : "--";
  }
</script>

<WidgetCard title="Power Cells" class="h-full min-w-0">
  {#if dynamicSystemState.batteries.length === 0}
    <div
      class="flex min-h-44 items-center justify-center rounded-2xl border border-dashed border-zinc-800 bg-[#0b1018] p-5 text-center"
      role="status"
    >
      <div class="max-w-xs">
        <p class="text-sm font-semibold text-zinc-300">No battery detected</p>
        <p class="mt-2 text-xs leading-relaxed text-zinc-500">
          Power-cell telemetry will appear here when the device exposes a
          battery.
        </p>
      </div>
    </div>
  {:else}
    <div class="grid min-w-0 gap-6">
      {#each dynamicSystemState.batteries as battery}
        {@const staticBattery = staticSystemState.batteries.find(
          (item) => item.id === battery.id,
        )}
        {#if staticBattery}
          {@const chargePercentage = getChargePercentage(
            battery.energy,
            staticBattery.energyFull,
          )}
          <article class="flex min-w-0 flex-col gap-5 sm:gap-6">
            <div
              class="relative mx-auto aspect-square w-36 shrink-0 rounded-full border border-zinc-800 bg-[#0b1020] shadow-[inset_0_0_0_1px_rgba(148,163,184,0.1)] sm:w-44"
              role="progressbar"
              aria-label={`Charge level for ${staticBattery.model ?? battery.id}`}
              aria-valuemin="0"
              aria-valuemax="100"
              aria-valuenow={chargePercentage}
              aria-valuetext={`${chargePercentage.toFixed(1)} percent, ${battery.state}`}
            >
              <div
                class="absolute inset-2 rounded-full bg-[#090b14]"
                style={`background: conic-gradient(#38bdf8 0deg ${chargePercentage * 3.6}deg, rgba(30, 41, 59, 0.2) ${chargePercentage * 3.6}deg 360deg);`}
                aria-hidden="true"
              ></div>
              <div
                class="absolute inset-7 flex flex-col items-center justify-center rounded-full bg-[#07090f] text-center sm:inset-8"
                aria-hidden="true"
              >
                <span
                  class="text-2xl font-black tabular-nums text-cyan-300 sm:text-3xl"
                  >{chargePercentage.toFixed(1)}%</span
                >
                <span
                  class="max-w-full truncate px-2 text-[9px] uppercase tracking-[0.16em] text-zinc-500 sm:text-[10px] sm:tracking-[0.22em]"
                  >{battery.state}</span
                >
                <span class="text-xs tabular-nums text-white sm:text-sm"
                  >{battery.voltage.toFixed(2)} V</span
                >
              </div>
            </div>

            <div class="grid min-w-0 gap-3">
              <div
                class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0b1018] p-4"
              >
                <p
                  class="break-words text-[10px] uppercase leading-relaxed tracking-[0.18em] text-zinc-500 sm:text-xs sm:tracking-[0.22em]"
                >
                  {staticBattery.model ?? "System battery"}
                  <span class="text-zinc-600">·</span>
                  {staticBattery.technology}
                  {#if staticBattery.vendor}
                    <span class="text-zinc-600">·</span>
                    {staticBattery.vendor}
                  {/if}
                </p>
                <p
                  class="mt-3 break-words text-center text-xl font-black uppercase text-white sm:text-2xl"
                >
                  {#if battery.timeToFull !== null && battery.timeToFull > 0}
                    {formatUptime(battery.timeToFull)}
                  {:else if battery.timeToEmpty !== null && battery.timeToEmpty > 0}
                    {formatUptime(battery.timeToEmpty)}
                  {:else}
                    No estimate
                  {/if}
                </p>
                <p class="mt-1 text-center text-xs text-zinc-500 sm:text-sm">
                  Remaining time estimate
                </p>
              </div>

              <dl class="grid min-w-0 gap-2 sm:grid-cols-2">
                <div
                  class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0b1018] p-4"
                >
                  <dt
                    class="text-[10px] uppercase tracking-[0.22em] text-zinc-500 sm:tracking-[0.3em]"
                  >
                    Voltage
                  </dt>
                  <dd
                    class="mt-3 break-words text-lg font-bold tabular-nums text-white"
                  >
                    {battery.voltage.toFixed(2)} V
                  </dd>
                </div>
                <div
                  class="min-w-0 rounded-2xl border border-zinc-800 bg-[#0b1018] p-4"
                >
                  <dt
                    class="text-[10px] uppercase tracking-[0.22em] text-zinc-500 sm:tracking-[0.3em]"
                  >
                    Consumption
                  </dt>
                  <dd
                    class="mt-3 break-words text-lg font-bold tabular-nums text-fuchsia-400"
                  >
                    {formatPower(battery.energyRate)}
                  </dd>
                </div>
              </dl>
            </div>
          </article>
        {:else}
          <div
            class="rounded-2xl border border-dashed border-zinc-800 bg-[#0b1018] p-5 text-center text-xs leading-relaxed text-zinc-500"
            role="status"
          >
            Waiting for metadata from battery {battery.id}.
          </div>
        {/if}
      {/each}
    </div>
  {/if}
</WidgetCard>
