mod collector;
mod models;
mod telemetry;
pub mod constants;

use telemetry::emitter::start_telemetry;
use telemetry::static_snapshot::get_static_info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_static_info
        ])
        .setup(|app| {
            start_telemetry(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}