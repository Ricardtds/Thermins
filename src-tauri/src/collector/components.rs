use sysinfo::Components;

use crate::models::{components::ComponentInfo};

pub fn get_components_info(components: &Components) -> Vec<ComponentInfo> {
    components.iter().map(ComponentInfo::from).collect()
}