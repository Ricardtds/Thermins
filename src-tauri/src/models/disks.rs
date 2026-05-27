use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub kind: String,
    pub total_space: u64,
    pub available_space: u64,
    pub filesystem: String,
    pub read_only: bool,
    pub removable: bool,
    pub total_read_bytes: u64,
    pub total_written_bytes: u64,
    pub written_bytes: u64,
    pub read_bytes: u64,
    pub mount_point: String,
}
