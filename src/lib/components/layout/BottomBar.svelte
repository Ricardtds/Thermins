<script lang="ts">
  import { page } from "$app/state";

  const items = [
    { href: "/dashboard", label: "Dashboard", symbol: "monitoring" },
    { href: "/processes", label: "Processes", symbol: "memory" },
    { href: "/sensors", label: "Sensors", symbol: "thermostat" },
    { href: "/logs", label: "Terminal", symbol: "terminal" },
  ];
</script>

<!-- Bottom Navigation -->
<nav
  class="fixed bottom-0 left-0 w-full bg-[#111317]/90 backdrop-blur-xl border-t border-white/10 px-6 py-0 flex justify-around items-center z-50"
>
  {#each items as item}
    <a
      href={item.href}
      class={`flex items-center gap-4 border border-transparent px-4 py-4 text-sm uppercase tracking-widest transition-all ${
        page.url.pathname === item.href
          ? "border-zinc-700 bg-black text-cyan-300"
          : "text-zinc-400 hover:border-zinc-800 hover:bg-zinc-900"
      }`}
    >
      <button class="flex flex-col items-center gap-1 text-[#bd00ff] scale-105">
        <span class="material-symbols-outlined text-2xl">{item.symbol}</span>
        <span class="text-[9px] font-bold uppercase tracking-tighter"
          >{item.label}</span
        >
      </button>
    </a>
  {/each}
</nav>
