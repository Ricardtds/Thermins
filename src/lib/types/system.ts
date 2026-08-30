import type { StaticCpuInfo, DynamicCpuInfo } from "./cpu";
import type { StaticMemoryInfo, DynamicMemoryInfo } from "./memory";
import type { SensorInfo } from "./component";
import type { StaticDiskInfo, DynamicDiskInfo } from "./disk";
import type { NetworkInfo } from "./network";
import type { ProcessInfo } from "./process";
import type { HostInfo } from "./host";
import type { DynamicBatteryInfo, StaticBatteryInfo } from "./battery";

export interface DynamicSystemSnapshot {
    batteries: DynamicBatteryInfo[];
    components: SensorInfo[];
    cpu: DynamicCpuInfo;
    disks: DynamicDiskInfo[];
    memory: DynamicMemoryInfo;
    networks: NetworkInfo[];
    processes: ProcessInfo[];
    uptime: number;
    refreshRate: number;
}

export interface StaticSystemSnapshot {
    batteries: StaticBatteryInfo[];
    host: HostInfo;
    cpu: StaticCpuInfo;
    disks: StaticDiskInfo[];
    memory: StaticMemoryInfo;
}
