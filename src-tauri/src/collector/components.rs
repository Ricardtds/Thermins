use sysinfo::Components;

use crate::models::{components::ComponentInfo};

pub fn get_components_info(components: &Components) -> Vec<ComponentInfo> {
    components
        .iter()
        .map(|component| ComponentInfo {
            id: component.id().unwrap_or_default().to_string(),
            label: component.label().to_string(),
            temperature: component.temperature().unwrap_or(0.0),
            critical: component.critical().unwrap_or(0.0),
            max_temperature: component.max().unwrap_or(0.0),
        })
        .collect()
}