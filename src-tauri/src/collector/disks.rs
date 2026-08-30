use sysinfo::Disks;

use crate::models::disks::{DynamicDiskInfo, StaticDiskInfo};

pub fn get_static_disks_info(disks: &Disks) -> Vec<StaticDiskInfo> {
    disks.iter().map(StaticDiskInfo::from).collect()
}

pub fn get_dynamic_disks_info(disks: &Disks, elapsed_seconds: f64) -> Vec<DynamicDiskInfo> {
    disks
        .iter()
        .map(|disk| DynamicDiskInfo::from_disk(disk, elapsed_seconds))
        .collect()
}
