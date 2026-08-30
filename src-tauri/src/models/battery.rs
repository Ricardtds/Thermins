use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicBatteryInfo {
    pub id: String,
    pub state: String,
    pub energy: f32,
    pub time_to_empty: Option<u64>,
    pub time_to_full: Option<u64>,
    pub temperature: Option<f32>,
    pub voltage: f32,
    pub energy_rate: f32,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticBatteryInfo {
    pub id: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub technology: String,
    pub cycle_count: Option<u32>,
    pub energy_full: f32,
    pub energy_full_design: f32,
    pub state_of_health: f32,
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
impl From<battery::Battery> for DynamicBatteryInfo {
    fn from(battery: battery::Battery) -> Self {
        Self {
            id: battery.serial_number().unwrap_or("unknown").to_string(),
            state: battery.state().to_string(),
            energy: battery.energy().value,
            time_to_empty: battery.time_to_empty().map(|t| t.value as u64),
            time_to_full: battery.time_to_full().map(|t| t.value as u64),
            temperature: battery.temperature().map(|t| t.value),
            voltage: battery.voltage().value,
            energy_rate: battery.energy_rate().value,
        }
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
impl From<battery::Battery> for StaticBatteryInfo {
    fn from(battery: battery::Battery) -> Self {
        Self {
            id: battery.serial_number().unwrap_or("unknown").to_string(),
            vendor: battery.vendor().map(str::to_owned),
            model: battery.model().map(str::to_owned),
            serial_number: battery.serial_number().map(str::to_owned),
            technology: battery.technology().to_string(),
            cycle_count: battery.cycle_count(),
            energy_full: battery.energy_full().value,
            energy_full_design: battery.energy_full_design().value,
            state_of_health: battery.state_of_health().value,
        }
    }
}
