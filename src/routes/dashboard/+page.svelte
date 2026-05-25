<script lang="ts">
  import CpuChart from "$lib/components/dashboard/CpuChart.svelte";
  import EventTable from "$lib/components/dashboard/EventTable.svelte";
  import HardwareStatus from "$lib/components/dashboard/HardwareStatus.svelte";
  import NetworkWidget from "$lib/components/dashboard/NetworkWidget.svelte";
  import RamWidget from "$lib/components/dashboard/RamWidget.svelte";
  import StorageWidget from "$lib/components/dashboard/StorageWidget.svelte";
</script>

<section class="grid grid-cols-12 gap-4 bg-[#0b0c10] p-4">
  <CpuChart />

  <RamWidget />

  <StorageWidget />

  <NetworkWidget />

  <EventTable />

  <HardwareStatus />
</section>
