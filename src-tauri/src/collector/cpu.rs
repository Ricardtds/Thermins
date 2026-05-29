use sysinfo::System;

use crate::models::cpu::{DynamicCpuCoreInfo, DynamicCpuInfo, StaticCpuInfo};

pub fn get_cpu_info(sys: &System) -> DynamicCpuInfo {
    let cores = sys
        .cpus()
        .iter()
        .map(|cpu| DynamicCpuCoreInfo {
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        })
        .collect();

    DynamicCpuInfo {
        usage: sys.global_cpu_usage(),
        cores,
    }
}

pub fn get_static_cpu_info(sys: &System) -> StaticCpuInfo {
    StaticCpuInfo {
        brand: sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default(),

        vendor_id: sys
            .cpus()
            .first()
            .map(|c| c.vendor_id().to_string())
            .unwrap_or_default(),

        physical_cores: System::physical_core_count().unwrap_or(0),
    }
}