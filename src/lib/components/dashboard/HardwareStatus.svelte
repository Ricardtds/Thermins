<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
</script>

<div class="col-span-12 xl:col-span-3">
  <WidgetCard title="HARDWARE STATUS">
    <p class="leading-relaxed text-zinc-400">
      Identity verified via ECC-256 secure enclave. All modules operating within
      nominal parameters.
    </p>

    <div class="mt-10 border border-cyan-500/40 bg-black p-5">
      <div class="flex items-center gap-4">
        <div
          class="flex h-14 w-14 items-center justify-center bg-cyan-400 text-black"
        >
          ⛨
        </div>

        <div>
          <p class="text-xs uppercase tracking-widest text-cyan-300">
            Node Verified
          </p>

          <p class="mt-2 text-3xl font-bold text-zinc-100">SYS_HEALTH: 100%</p>
        </div>
      </div>
    </div>
  </WidgetCard>
</div>
