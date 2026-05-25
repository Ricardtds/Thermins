<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
</script>

<div class="col-span-12 xl:col-span-8">
  <WidgetCard title="NETWORK I/O TRAFFIC">
    <div class="grid grid-cols-1 gap-6 xl:grid-cols-2">
      <div>
        <div class="mb-4 flex items-center justify-between">
          <span class="text-xs uppercase tracking-widest text-zinc-500">
            Download
          </span>

          <span class="text-4xl font-bold text-cyan-300"> 124.5 Mbps </span>
        </div>

        <div class="h-32 border border-zinc-800 bg-cyan-950/20" />
      </div>

      <div>
        <div class="mb-4 flex items-center justify-between">
          <span class="text-xs uppercase tracking-widest text-zinc-500">
            Upload
          </span>

          <span class="text-4xl font-bold text-fuchsia-400"> 12.8 Mbps </span>
        </div>

        <div class="h-32 border border-zinc-800 bg-fuchsia-950/20" />
      </div>
    </div>
  </WidgetCard>
</div>
