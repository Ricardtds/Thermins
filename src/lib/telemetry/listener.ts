import { listen } from "@tauri-apps/api/event";
import { cpuHistory } from "$lib/stores/history.svelte";
import type { SystemSnapshot } from "$lib/types/system";

import { systemState } from "$lib/stores/system.svelte";

let initialized = false;

export async function startTelemetryListener() {
    if (initialized) return;

    initialized = true;

    await listen<SystemSnapshot>(
        "system_snapshot",
        (event) => {
            if (systemState === null) {
                Object.assign(
                    systemState = $state(event.payload)
                );

                return;
            }
            cpuHistory.push(
                event.payload.cpu.usage
            );

            if (cpuHistory.length > 60) {
                cpuHistory.shift();
            }
            Object.assign(systemState, event.payload);
        }
    );
}