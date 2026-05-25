<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";

  const events = [
    {
      time: "14:22:01.045",
      service: "KERNEL_PROC",
      event: "I/O Scheduler optimized",
      level: "INFO",
    },
    {
      time: "14:19:55.201",
      service: "NET_MANAGER",
      event: "DNS cache flushed",
      level: "INFO",
    },
    {
      time: "14:15:30.988",
      service: "AUTH_DAEMON",
      event: "Failed login attempt",
      level: "WARN",
    },
  ];
</script>

<div class="col-span-12 xl:col-span-9">
  <WidgetCard title="RECENT SYSTEM EVENTS">
    <table class="w-full border-collapse">
      <thead>
        <tr
          class="border-b border-zinc-800 text-left text-xs uppercase tracking-widest text-zinc-500"
        >
          <th class="pb-4">Timestamp</th>
          <th class="pb-4">Service</th>
          <th class="pb-4">Event</th>
          <th class="pb-4">Level</th>
        </tr>
      </thead>

      <tbody>
        {#each events as event}
          <tr class="border-b border-zinc-900 text-sm text-zinc-300">
            <td class="py-4">{event.time}</td>
            <td class="py-4">{event.service}</td>
            <td class="py-4">{event.event}</td>
            <td class="py-4 text-cyan-300">{event.level}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </WidgetCard>
</div>
