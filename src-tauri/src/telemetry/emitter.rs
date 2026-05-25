use std::{
    thread,
    time::Duration,
};

use tauri::{AppHandle, Emitter};
use sysinfo::System;

use crate::{
    collector::{
        cpu::get_cpu_info,
        memory::get_memory_info,
    },
    models::snapshot::SystemSnapshot,
};

pub fn start_telemetry(app: AppHandle) {
    thread::spawn(move || {
        let mut sys = System::new_all();

        loop {
            sys.refresh_all();

            let snapshot = SystemSnapshot {
                cpu: get_cpu_info(&sys),
                memory: get_memory_info(&sys),
                uptime: System::uptime(),
            };

            app.emit("system_snapshot", snapshot)
                .unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });
}