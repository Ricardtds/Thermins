use sysinfo::Networks;

use crate::models::network::NetworkInfo;

const BITS_PER_BYTE: f64 = 8.0;
const BITS_PER_MEGABIT: f64 = 1_000_000.0;

#[inline]
fn bytes_to_mbps(bytes: u64, elapsed_seconds: f64) -> f64 {
    (bytes as f64 * BITS_PER_BYTE) / BITS_PER_MEGABIT / elapsed_seconds
}

#[inline]
fn bytes_to_megabits(bytes: u64) -> f64 {
    (bytes as f64 * BITS_PER_BYTE) / BITS_PER_MEGABIT
}

pub fn get_networks_info(networks: &Networks, elapsed_seconds: f64) -> Vec<NetworkInfo> {
    let mut interfaces: Vec<_> = networks
        .iter()
        .filter(|(_, data)| data.total_received() > 100_000)
        .map(|(interface_name, data)| NetworkInfo {
            name: interface_name.to_string(),

            received: bytes_to_mbps(data.received(), elapsed_seconds),
            transmitted: bytes_to_mbps(data.transmitted(), elapsed_seconds),

            total_received: bytes_to_megabits(data.total_received()),
            total_transmitted: bytes_to_megabits(data.total_transmitted()),
        })
        .collect();

    interfaces.sort_by(|a, b| b.total_received.total_cmp(&a.total_received));

    interfaces
}
