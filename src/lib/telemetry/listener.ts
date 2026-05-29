import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { cpuHistory } from "$lib/stores/history.svelte";
import type { StaticSystemSnapshot, DynamicSystemSnapshot } from "$lib/types/system";

import { staticSystemState, dynamicSystemState } from "$lib/stores/system.svelte";

let initialized = false;

export async function startTelemetryListener() {
    if (initialized) return;

    initialized = true;

    await listen<DynamicSystemSnapshot>(
        "system_snapshot",
        (event) => {
            Object.assign(
                dynamicSystemState,
                event.payload
            );
            console.log(event.payload);

            cpuHistory.push(
                event.payload.cpu.usage
            );

            if (cpuHistory.length > 60) {
                cpuHistory.shift();
            }
        }
    );
}

export async function getSystemInfo() {
    let system = await invoke<StaticSystemSnapshot>("get_static_info")
    console.log("Vindo do invoke", system);
    Object.assign(
        staticSystemState,
        system
    );
}