import type { CpuInfo } from "./cpu";
import type { MemoryInfo } from "./memory";

export interface SystemSnapshot {
    cpu: CpuInfo;
    memory: MemoryInfo;
    uptime: number;
}