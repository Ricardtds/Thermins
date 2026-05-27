use sysinfo::{DiskKind, Disks};

use crate::models::disks::DiskInfo;

pub fn get_disks_info(disks: &Disks) -> Vec<DiskInfo> {
    disks.iter().map(|disk| DiskInfo {
        kind: match disk.kind() {
            DiskKind::HDD => "HDD".to_string(),
            DiskKind::SSD => "SSD".to_string(),
            DiskKind::Unknown(_) => "Unknown".to_string(),
        },
        name: disk.name().to_string_lossy().into_owned(),
        total_space: disk.total_space(),
        available_space: disk.available_space(),
        filesystem: disk.file_system().to_string_lossy().into_owned(),
        read_only: disk.is_read_only(),
        removable: disk.is_removable(),
        total_read_bytes: disk.usage().total_read_bytes,
        total_written_bytes: disk.usage().total_written_bytes,
        written_bytes: disk.usage().written_bytes,
        read_bytes: disk.usage().read_bytes,
        mount_point: disk.mount_point().to_string_lossy().into_owned(),
    }).collect()
}