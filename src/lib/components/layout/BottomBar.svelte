<script lang="ts">
  import { page } from "$app/state";
  import SystemIcon from "$lib/components/ui/SystemIcon.svelte";

  const items = [
    { href: "/dashboard", label: "Dashboard", symbol: "monitoring" },
    { href: "/processes", label: "Processes", symbol: "memory" },
    { href: "/sensors", label: "Sensors", symbol: "thermostat" },
    { href: "/terminal", label: "Terminal", symbol: "terminal" },
  ];

  function isActive(href: string) {
    return page.url.pathname === href || page.url.pathname.startsWith(`${href}/`);
  }
</script>

<nav
  aria-label="Navegação principal"
  class="app-bottom-bar fixed inset-x-0 bottom-0 z-40 grid grid-cols-4 border-t border-white/10 bg-[#111317]/95 px-1 pb-[env(safe-area-inset-bottom)] backdrop-blur-xl lg:hidden"
>
  {#each items as item}
    <a
      href={item.href}
      aria-current={isActive(item.href) ? "page" : undefined}
      class={`flex min-h-[4.5rem] min-w-0 flex-col items-center justify-center gap-1 rounded-lg px-1 text-center transition-colors ${
        isActive(item.href)
          ? "bg-cyan-400/8 text-cyan-300"
          : "text-zinc-500 hover:bg-white/5 hover:text-zinc-200"
      }`}
    >
      <SystemIcon name={item.symbol} size={24} class="shrink-0" />
      <span class="max-w-full truncate text-[9px] font-bold uppercase tracking-tight">
        {item.label}
      </span>
    </a>
  {/each}
</nav>
