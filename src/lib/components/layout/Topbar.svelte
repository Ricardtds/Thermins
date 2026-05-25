<script lang="ts">
  import { systemState } from "$lib/stores/system.svelte";
  import { formatUptime } from "$lib/utils/time";
</script>

<header
  class="flex h-16 items-center justify-between border-b border-zinc-800 bg-[#0d0d12] px-6"
>
  <div
    class="flex items-center gap-3 text-sm uppercase tracking-[0.3em] text-cyan-300"
  >
    <div class="h-2 w-2 rounded-full bg-cyan-400"></div>

    <span>
      {formatUptime(systemState.uptime)}
    </span>
  </div>

  <div class="flex items-center gap-6 text-zinc-400">
    <button class="transition hover:text-cyan-300"> ⚙ </button>

    <button class="transition hover:text-cyan-300"> ⌁ </button>

    <div
      class="flex h-10 w-10 items-center justify-center border border-zinc-700 bg-zinc-900 text-sm"
    >
      RT
    </div>
  </div>
</header>
