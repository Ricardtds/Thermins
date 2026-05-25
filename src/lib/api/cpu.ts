import { invoke } from "@tauri-apps/api/core";
import type { CpuInfo } from "$lib/types/cpu";

export async function getCpuInfo(): Promise<CpuInfo> {
    return invoke<CpuInfo>("cpu_info");
}