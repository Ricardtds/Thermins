use std::{
    thread,
    time::Duration,
};

use tauri::{AppHandle, Emitter};
use sysinfo::{Components, Disks, Networks, System};
use crate::constants::REFRESH_RATE;

use crate::{
    collector::{
        cpu::get_cpu_info,
        memory::get_dynamic_memory_info,
        disks::get_dynamic_disks_info,
        network::get_networks_info,
        components::get_components_info,
        processes::get_process_info,
        battery::get_battery_info
    },
    models::snapshot::DynamicSystemSnapshot,
};
pub fn start_telemetry(app: AppHandle) {
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut disks = Disks::new_with_refreshed_list();
        let mut networks = Networks::new_with_refreshed_list();
        let mut components = Components::new_with_refreshed_list();
        loop {
            sys.refresh_all();
            disks.refresh(true);
            networks.refresh(true);
            components.refresh(true);
            let snapshot = DynamicSystemSnapshot {
                cpu: get_cpu_info(&sys),
                memory: get_dynamic_memory_info(&sys),
                disks: get_dynamic_disks_info(&disks),
                networks: get_networks_info(&networks),
                components: get_components_info(&components),
                processes: get_process_info(&sys),
                batteries: get_battery_info().ok(),
                uptime: System::uptime(),
                refresh_rate: REFRESH_RATE
            };

            app.emit("system_snapshot", snapshot)
                .unwrap();

            thread::sleep(Duration::from_secs(REFRESH_RATE));
        }
    });
}