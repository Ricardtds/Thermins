<script lang="ts">
  import { page } from "$app/state";

  const items = [
    {
      href: "/dashboard",
      label: "Dashboard",
      icon: "◫",
    },
    {
      href: "/processes",
      label: "Processes",
      icon: "⌘",
    },
    {
      href: "/sensors",
      label: "Sensors",
      icon: "◉",
    },
    {
      href: "/logs",
      label: "Logs",
      icon: "▤",
    },
    {
      href: "/performance",
      label: "Performance",
      icon: "◌",
    },
  ];
</script>

<aside
  class="flex h-screen w-64 flex-col border-r border-zinc-800 bg-[#111116]"
>
  <div class="border-b border-zinc-800 p-6">
    <h1 class="text-4xl font-bold tracking-wider text-zinc-100">Thermins</h1>

    <p class="mt-1 text-sm uppercase tracking-widest text-zinc-500">
      Local Host
    </p>
  </div>

  <nav class="flex-1 space-y-2 p-4">
    {#each items as item}
      <a
        href={item.href}
        class={`flex items-center gap-4 border border-transparent px-4 py-4 text-sm uppercase tracking-widest transition-all ${
          page.url.pathname === item.href
            ? "border-zinc-700 bg-black text-cyan-300"
            : "text-zinc-400 hover:border-zinc-800 hover:bg-zinc-900"
        }`}
      >
        <span class="text-lg">
          {item.icon}
        </span>

        <span>
          {item.label}
        </span>
      </a>
    {/each}
  </nav>

  <div class="border-t border-zinc-800 p-4">
    <button
      class="w-full border border-zinc-700 px-4 py-3 text-sm uppercase tracking-widest text-zinc-300 transition hover:border-cyan-400 hover:text-cyan-300"
    >
      Reboot
    </button>
  </div>
</aside>
