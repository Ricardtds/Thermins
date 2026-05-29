<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";
</script>

<WidgetCard title="Sensor Log" class="p-6">
  <div class="space-y-4">
    <div class="rounded-3xl border border-zinc-800 bg-[#0b1018] p-4">
      <div
        class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between text-xs uppercase tracking-[0.3em] text-zinc-500"
      >
        <span>Sampling: 1Hz</span>
        <span>Active Sensors: {dynamicSystemState.components.length}</span>
      </div>

      <div class="mt-4 overflow-x-auto">
        <table class="min-w-full text-left text-sm text-zinc-300">
          <thead
            class="border-b border-zinc-800 text-[10px] uppercase tracking-[0.3em] text-zinc-500"
          >
            <tr>
              <th class="py-3 px-2">Timestamp</th>
              <th class="py-3 px-2">Sensor ID</th>
              <th class="py-3 px-2">Module</th>
            </tr>
          </thead>
          <tbody>
            {#each dynamicSystemState.components as component}
              <tr class="border-b border-zinc-900">
                <td class="py-3 px-2 text-sm text-white">abc</td>
                <td class="py-3 px-2">{component.id}</td>
                <td class="py-3 px-2 truncate">{component.label}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</WidgetCard>
