<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";

  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
  import { startTelemetryListener } from "$lib/telemetry/listener";

  onMount(() => {
    startTelemetryListener();
  });

  let { children } = $props();
</script>

<div class="flex h-screen overflow-hidden bg-[#09090c] text-zinc-100">
  <Sidebar />

  <div class="flex flex-1 flex-col overflow-hidden">
    <Topbar />

    <main
      class="flex-1 overflow-auto bg-[radial-gradient(circle_at_center,rgba(0,255,255,0.03)_0,transparent_70%)]"
    >
      {@render children()}
    </main>
  </div>
</div>
