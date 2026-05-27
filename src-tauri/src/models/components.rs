use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ComponentInfo {
    pub label: String,
    pub temperature: f32,
    pub critical: f32,
    pub max_temperature: f32,
    pub id: String,
}
