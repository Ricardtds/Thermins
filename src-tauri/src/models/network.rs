use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NetworkInfo {
    pub name: String,
    pub transmitted: u64,
    pub received: u64,
}