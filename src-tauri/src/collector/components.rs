use sysinfo::Components;

use crate::models::components::ComponentInfo;

pub fn get_components_info(components: &Components) -> Vec<ComponentInfo> {
    components
        .iter()
        .filter(|component| {
            component
                .temperature()
                .is_some_and(|temperature| temperature.is_finite())
        })
        .map(ComponentInfo::from)
        .collect()
}
