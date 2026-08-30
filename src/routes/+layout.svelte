<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";

  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Topbar from "$lib/components/layout/Topbar.svelte";
  import {
    startTelemetryListener,
    startTelemetryWatchdog,
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
    let stopListener: (() => void) | undefined;
    let mounted = true;
    const stopWatchdog = startTelemetryWatchdog();

    void startTelemetryListener()
      .then((stop) => {
        if (mounted) {
          stopListener = stop;
        } else {
          stop();
        }
      })
      .catch(() => {
        // The status store exposes this failure in the UI. Browser-only preview
        // is allowed to render without a running Tauri backend.
      });

    void getSystemInfo().catch(() => {
      // Dynamic telemetry may still recover even if the static snapshot fails.
    });

    return () => {
      mounted = false;
      stopListener?.();
      stopWatchdog();
    };
  });

  let { children } = $props();
</script>

<div class="flex h-dvh min-h-0 overflow-hidden bg-[#09090c] text-zinc-100">
  <Sidebar isOpen={sidebarOpen} {closeSidebar} />

  <div class="flex flex-1 flex-col overflow-hidden">
    <Topbar {toggleSidebar} />

    <main
      class="app-content flex-1 overflow-auto overscroll-contain pb-[calc(4.75rem+env(safe-area-inset-bottom))] bg-[radial-gradient(circle_at_center,rgba(0,255,255,0.03)_0,transparent_70%)] lg:pb-0"
    >
      {@render children()}
    </main>
    <BottomBar />
  </div>
</div>
