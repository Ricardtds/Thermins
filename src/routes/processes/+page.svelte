<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";
  import { dynamicSystemState } from "$lib/stores/system.svelte";

  let search = $state("");

  const filteredProcesses = $derived.by(() =>
    dynamicSystemState.processes.filter((process) =>
      process.name.toLowerCase().includes(search.toLowerCase()),
    ),
  );

  function formatMemory(bytes: number) {
    return (bytes / 1024 / 1024).toFixed(1);
  }
</script>

<WidgetCard title="ACTIVE PROCESSES" class="h-full flex flex-col">
  <div class="flex flex-col h-full">
    <!-- HEADER -->
    <div
      class="mb-4 flex flex-col gap-3 xl:flex-row xl:items-center xl:justify-between"
    >
      <div class="flex justify-between w-full">
        <div>
          <p
            class="text-[10px] sm:text-xs uppercase tracking-[0.25em] text-zinc-500"
          >
            Process Monitor
          </p>
          <h2
            class="mt-1 text-2xl sm:text-3xl md:text-4xl font-black text-zinc-100"
          >
            {dynamicSystemState.processes.length}
          </h2>
        </div>

        <!-- <p class="mt-1 text-xs sm:text-sm text-zinc-500">
          Active system processes
        </p> -->
      </div>

      <div class="w-full xl:w-80">
        <input
          bind:value={search}
          type="text"
          aria-label="Search processes"
          placeholder="Search process..."
          class="w-full border border-zinc-800 bg-black px-3 py-2 sm:px-4 sm:py-3 text-xs sm:text-sm text-zinc-200 outline-none transition placeholder:text-zinc-600 focus:border-cyan-400"
        />
      </div>
    </div>

    <!-- TABLE -->
    <div class="min-h-0 flex-1 overflow-auto border border-zinc-800">
      <div class="min-w-[820px]">
        <table class="w-full table-fixed border-collapse text-xs sm:text-sm">
          <!-- HEADER -->
          <thead class="sticky top-0 z-20 bg-[#0f1015]">
            <tr
              class="border-b border-zinc-800 text-left uppercase tracking-[0.25em] text-zinc-500 text-[10px] sm:text-xs"
            >
              <th class="px-2 sm:px-3 md:px-4 py-2">PID</th>

              <th class="px-2 sm:px-3 md:px-4 py-2">Process</th>

              <th class="px-2 sm:px-3 md:px-4 py-2">CPU</th>

              <th class="px-2 sm:px-3 md:px-4 py-2">Memory</th>

              <th class="px-2 sm:px-3 md:px-4 py-2">Directory</th>
            </tr>
          </thead>

          <!-- BODY -->
          <tbody>
            {#each filteredProcesses as process}
              <tr
                class="border-b border-zinc-900 transition hover:bg-cyan-500/5"
              >
                <!-- PID -->
                <td class="px-2 sm:px-3 md:px-4 py-2 font-mono text-cyan-300">
                  {process.id}
                </td>

                <!-- NAME -->
                <td class="px-2 sm:px-3 md:px-4 py-2">
                  <p class="font-semibold text-zinc-100 text-xs sm:text-sm">
                    {process.name}
                  </p>

                  <p
                    class="text-[10px] sm:text-xs uppercase tracking-widest text-zinc-600"
                  >
                    runtime
                  </p>
                </td>

                <!-- CPU -->
                <td class="px-2 sm:px-3 md:px-4 py-2">
                  <div class="flex items-center gap-2 sm:gap-3">
                    <div
                      class="h-2 w-12 sm:w-16 md:w-28 overflow-hidden bg-zinc-900"
                    >
                      <div
                        class="h-full bg-cyan-400 transition-all duration-300"
                        style={`width: ${Math.min(process.cpuUsage, 100)}%`}
                      ></div>
                    </div>

                    <span
                      class="w-10 text-right font-mono text-zinc-300 text-[10px] sm:text-xs"
                    >
                      {process.cpuUsage.toFixed(1)}%
                    </span>
                  </div>
                </td>

                <!-- MEMORY -->
                <td class="px-2 sm:px-3 md:px-4 py-2">
                  <div>
                    <p
                      class="font-mono text-fuchsia-300 text-[10px] sm:text-xs"
                    >
                      {formatMemory(process.memoryUsage)} MB
                    </p>

                    <div class="mt-1 h-1.5 bg-zinc-900">
                      <div
                        class="h-full bg-fuchsia-400"
                        style={`width: ${Math.min(
                          process.memoryUsage / 1024 / 1024 / 10,
                          100,
                        )}%`}
                      ></div>
                    </div>
                  </div>
                </td>

                <!-- DIRECTORY -->
                <td class="px-2 sm:px-3 md:px-4 py-2">
                  <p
                    class="truncate font-mono text-[10px] sm:text-xs text-zinc-500 max-w-[120px] sm:max-w-[200px] md:max-w-[420px]"
                    title={process.workingDirectory}
                  >
                    {process.workingDirectory || "-"}
                  </p>
                </td>
              </tr>
            {:else}
              <tr>
                <td
                  colspan="5"
                  class="px-4 py-12 text-center text-sm text-zinc-500"
                >
                  No process matches this search.
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- FOOTER -->
    <div
      class="mt-3 flex items-center justify-between border-t border-zinc-800 pt-3 text-[10px] sm:text-xs uppercase tracking-[0.25em] text-zinc-500"
    >
      <span>Real-Time Process Telemetry</span>

      <span>{filteredProcesses.length} Processes</span>
    </div>
  </div>
</WidgetCard>
