use sysinfo::System;

use crate::models::cpu::{CpuCoreInfo, CpuInfo, CpuStaticInfo};

pub fn get_cpu_info(sys: &System) -> CpuInfo {
    let cores = sys
        .cpus()
        .iter()
        .map(|cpu| CpuCoreInfo {
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        })
        .collect();

    CpuInfo {
        usage: sys.global_cpu_usage(),
        cores,
        info: CpuStaticInfo {
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
        },
    }
}
