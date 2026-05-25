<script lang="ts">
  let { title, children, class: className = "" } = $props();
</script>

<div class={`border border-zinc-800 bg-[#111116] p-5 ${className}`}>
  <div class="mb-5 flex items-center justify-between">
    <h2 class="text-xs uppercase tracking-[0.25em] text-zinc-400">
      {title}
    </h2>

    <div class="h-px flex-1 bg-zinc-800 ml-4" />
  </div>

  {@render children()}
</div>
