use battery::Manager;
use crate::models::battery::{DynamicBatteryInfo, StaticBatteryInfo};

pub fn get_battery_info() -> Result<Vec<DynamicBatteryInfo>, battery::Error> {
    let manager = Manager::new()?;

    manager
    .batteries()?
    .map(|battery| {
        let mut battery = battery?;
        let _ = battery.refresh();

        Ok(DynamicBatteryInfo::from(battery))
    })
    .collect()
}

pub fn get_static_battery_info() -> Result<Vec<StaticBatteryInfo>, battery::Error> {
    let manager = Manager::new()?;

    manager
        .batteries()?
        .map(|battery| {
            let battery = battery?;

            Ok(StaticBatteryInfo::from(battery))
        })
        .collect()
}