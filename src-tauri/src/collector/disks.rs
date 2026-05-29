use sysinfo::{Disks};

use crate::models::disks::{DynamicDiskInfo, StaticDiskInfo};

pub fn get_static_disks_info(disks: &Disks) -> Vec<StaticDiskInfo> {
    disks.iter().filter(|a| {a.available_space() > 1_000_000_000}).map(StaticDiskInfo::from).collect()
}

pub fn get_dynamic_disks_info(disks: &Disks) -> Vec<DynamicDiskInfo> {
    disks.iter().filter(|a| {a.available_space() > 1_000_000_000}).map(DynamicDiskInfo::from).collect()
}