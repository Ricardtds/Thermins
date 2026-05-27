import type { CpuInfo } from "./cpu";
import type { MemoryInfo } from "./memory";
import type { SensoInfo } from "./component";
import type { DiskInfo } from "./disk";
import type { NetworkInfo } from "./network";
import type { ProcessInfo } from "./process";

export interface SystemSnapshot {
    components: SensoInfo[];
    cpu: CpuInfo;
    disks: DiskInfo[];
    memory: MemoryInfo;
    networks: NetworkInfo[];
    processes: ProcessInfo[];
    uptime: number;
}