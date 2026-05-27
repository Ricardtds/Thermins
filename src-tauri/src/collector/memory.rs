use sysinfo::System;

use crate::models::memory::MemoryInfo;

pub fn get_memory_info(sys: &System) -> MemoryInfo {
    let total = sys.total_memory();
    let used = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let free_memory = sys.free_memory();

    let usage_percent = (used as f32 / total as f32) * 100.0;

    MemoryInfo {
        total,
        used,
        usage_percent,
        total_swap,
        used_swap,
        free_memory,
    }
}