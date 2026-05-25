use serde::Serialize;

use crate::models::cpu::CpuInfo;
use crate::models::memory::MemoryInfo;

#[derive(Serialize, Clone)]
pub struct SystemSnapshot {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub uptime: u64,
}