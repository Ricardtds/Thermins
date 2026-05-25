use sysinfo::System;

pub fn get_uptime() -> u64 {
    System::uptime()
}
