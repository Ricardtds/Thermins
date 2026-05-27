use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ProcessInfo {
    pub id: String,
    pub name: String,
    pub working_directory: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}
