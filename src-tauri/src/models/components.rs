use serde::Serialize;
use sysinfo::Component;
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComponentInfo {
    pub label: String,
    pub temperature: f32,
    pub critical: f32,
    pub max_temperature: f32,
    pub id: String,
}

impl From<&Component> for ComponentInfo {
    fn from(component: &Component) -> Self {
        Self {
            id: component.id().unwrap_or_default().to_string(),
            label: component.label().to_string(),
            temperature: component.temperature().unwrap_or(0.0),
            critical: component.critical().unwrap_or(0.0),
            max_temperature: component.max().unwrap_or(0.0),
        }
    }
}