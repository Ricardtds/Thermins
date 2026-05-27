use serde::Serialize;

use crate::models::components::ComponentInfo;
use crate::models::cpu::CpuInfo;
use crate::models::memory::MemoryInfo;
use crate::models::disks::DiskInfo;
use crate::models::network::NetworkInfo;
use crate::models::processes::ProcessInfo;


#[derive(Serialize, Clone)]
pub struct SystemSnapshot {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub uptime: u64,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub components: Vec<ComponentInfo>,
    pub processes: Vec<ProcessInfo>,
}