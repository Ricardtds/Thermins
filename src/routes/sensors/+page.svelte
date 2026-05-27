<script lang="ts">
  import WidgetCard from "$lib/components/ui/WidgetCard.svelte";

  import { systemState } from "$lib/stores/system.svelte";

  function getTemperatureColor(temp: number) {
    if (temp >= 85) {
      return "text-red-400";
    }

    if (temp >= 70) {
      return "text-orange-400";
    }

    if (temp >= 55) {
      return "text-yellow-300";
    }

    return "text-cyan-300";
  }

  function getBarColor(temp: number) {
    if (temp >= 85) {
      return "bg-red-500";
    }

    if (temp >= 70) {
      return "bg-orange-400";
    }

    if (temp >= 55) {
      return "bg-yellow-300";
    }

    return "bg-cyan-400";
  }
</script>

<section class="grid grid-cols-12 gap-4">
  <!-- THERMAL OVERVIEW -->
  <div class="col-span-12 xl:col-span-4">
    <WidgetCard title="THERMAL OVERVIEW">
      <div class="space-y-6">
        <!-- GLOBAL TEMP -->
        <div
          class="
            border
            border-zinc-800
            bg-black/40
            p-6
          "
        >
          <p
            class="
              text-xs
              uppercase
              tracking-[0.25em]
              text-zinc-500
            "
          >
            Package Temperature
          </p>

          <div class="mt-5 flex items-end gap-3">
            <h2
              class="
                text-7xl
                font-black
                text-orange-400
              "
            >
              {systemState.components[4]?.temperature.toFixed(0)}
            </h2>

            <span
              class="
                mb-2
                text-2xl
                text-zinc-500
              "
            >
              °C
            </span>
          </div>

          <div
            class="
              mt-6
              h-2
              overflow-hidden
              bg-zinc-900
            "
          >
            <div
              class="
                h-full
                bg-orange-400
              "
              style={`width: ${systemState.components[4]?.temperature || 0}%`}
            ></div>
          </div>
        </div>

        <!-- SENSOR STATUS -->
        <div
          class="
            grid
            grid-cols-2
            gap-4
          "
        >
          <div
            class="
              border
              border-zinc-800
              p-4
            "
          >
            <p
              class="
                text-xs
                uppercase
                tracking-[0.25em]
                text-zinc-500
              "
            >
              Active Sensors
            </p>

            <p
              class="
                mt-4
                text-5xl
                font-bold
                text-cyan-300
              "
            >
              {systemState.components.length}
            </p>
          </div>

          <div
            class="
              border
              border-zinc-800
              p-4
            "
          >
            <p
              class="
                text-xs
                uppercase
                tracking-[0.25em]
                text-zinc-500
              "
            >
              Critical Limit
            </p>

            <p
              class="
                mt-4
                text-5xl
                font-bold
                text-red-400
              "
            >
              100°
            </p>
          </div>
        </div>
      </div>
    </WidgetCard>
  </div>

  <!-- SENSOR MATRIX -->
  <div class="col-span-12 xl:col-span-8">
    <WidgetCard title="CORE DISTRIBUTION MATRIX">
      <div
        class="
          grid
          grid-cols-1
          gap-4
          xl:grid-cols-2
        "
      >
        {#each systemState.components as sensor}
          <div
            class="
              border
              border-zinc-800
              bg-black/30
              p-5
              transition
              hover:border-cyan-400/40
            "
          >
            <!-- HEADER -->
            <div
              class="
                flex
                items-start
                justify-between
                gap-4
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
                  Sensor Label
                </p>

                <h3
                  class="
                    mt-2
                    text-lg
                    font-bold
                    text-zinc-100
                  "
                >
                  {sensor.label}
                </h3>
              </div>

              <div
                class={`
                  text-4xl
                  font-black

                  ${getTemperatureColor(sensor.temperature)}
                `}
              >
                {sensor.temperature.toFixed(1)}°
              </div>
            </div>

            <!-- THERMAL BAR -->
            <div class="mt-6">
              <div
                class="
                  flex
                  items-center
                  justify-between
                  text-xs
                  uppercase
                  tracking-widest
                  text-zinc-500
                "
              >
                <span> Thermal Load </span>

                <span>
                  MAX:
                  {sensor.max_temperature.toFixed(0)}°
                </span>
              </div>

              <div
                class="
                  mt-3
                  h-3
                  overflow-hidden
                  border
                  border-zinc-800
                  bg-zinc-950
                "
              >
                <div
                  class={`
                    h-full
                    transition-all
                    duration-300

                    ${getBarColor(sensor.temperature)}
                  `}
                  style={`width: ${Math.min(sensor.temperature, 100)}%`}
                ></div>
              </div>
            </div>

            <!-- STATS -->
            <div
              class="
                mt-6
                grid
                grid-cols-3
                gap-3
              "
            >
              <div
                class="
                  border
                  border-zinc-800
                  p-3
                "
              >
                <p
                  class="
                    text-[10px]
                    uppercase
                    tracking-widest
                    text-zinc-600
                  "
                >
                  Current
                </p>

                <p
                  class="
                    mt-2
                    text-xl
                    font-bold
                    text-zinc-100
                  "
                >
                  {sensor.temperature.toFixed(1)}°
                </p>
              </div>

              <div
                class="
                  border
                  border-zinc-800
                  p-3
                "
              >
                <p
                  class="
                    text-[10px]
                    uppercase
                    tracking-widest
                    text-zinc-600
                  "
                >
                  Max
                </p>

                <p
                  class="
                    mt-2
                    text-xl
                    font-bold
                    text-orange-400
                  "
                >
                  {sensor.max_temperature.toFixed(1)}°
                </p>
              </div>

              <div
                class="
                  border
                  border-zinc-800
                  p-3
                "
              >
                <p
                  class="
                    text-[10px]
                    uppercase
                    tracking-widest
                    text-zinc-600
                  "
                >
                  Critical
                </p>

                <p
                  class="
                    mt-2
                    text-xl
                    font-bold
                    text-red-400
                  "
                >
                  {sensor.critical.toFixed(0)}°
                </p>
              </div>
            </div>

            <!-- SENSOR ID -->
            <div
              class="
                mt-5
                border-t
                border-zinc-800
                pt-4
              "
            >
              <p
                class="
                  truncate
                  font-mono
                  text-xs
                  text-zinc-600
                "
              >
                {sensor.id}
              </p>
            </div>
          </div>
        {/each}
      </div>
    </WidgetCard>
  </div>
</section>
