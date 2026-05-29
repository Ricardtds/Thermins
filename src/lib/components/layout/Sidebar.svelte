<script lang="ts">
  import { page } from "$app/state";

  export let isOpen = false;
  export let closeSidebar: () => void = () => {};

  const items = [
    { href: "/dashboard", label: "Dashboard", icon: "◫" },
    { href: "/processes", label: "Processes", icon: "⌘" },
    { href: "/sensors", label: "Sensors", icon: "◉" },
    { href: "/logs", label: "Logs", icon: "▤" },
    { href: "/performance", label: "Performance", icon: "◌" },
  ];
</script>

<!-- BACKDROP -->
{#if isOpen}
  <button
    class="fixed inset-0 z-40 bg-black/50"
    on:click={closeSidebar}
    aria-label="Fechar menu"
  />
{/if}

<!-- SIDEBAR -->
<aside
  class={`fixed inset-y-0 left-0 z-50 w-64 flex flex-col border-r border-zinc-800 bg-[#111116] transition-transform duration-300 ${
    isOpen ? "translate-x-0" : "-translate-x-full"
  }`}
>
  <!-- HEADER -->
  <div
    class="flex items-center justify-between gap-4 border-b border-zinc-800 p-6"
  >
    <div>
      <h1 class="text-4xl font-bold tracking-wider text-zinc-100">Thermins</h1>
      <p class="mt-1 text-sm uppercase tracking-widest text-zinc-500">
        Local Host
      </p>
    </div>

    <button
      class="rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-sm uppercase tracking-[0.3em] text-zinc-300 transition hover:border-cyan-400 hover:text-cyan-300"
      on:click={closeSidebar}
    >
      ✕
    </button>
  </div>

  <!-- NAV -->
  <nav class="flex-1 space-y-2 p-4">
    {#each items as item}
      <a
        href={item.href}
        class={`flex items-center gap-4 border border-transparent px-4 py-4 text-sm uppercase tracking-widest transition-all ${
          page.url.pathname === item.href
            ? "border-zinc-700 bg-black text-cyan-300"
            : "text-zinc-400 hover:border-zinc-800 hover:bg-zinc-900"
        }`}
        on:click={closeSidebar}
      >
        <span class="text-lg">{item.icon}</span>
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>

  <!-- FOOTER -->
  <div class="border-t border-zinc-800 p-4">
    <button
      class="w-full border border-zinc-700 px-4 py-3 text-sm uppercase tracking-widest text-zinc-300 transition hover:border-cyan-400 hover:text-cyan-300"
    >
      Reboot
    </button>
  </div>
</aside>
