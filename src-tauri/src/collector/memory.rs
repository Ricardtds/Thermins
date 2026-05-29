use sysinfo::System;

use crate::models::memory::{DynamicMemoryInfo, StaticMemoryInfo};

pub fn get_static_memory_info(sys: &System) -> StaticMemoryInfo {
    StaticMemoryInfo::from(sys)
}

pub fn get_dynamic_memory_info(sys: &System) -> DynamicMemoryInfo {
    DynamicMemoryInfo::from(sys)
}