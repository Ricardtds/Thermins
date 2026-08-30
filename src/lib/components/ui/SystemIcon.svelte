<script lang="ts">
  let {
    name,
    size = 24,
    class: className = "",
  }: { name: string; size?: number; class?: string } = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.8"
  stroke-linecap="round"
  stroke-linejoin="round"
  class={className}
  aria-hidden="true"
>
  {#if name === "dashboard" || name === "monitoring"}
    <rect x="3" y="3" width="7" height="8" rx="1"></rect>
    <rect x="14" y="3" width="7" height="5" rx="1"></rect>
    <rect x="14" y="12" width="7" height="9" rx="1"></rect>
    <rect x="3" y="15" width="7" height="6" rx="1"></rect>
  {:else if name === "processes" || name === "memory"}
    <rect x="6" y="6" width="12" height="12" rx="2"></rect>
    <path d="M9 1v3M15 1v3M9 20v3M15 20v3M20 9h3M20 14h3M1 9h3M1 14h3"></path>
    <path d="M10 10h4v4h-4z"></path>
  {:else if name === "sensors" || name === "thermostat"}
    <path d="M14 14.8V5a4 4 0 0 0-8 0v9.8a6 6 0 1 0 8 0Z"></path>
    <path d="M10 11v6"></path>
    <circle cx="10" cy="17" r="1.5"></circle>
  {:else if name === "terminal"}
    <rect x="3" y="4" width="18" height="16" rx="2"></rect>
    <path d="m7 9 3 3-3 3M13 15h4"></path>
  {:else if name === "database"}
    <ellipse cx="12" cy="5" rx="8" ry="3"></ellipse>
    <path d="M4 5v6c0 1.7 3.6 3 8 3s8-1.3 8-3V5M4 11v6c0 1.7 3.6 3 8 3s8-1.3 8-3v-6"></path>
  {:else if name === "storage" || name === "hard_drive"}
    <rect x="3" y="5" width="18" height="14" rx="2"></rect>
    <path d="M7 15h.01M11 15h6"></path>
  {:else if name === "filter"}
    <path d="M4 6h16M7 12h10M10 18h4"></path>
  {:else if name === "wifi"}
    <path d="M5 12.6a10 10 0 0 1 14 0M8.5 16a5 5 0 0 1 7 0"></path>
    <circle cx="12" cy="19" r="1"></circle>
  {:else}
    <circle cx="12" cy="12" r="9"></circle>
    <path d="M12 8v4M12 16h.01"></path>
  {/if}
</svg>
