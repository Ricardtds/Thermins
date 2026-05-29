use sysinfo::System;

use crate::models::host::HostInfo;

#[tauri::command]
pub fn get_system_info() -> HostInfo {
    HostInfo {
        name: System::name().unwrap_or("Not Identified".into()),
        kernel_version: System::kernel_version().unwrap_or("Unknown".into()),
        os_version: System::os_version().unwrap_or("Unknown".into()),
        host_name: System::host_name().unwrap_or("Unknown".into()),
    }
}