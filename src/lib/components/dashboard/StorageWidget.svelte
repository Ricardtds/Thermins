<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";

  import {
    staticSystemState,
    dynamicSystemState,
  } from "$lib/stores/system.svelte";

  const GB = 1024 * 1024 * 1024;

  function getDynamicDisk(name: string) {
    return dynamicSystemState.disks.find((disk) => disk.name === name);
  }
</script>

<div class="col-span-12 xl:col-span-4">
  <WidgetCard title="STORAGE CAPACITY">
    <div class="space-y-6">
      {#each staticSystemState.disks as disk}
        {@const dynamicDisk = getDynamicDisk(disk.name)}

        {#if dynamicDisk}
          {@const usedSpace = disk.totalSpace - dynamicDisk.availableSpace}

          {@const usagePercent = (usedSpace / disk.totalSpace) * 100}

          <div class="border border-zinc-800 bg-zinc-950/40 p-4">
            <!-- HEADER -->
            <div class="mb-4 flex items-center justify-between">
              <div>
                <p class="text-xs uppercase tracking-widest text-zinc-500">
                  {disk.kind}
                </p>

                <h3 class="font-mono text-lg text-zinc-100">
                  {disk.mountPoint}
                </h3>
              </div>

              <span
                class="rounded border border-zinc-700 px-2 py-1 text-xs uppercase text-cyan-300"
              >
                {disk.filesystem}
              </span>
            </div>

            <!-- CIRCLE -->
            <div class="mb-6 flex items-center justify-center">
              <div
                class="flex h-40 w-40 flex-col items-center justify-center rounded-full border-12 border-cyan-400"
              >
                <span class="text-4xl font-bold text-zinc-100">
                  {usagePercent.toFixed(0)}%
                </span>

                <span
                  class="mt-1 text-xs uppercase tracking-widest text-zinc-500"
                >
                  Used
                </span>
              </div>
            </div>

            <!-- STORAGE -->
            <div class="mb-4 grid grid-cols-2 gap-4 text-sm">
              <div>
                <p class="text-zinc-500">Used</p>

                <p class="font-semibold text-zinc-100">
                  {(usedSpace / GB).toFixed(1)} GB
                </p>
              </div>

              <div class="text-right">
                <p class="text-zinc-500">Available</p>

                <p class="font-semibold text-cyan-300">
                  {(dynamicDisk.availableSpace / GB).toFixed(1)} GB
                </p>
              </div>
            </div>

            <!-- IO -->
            <div class="grid grid-cols-2 gap-4 border-t border-zinc-800 pt-4">
              <div>
                <p class="text-xs uppercase tracking-widest text-zinc-500">
                  Read
                </p>

                <p class="text-zinc-100">
                  {(dynamicDisk.readBytes / (1024 * 1024)).toFixed(2)}
                  MB/s
                </p>
              </div>

              <div class="text-right">
                <p class="text-xs uppercase tracking-widest text-zinc-500">
                  Write
                </p>

                <p class="text-zinc-100">
                  {(dynamicDisk.writtenBytes / (1024 * 1024)).toFixed(2)}
                  MB/s
                </p>
              </div>
            </div>
          </div>
        {/if}
      {/each}
    </div>
  </WidgetCard>
</div>
