use sysinfo::System;

use crate::models::system::SystemInfo;

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        name: System::name().unwrap_or("Not Identified".into()),
        kernel_version: System::kernel_version().unwrap_or("Unknown".into()),
        os_version: System::os_version().unwrap_or("Unknown".into()),
        host_name: System::host_name().unwrap_or("Unknown".into()),
    }
}