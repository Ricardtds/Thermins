<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";

  import { systemState } from "$lib/stores/system.svelte";

  let search = $state("");

  const filteredProcesses = $derived.by(() => {
    return systemState.processes
      .filter((process) => {
        return process.name.toLowerCase().includes(search.toLowerCase());
      })
      .sort((a, b) => {
        return b.memory_usage - a.memory_usage;
      });
  });

  function formatMemory(bytes: number) {
    return (bytes / 1024 / 1024).toFixed(1);
  }
</script>

<div class="col-span-12">
  <WidgetCard title="ACTIVE PROCESS TABLE">
    <!-- HEADER -->
    <div
      class="
        mb-6
        flex
        flex-col
        gap-4
        xl:flex-row
        xl:items-center
        xl:justify-between
      "
    >
      <div>
        <p
          class="
            text-xs
            uppercase
            tracking-[0.25em]
            text-zinc-500
          "
        >
          Process Monitor
        </p>

        <h2
          class="
            mt-2
            text-4xl
            font-black
            text-zinc-100
          "
        >
          {systemState.processes.length}
        </h2>

        <p class="mt-1 text-sm text-zinc-500">Active system processes</p>
      </div>

      <div class="w-full xl:w-80">
        <input
          bind:value={search}
          type="text"
          placeholder="Search process..."
          class="
            w-full
            border
            border-zinc-800
            bg-black
            px-4
            py-3
            text-sm
            text-zinc-200
            outline-none
            transition
            placeholder:text-zinc-600
            focus:border-cyan-400
          "
        />
      </div>
    </div>

    <!-- TABLE -->
    <div
      class="
        overflow-hidden
        border
        border-zinc-800
      "
    >
      <div class="max-h-175 overflow-auto">
        <table
          class="
            w-full
            border-collapse
            text-sm
          "
        >
          <thead
            class="
              sticky
              top-0
              z-10
              bg-[#0f1015]
            "
          >
            <tr
              class="
                border-b
                border-zinc-800
                text-left
                uppercase
                tracking-[0.25em]
                text-zinc-500
                text-xs
              "
            >
              <th class="px-4 py-4"> PID </th>

              <th class="px-4 py-4"> Process Name </th>

              <th class="px-4 py-4"> CPU </th>

              <th class="px-4 py-4"> Memory </th>

              <th class="px-4 py-4"> Working Directory </th>
            </tr>
          </thead>

          <tbody>
            {#each filteredProcesses as process}
              <tr
                class="
                  border-b
                  border-zinc-900
                  transition
                  hover:bg-cyan-500/5
                "
              >
                <!-- PID -->
                <td
                  class="
                    px-4
                    py-4
                    font-mono
                    text-cyan-300
                  "
                >
                  {process.id}
                </td>

                <!-- NAME -->
                <td class="px-4 py-4">
                  <div>
                    <p
                      class="
                        font-semibold
                        text-zinc-100
                      "
                    >
                      {process.name}
                    </p>

                    <p
                      class="
                        mt-1
                        text-xs
                        uppercase
                        tracking-widest
                        text-zinc-600
                      "
                    >
                      Runtime Process
                    </p>
                  </div>
                </td>

                <!-- CPU -->
                <td class="px-4 py-4">
                  <div class="flex items-center gap-3">
                    <div
                      class="
                        h-2
                        w-28
                        overflow-hidden
                        bg-zinc-900
                      "
                    >
                      <div
                        class="
                          h-full
                          bg-cyan-400
                          transition-all
                          duration-300
                        "
                        style={`width: ${Math.min(process.cpu_usage, 100)}%`}
                      ></div>
                    </div>

                    <span
                      class="
                        w-12
                        text-right
                        font-mono
                        text-zinc-300
                      "
                    >
                      {process.cpu_usage.toFixed(1)}%
                    </span>
                  </div>
                </td>

                <!-- MEMORY -->
                <td class="px-4 py-4">
                  <div>
                    <p
                      class="
                        font-mono
                        text-fuchsia-300
                      "
                    >
                      {formatMemory(process.memory_usage)}
                      MB
                    </p>

                    <div
                      class="
                        mt-2
                        h-1.5
                        overflow-hidden
                        bg-zinc-900
                      "
                    >
                      <div
                        class="
                          h-full
                          bg-fuchsia-400
                        "
                        style={`width: ${Math.min(
                          process.memory_usage / 1024 / 1024 / 10,
                          100,
                        )}%`}
                      ></div>
                    </div>
                  </div>
                </td>

                <!-- DIRECTORY -->
                <td class="px-4 py-4">
                  <p
                    class="
                      max-w-105
                      truncate
                      font-mono
                      text-xs
                      text-zinc-500
                    "
                  >
                    {process.working_directory || "-"}
                  </p>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- FOOTER -->
    <div
      class="
        mt-4
        flex
        items-center
        justify-between
        border-t
        border-zinc-800
        pt-4
        text-xs
        uppercase
        tracking-[0.25em]
        text-zinc-500
      "
    >
      <span> Real-Time Process Telemetry </span>

      <span> Updating Live </span>
    </div>
  </WidgetCard>
</div>
