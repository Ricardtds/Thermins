<script lang="ts">
  import PowerStatus from "$lib/components/dashboard/PowerStatus.svelte";
  import ThermalAnalysis from "$lib/components/dashboard/ThermalAnalysis.svelte";
  import ThermalGrid from "$lib/components/dashboard/ThermalGrid.svelte";
  import SensorLog from "$lib/components/dashboard/SensorLog.svelte";
</script>

<section
  class="min-h-full min-w-0 bg-[#07090f] p-3 sm:p-4 md:p-6 lg:p-8"
  aria-labelledby="sensors-page-title"
>
  <h1 id="sensors-page-title" class="sr-only">System sensors</h1>

  <div
    class="grid min-w-0 items-stretch gap-4 lg:grid-cols-2 2xl:grid-cols-12"
  >
    <div class="min-w-0 2xl:col-span-5">
      <ThermalAnalysis />
    </div>

    <div class="min-w-0 2xl:col-span-3">
      <PowerStatus />
    </div>

    <div class="min-w-0 lg:col-span-2 2xl:col-span-4">
      <SensorLog />
    </div>
  </div>

  <div class="mt-4 min-w-0">
    <ThermalGrid />
  </div>
</section>
