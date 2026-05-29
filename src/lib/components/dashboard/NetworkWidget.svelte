<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";
</script>

<div class="col-span-12 xl:col-span-8">
  <WidgetCard title="NETWORK I/O TRAFFIC">
    <div class="space-y-6">
      {#each dynamicSystemState.networks as network}
        <div class="border border-zinc-800 bg-zinc-950/30">
          <!-- HEADER -->
          <div
            class="flex items-center justify-between border-b border-zinc-800 px-4 py-3"
          >
            <div>
              <p class="text-xs uppercase tracking-widest text-zinc-500">
                Interface
              </p>

              <h3 class="font-mono text-lg text-zinc-100">
                {network.name}
              </h3>
            </div>

            <div class="flex gap-8 text-right">
              <div>
                <p class="text-xs uppercase tracking-widest text-zinc-500">
                  Total Download
                </p>

                <p class="text-sm font-semibold text-cyan-300">
                  {network.totalReceived.toFixed(2)} Mb
                </p>
              </div>

              <div>
                <p class="text-xs uppercase tracking-widest text-zinc-500">
                  Total Upload
                </p>

                <p class="text-sm font-semibold text-fuchsia-400">
                  {network.totalTransmitted.toFixed(2)} Mb
                </p>
              </div>
            </div>
          </div>

          <!-- BODY -->
          <div class="grid grid-cols-1 gap-6 p-4 xl:grid-cols-2">
            <!-- DOWNLOAD -->
            <div>
              <div class="mb-4 flex items-center justify-between">
                <span class="text-xs uppercase tracking-widest text-zinc-500">
                  Download Rate
                </span>

                <span class="text-4xl font-bold text-cyan-300">
                  {network.received.toFixed(2)}
                  <span class="text-lg">Mbps</span>
                </span>
              </div>

              <div class="h-32 border border-zinc-800 bg-cyan-950/20"></div>
            </div>

            <!-- UPLOAD -->
            <div>
              <div class="mb-4 flex items-center justify-between">
                <span class="text-xs uppercase tracking-widest text-zinc-500">
                  Upload Rate
                </span>

                <span class="text-4xl font-bold text-fuchsia-400">
                  {network.transmitted.toFixed(2)}
                  <span class="text-lg">Mbps</span>
                </span>
              </div>

              <div class="h-32 border border-zinc-800 bg-fuchsia-950/20"></div>
            </div>
          </div>

          <!-- FOOTER -->
          <div
            class="flex justify-between border-t border-zinc-800 px-4 py-2 text-xs uppercase tracking-widest text-zinc-500"
          >
            <span>
              RX Total:
              {network.totalReceived.toFixed(2)} Mb
            </span>

            <span>
              TX Total:
              {network.totalTransmitted.toFixed(2)} Mb
            </span>
          </div>
        </div>
      {/each}
    </div>
  </WidgetCard>
</div>
