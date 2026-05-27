use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub usage_percent: f32,
    pub total_swap: u64,
    pub used_swap: u64,
    pub free_memory: u64,
}