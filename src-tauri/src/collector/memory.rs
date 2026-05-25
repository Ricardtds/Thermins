use sysinfo::System;

use crate::models::memory::MemoryInfo;

pub fn get_memory_info(sys: &System) -> MemoryInfo {
    let total = sys.total_memory();
    let used = sys.used_memory();

    let usage_percent = (used as f32 / total as f32) * 100.0;

    MemoryInfo {
        total,
        used,
        usage_percent,
    }
}