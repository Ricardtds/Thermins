use serde::Serialize;
use sysinfo::{Disk, DiskKind};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticDiskInfo {
    pub name: String,
    pub kind: String,
    pub filesystem: String,
    pub read_only: bool,
    pub removable: bool,
    pub mount_point: String,
    pub total_space: u64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicDiskInfo {
    pub name: String,
    pub kind: String,
    pub filesystem: String,
    pub read_only: bool,
    pub removable: bool,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub total_read_bytes: u64,
    pub total_written_bytes: u64,
    pub read_bytes: u64,
    pub written_bytes: u64,
}

impl From<&Disk> for StaticDiskInfo {
    fn from(disk: &Disk) -> Self {
        Self {
            name: disk.name().to_string_lossy().into_owned(),
            kind: match disk.kind() {
                DiskKind::HDD => "HDD".to_string(),
                DiskKind::SSD => "SSD".to_string(),
                DiskKind::Unknown(_) => "Unknown".to_string(),
            },
            total_space: disk.total_space(),
            filesystem: disk.file_system().to_string_lossy().into_owned(),
            read_only: disk.is_read_only(),
            removable: disk.is_removable(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
        }
    }
}

impl DynamicDiskInfo {
    pub fn from_disk(disk: &Disk, elapsed_seconds: f64) -> Self {
        let usage = disk.usage();
        let bytes_per_second = |bytes: u64| (bytes as f64 / elapsed_seconds).round() as u64;

        Self {
            name: disk.name().to_string_lossy().into_owned(),
            kind: match disk.kind() {
                DiskKind::HDD => "HDD".to_string(),
                DiskKind::SSD => "SSD".to_string(),
                DiskKind::Unknown(_) => "Unknown".to_string(),
            },
            filesystem: disk.file_system().to_string_lossy().into_owned(),
            read_only: disk.is_read_only(),
            removable: disk.is_removable(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            total_read_bytes: usage.total_read_bytes,
            total_written_bytes: usage.total_written_bytes,
            written_bytes: bytes_per_second(usage.written_bytes),
            read_bytes: bytes_per_second(usage.read_bytes),
        }
    }
}
