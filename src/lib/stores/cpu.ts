import { writable } from "svelte/store";
import type { CpuInfo } from "$lib/types/cpu";
import { getCpuInfo } from "$lib/api/cpu";

export const cpuStore = writable<CpuInfo | null>(null);

export async function updateCpu() {
    const data = await getCpuInfo();

    cpuStore.set(data);
}