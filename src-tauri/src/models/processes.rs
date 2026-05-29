use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    pub id: String,
    pub name: String,
    pub working_directory: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub virtual_memory: u64,
    pub cmd: Vec<String>,
    pub parent_id: Option<u32>,
    pub start_time: u64,
    pub run_time: u64,
    pub user_id: Option<String>,
}
