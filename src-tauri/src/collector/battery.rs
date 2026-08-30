use crate::models::battery::{DynamicBatteryInfo, StaticBatteryInfo};

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use battery::Manager;

pub fn get_battery_info() -> Option<Vec<DynamicBatteryInfo>> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = Manager::new().ok()?;

        return manager
            .batteries()
            .ok()?
            .map(|battery| {
                let mut battery = battery?;
                let _ = battery.refresh();

                Ok(DynamicBatteryInfo::from(battery))
            })
            .collect::<Result<Vec<_>, battery::Error>>()
            .ok();
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    None
}

pub fn get_static_battery_info() -> Option<Vec<StaticBatteryInfo>> {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let manager = Manager::new().ok()?;

        return manager
            .batteries()
            .ok()?
            .map(|battery| {
                let battery = battery?;

                Ok(StaticBatteryInfo::from(battery))
            })
            .collect::<Result<Vec<_>, battery::Error>>()
            .ok();
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    None
}
