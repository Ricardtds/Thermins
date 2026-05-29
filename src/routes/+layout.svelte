<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";

  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
  import {
    startTelemetryListener,
    getSystemInfo,
  } from "$lib/telemetry/listener";
  import BottomBar from "$lib/components/layout/BottomBar.svelte";

  let sidebarOpen = $state(false);

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  function closeSidebar() {
    sidebarOpen = false;
  }

  onMount(() => {
    getSystemInfo();
    startTelemetryListener();
  });

  let { children } = $props();
</script>

<div class="flex h-screen overflow-hidden bg-[#09090c] text-zinc-100">
  <Sidebar isOpen={sidebarOpen} {closeSidebar} />

  <div class="flex flex-1 flex-col overflow-hidden">
    <Topbar {toggleSidebar} />

    <main
      class="flex-1 overflow-auto bg-[radial-gradient(circle_at_center,rgba(0,255,255,0.03)_0,transparent_70%)]"
    >
      {@render children()}
    </main>
    <BottomBar />
  </div>
</div>
