use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::constants::REFRESH_RATE;
use sysinfo::{Components, Disks, Networks, System};
use tauri::{AppHandle, Emitter};

use crate::{
    collector::{
        battery::get_battery_info, components::get_components_info, cpu::get_cpu_info,
        disks::get_dynamic_disks_info, memory::get_dynamic_memory_info, network::get_networks_info,
        processes::get_process_info,
    },
    models::snapshot::DynamicSystemSnapshot,
};
#[derive(Clone)]
pub struct TelemetryControl {
    active: Arc<(Mutex<bool>, Condvar)>,
}

impl Default for TelemetryControl {
    fn default() -> Self {
        Self {
            active: Arc::new((Mutex::new(true), Condvar::new())),
        }
    }
}

impl TelemetryControl {
    pub fn set_active(&self, active: bool) {
        let (state, resumed) = &*self.active;
        *state.lock().expect("telemetry lifecycle lock poisoned") = active;

        if active {
            resumed.notify_all();
        }
    }

    fn wait_until_active(&self) {
        let (state, resumed) = &*self.active;
        let active = state.lock().expect("telemetry lifecycle lock poisoned");
        drop(
            resumed
                .wait_while(active, |active| !*active)
                .expect("telemetry lifecycle lock poisoned"),
        );
    }
}

pub fn start_telemetry(app: AppHandle, control: TelemetryControl) {
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut disks = Disks::new_with_refreshed_list();
        let mut networks = Networks::new_with_refreshed_list();
        let mut components = Components::new_with_refreshed_list();
        let mut last_refresh: Option<Instant> = None;

        loop {
            control.wait_until_active();

            let now = Instant::now();
            let elapsed_seconds = last_refresh
                .map(|last| now.duration_since(last).as_secs_f64())
                .unwrap_or(REFRESH_RATE as f64)
                .max(0.001);
            last_refresh = Some(now);

            sys.refresh_all();
            disks.refresh(true);
            networks.refresh(true);
            components.refresh(true);
            let snapshot = DynamicSystemSnapshot {
                cpu: get_cpu_info(&sys),
                memory: get_dynamic_memory_info(&sys),
                disks: get_dynamic_disks_info(&disks, elapsed_seconds),
                networks: get_networks_info(&networks, elapsed_seconds),
                components: get_components_info(&components),
                processes: get_process_info(&sys),
                batteries: get_battery_info(),
                uptime: System::uptime(),
                refresh_rate: REFRESH_RATE,
            };

            // A closed or not-yet-ready webview must not crash the collector
            // thread. The next sample can still be delivered after recovery.
            let _ = app.emit("system_snapshot", snapshot);

            thread::sleep(Duration::from_secs(REFRESH_RATE));
        }
    });
}
