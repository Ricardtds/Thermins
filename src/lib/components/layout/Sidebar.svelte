<script lang="ts">
  import { page } from "$app/state";
  import SystemIcon from "$lib/components/ui/SystemIcon.svelte";
  import { onMount } from "svelte";

  let {
    isOpen = false,
    closeSidebar = () => {},
  }: { isOpen?: boolean; closeSidebar?: () => void } = $props();

  let isDesktop = $state(false);

  const items = [
    { href: "/dashboard", label: "Dashboard", symbol: "monitoring" },
    { href: "/processes", label: "Processes", symbol: "memory" },
    { href: "/sensors", label: "Sensors", symbol: "thermostat" },
    { href: "/terminal", label: "Terminal", symbol: "terminal" },
  ];

  function isActive(href: string) {
    return page.url.pathname === href || page.url.pathname.startsWith(`${href}/`);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && isOpen && !isDesktop) closeSidebar();
  }

  onMount(() => {
    const mediaQuery = window.matchMedia("(min-width: 64rem)");
    const updateViewport = () => (isDesktop = mediaQuery.matches);

    updateViewport();
    mediaQuery.addEventListener("change", updateViewport);

    return () => mediaQuery.removeEventListener("change", updateViewport);
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <button
    type="button"
    class="fixed inset-0 z-40 bg-black/65 backdrop-blur-sm lg:hidden"
    onclick={closeSidebar}
    aria-label="Fechar menu"
  ></button>
{/if}

<aside
  aria-label="Navegação principal"
  aria-hidden={!isDesktop && !isOpen ? "true" : undefined}
  inert={!isDesktop && !isOpen}
  class={`app-sidebar fixed inset-y-0 left-0 z-50 flex w-64 shrink-0 flex-col border-r border-zinc-800 bg-[#111116] transition-transform duration-200 lg:static lg:z-auto lg:w-60 lg:translate-x-0 ${
    isOpen ? "translate-x-0" : "-translate-x-full"
  }`}
>
  <div class="flex items-center justify-between gap-4 border-b border-zinc-800 p-5">
    <div class="min-w-0">
      <h1 class="text-2xl font-bold tracking-wider text-zinc-100">Thermins</h1>
      <p class="mt-1 text-[10px] uppercase tracking-[0.28em] text-zinc-500">
        System monitor
      </p>
    </div>

    <button
      type="button"
      class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg border border-zinc-700 bg-zinc-900 text-zinc-300 transition hover:border-cyan-400 hover:text-cyan-300 lg:hidden"
      onclick={closeSidebar}
      aria-label="Fechar menu"
    >
      ✕
    </button>
  </div>

  <nav class="flex-1 space-y-2 overflow-y-auto p-4">
    {#each items as item}
      <a
        href={item.href}
        aria-current={isActive(item.href) ? "page" : undefined}
        class={`flex min-h-12 items-center gap-4 rounded-lg border px-4 py-3 text-xs uppercase tracking-[0.18em] transition-colors ${
          isActive(item.href)
            ? "border-cyan-400/30 bg-cyan-400/10 text-cyan-300"
            : "border-transparent text-zinc-400 hover:border-zinc-800 hover:bg-zinc-900 hover:text-zinc-100"
        }`}
        onclick={closeSidebar}
      >
        <SystemIcon name={item.symbol} size={20} class="shrink-0" />
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>

  <div class="border-t border-zinc-800 p-4 text-[10px] uppercase tracking-[0.2em] text-zinc-600">
    Local telemetry · v0.1.0
  </div>
</aside>
