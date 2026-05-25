<script lang="ts">
  let { title, children } = $props();
</script>

<div
  class="
    rounded-sm
    border
    border-zinc-800
    bg-[#111116]
    p-4
  "
>
  <div class="mb-4 text-xs tracking-widest text-zinc-400">
    {title}
  </div>

  {@render children()}
</div>
