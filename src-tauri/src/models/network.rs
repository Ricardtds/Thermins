use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    pub name: String,
    pub transmitted: f64,
    pub total_transmitted: f64,
    pub received: f64,
    pub total_received: f64,
}