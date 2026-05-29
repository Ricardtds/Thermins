use serde::Serialize;
use sysinfo::System;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticMemoryInfo {
    pub total: u64,
    pub total_swap: u64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicMemoryInfo {
    pub used: u64,
    pub free_memory: u64,
    pub used_swap: u64,
    pub usage_percent: f32,
}

impl From<&System> for StaticMemoryInfo {
    fn from(sys: &System) -> Self {
        Self {
            total: sys.total_memory(),
            total_swap: sys.total_swap(),
        }
    }
}

impl From<&System> for DynamicMemoryInfo {
    fn from(sys: &System) -> Self {
        let total = sys.total_memory();
        let used = sys.used_memory();

        Self {
            used,
            free_memory: sys.free_memory(),
            used_swap: sys.used_swap(),
            usage_percent: if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            },
        }
    }
}