<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import {
    dynamicSystemState,
    staticSystemState,
  } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";
</script>

<WidgetCard title="Power Cells" class="p-6">
  {#each dynamicSystemState.batteries as battery}
    {@const staticBattery = staticSystemState.batteries.find(
      (b) => b.id === battery.id,
    )}
    {#if staticBattery}
      <div class="flex flex-col gap-6">
        <div
          class="mx-auto flex h-44 w-44 items-center justify-center rounded-full bg-[#0b1020] border border-zinc-800 shadow-[inset_0_0_0_1px_rgba(148,163,184,0.1)]"
        >
          <div
            class="relative flex h-40 w-40 items-center justify-center rounded-full bg-[#090b14]"
          >
            <div
              class="absolute inset-0 rounded-full"
              style="background: conic-gradient(#38bdf8 0deg, #38bdf8 calc(360deg * {battery.energy /
                staticBattery.energyFull}), rgba(30,41,59,0.2) 0deg);"
            ></div>
            <div
              class="absolute inset-5 rounded-full bg-[#07090f] flex flex-col items-center justify-center text-center"
            >
              <span class="text-4xl font-black text-cyan-300"
                >{((battery.energy * 100) / staticBattery.energyFull).toFixed(
                  1,
                )}%</span
              >
              <span class="text-xs uppercase tracking-[0.3em] text-zinc-500"
                >{battery.state}</span
              >
              <span class="text-sm text-white"
                >{battery.voltage.toFixed(2)} V</span
              >
            </div>
          </div>
        </div>

        <div class="grid gap-3">
          <div class="rounded-2xl border border-zinc-800 bg-[#0b1018] p-4">
            <p class="text-xs uppercase tracking-[0.25em] text-zinc-500">
              {staticBattery.model}
              ({staticBattery.technology})
              {staticBattery.vendor}
            </p>
            <p
              class="mt-2 text-2xl font-black text-white uppercase text-center"
            >
              {#if battery.timeToFull}
                {formatUptime(battery.timeToFull)}
              {:else if battery.timeToEmpty}
                {formatUptime(battery.timeToEmpty)}
              {:else}
                ?
              {/if}
            </p>
            <p class="mt-1 text-sm text-zinc-500">Health status nominal</p>
          </div>

          <div class="grid gap-2 sm:grid-cols-2">
            <div class="rounded-2xl border border-zinc-800 bg-[#0b1018] p-4">
              <p class="text-[10px] uppercase tracking-[0.3em] text-zinc-500">
                Voltage
              </p>
              <p class="mt-3 w-100 text-lg font-bold text-white">
                {battery.voltage.toFixed(2)} V
              </p>
            </div>
            <div class="rounded-2xl border border-zinc-800 bg-[#0b1018] p-4">
              <p class="text-[10px] uppercase tracking-[0.3em] text-zinc-500">
                Consumption
              </p>
              <p class="mt-3 text-lg font-bold text-fuchsia-400">
                {battery.energyRate}W
              </p>
            </div>
          </div>
        </div>
      </div>
    {/if}
  {/each}
</WidgetCard>
