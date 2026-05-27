use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SystemInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}