use serde::Serialize;

use crate::models::battery::{DynamicBatteryInfo, StaticBatteryInfo};
use crate::models::components::ComponentInfo;
use crate::models::cpu::{DynamicCpuInfo, StaticCpuInfo};
use crate::models::disks::{DynamicDiskInfo, StaticDiskInfo};
use crate::models::host::HostInfo;
use crate::models::memory::{DynamicMemoryInfo, StaticMemoryInfo};
use crate::models::network::NetworkInfo;
use crate::models::processes::ProcessInfo;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSystemSnapshot {
    pub cpu: DynamicCpuInfo,
    pub memory: DynamicMemoryInfo,
    pub uptime: u64,
    pub disks: Vec<DynamicDiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub components: Vec<ComponentInfo>,
    pub processes: Vec<ProcessInfo>,
    pub batteries: Option<Vec<DynamicBatteryInfo>>,
    pub refresh_rate: u64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticSystemSnapshot {
    pub batteries: Option<Vec<StaticBatteryInfo>>,
    pub cpu: StaticCpuInfo,
    pub disks: Vec<StaticDiskInfo>,
    pub host: HostInfo,
    pub memory: StaticMemoryInfo,
}
