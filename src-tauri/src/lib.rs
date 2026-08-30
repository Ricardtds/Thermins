mod collector;
pub mod constants;
mod models;
mod telemetry;
mod terminal;

use telemetry::emitter::{start_telemetry, TelemetryControl};
use telemetry::static_snapshot::get_static_info;
use terminal::{get_terminal_capabilities, run_terminal_command};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let telemetry_control = TelemetryControl::default();
    let collector_control = telemetry_control.clone();
    let lifecycle_control = telemetry_control.clone();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_static_info,
            get_terminal_capabilities,
            run_terminal_command
        ])
        .setup(move |app| {
            start_telemetry(app.handle().clone(), collector_control.clone());
            Ok(())
        })
        .on_window_event(move |_window, event| {
            update_telemetry_lifecycle(event, &lifecycle_control);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn update_telemetry_lifecycle(event: &tauri::WindowEvent, control: &TelemetryControl) {
    #[cfg(mobile)]
    match event {
        tauri::WindowEvent::Suspended => control.set_active(false),
        tauri::WindowEvent::Resumed => control.set_active(true),
        _ => {}
    }

    #[cfg(not(mobile))]
    let _ = (event, control);
}
