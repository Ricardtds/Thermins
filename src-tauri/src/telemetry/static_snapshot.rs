use sysinfo::{System, Disks};

use crate::{collector::{battery, cpu, disks, system, memory}, models::snapshot::StaticSystemSnapshot};

#[tauri::command]
pub fn get_static_info() -> StaticSystemSnapshot {
    let sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();

    StaticSystemSnapshot {
        batteries: battery::get_static_battery_info().ok(),
        cpu: cpu::get_static_cpu_info(&sys),
        disks: disks::get_static_disks_info(&disks),
        host: system::get_system_info(),
        memory: memory::get_static_memory_info(&sys)
    }
}