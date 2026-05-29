<script lang="ts">
  import PowerStatus from "$lib/components/dashboard/PowerStatus.svelte";
  import ThermalAnalysis from "$lib/components/dashboard/ThermalAnalysis.svelte";
  import ThermalGrid from "$lib/components/dashboard/ThermalGrid.svelte";
  import SensorLog from "$lib/components/dashboard/SensorLog.svelte";
</script>

<section class="grid gap-4 bg-[#07090f] p-4 md:p-6 lg:p-8">
  <div class="grid gap-4 xl:grid-cols-[1fr_1fr_1fr]">
    <ThermalAnalysis />
    <PowerStatus />
    <SensorLog />
  </div>

  <div class="grid gap-4 lg:grid-cols-[2fr]">
    <ThermalGrid />
  </div>
</section>
