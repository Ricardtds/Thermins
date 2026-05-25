<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
</script>

<div class="col-span-12 xl:col-span-4">
  <WidgetCard title="STORAGE CAPACITY">
    <div class="flex items-center justify-center py-6">
      <div
        class="flex h-56 w-56 items-center justify-center rounded-full border-[14px] border-cyan-400"
      >
        <div class="text-center">
          <p class="text-5xl font-bold text-zinc-100">75%</p>

          <p class="mt-2 text-sm uppercase tracking-widest text-zinc-500">
            2.1TB Free
          </p>
        </div>
      </div>
    </div>
  </WidgetCard>
</div>
