import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { cpuHistory } from "$lib/stores/history.svelte";
import type { StaticSystemSnapshot, DynamicSystemSnapshot } from "$lib/types/system";

import {
    staticSystemState,
    dynamicSystemState,
    telemetryState,
} from "$lib/stores/system.svelte";

type StopListener = () => void;

let listenerPromise: Promise<StopListener> | null = null;

function getErrorMessage(error: unknown) {
    return error instanceof Error ? error.message : String(error);
}

export function startTelemetryListener(): Promise<StopListener> {
    if (listenerPromise) return listenerPromise;

    telemetryState.status = "connecting";
    telemetryState.message = "Waiting for the first telemetry sample";

    listenerPromise = listen<DynamicSystemSnapshot>(
        "system_snapshot",
        (event) => {
            Object.assign(dynamicSystemState, {
                ...event.payload,
                batteries: event.payload.batteries ?? [],
                components: event.payload.components ?? [],
                disks: event.payload.disks ?? [],
                networks: event.payload.networks ?? [],
                processes: event.payload.processes ?? [],
            });

            cpuHistory.push(
                event.payload.cpu.usage
            );

            if (cpuHistory.length > 60) {
                cpuHistory.shift();
            }

            telemetryState.status = "connected";
            telemetryState.message = "Live telemetry connected";
            telemetryState.lastUpdated = Date.now();
        }
    ).then((unlisten) => {
        return () => {
            unlisten();
            listenerPromise = null;
        };
    }).catch((error: unknown) => {
        listenerPromise = null;
        telemetryState.status = "unavailable";
        telemetryState.message = getErrorMessage(error);
        throw error;
    });

    return listenerPromise;
}

export async function getSystemInfo() {
    try {
        const system = await invoke<StaticSystemSnapshot>("get_static_info");

        Object.assign(staticSystemState, {
            ...system,
            batteries: system.batteries ?? [],
            disks: system.disks ?? [],
        });
    } catch (error) {
        telemetryState.status = "unavailable";
        telemetryState.message = getErrorMessage(error);
        throw error;
    }
}

export function startTelemetryWatchdog() {
    const checkConnection = () => {
        if (document.visibilityState === "hidden") return;
        if (telemetryState.lastUpdated === null) return;

        const sampleInterval = Math.max(dynamicSystemState.refreshRate, 1) * 1_000;
        const timeout = Math.max(sampleInterval * 3, 10_000);

        if (Date.now() - telemetryState.lastUpdated > timeout) {
            telemetryState.status = "unavailable";
            telemetryState.message = "Telemetry has not updated within the expected interval";
        }
    };

    const handleVisibilityChange = () => {
        if (document.visibilityState === "hidden") {
            telemetryState.status = "connecting";
            telemetryState.message = "Telemetry paused while the app is in the background";
            return;
        }

        if (telemetryState.lastUpdated !== null) {
            telemetryState.status = "connecting";
            telemetryState.message = "Resuming live telemetry";
            checkConnection();
        }
    };

    const interval = window.setInterval(checkConnection, 2_000);
    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
        window.clearInterval(interval);
        document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
}
