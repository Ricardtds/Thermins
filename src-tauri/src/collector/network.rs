use sysinfo::Networks;

use crate::{
    constants::REFRESH_RATE,
    models::network::NetworkInfo,
};

const BITS_PER_BYTE: f64 = 8.0;
const BITS_PER_MEGABIT: f64 = 1_000_000.0;

#[inline]
fn bytes_to_mbps(bytes: u64) -> f64 {
    (bytes as f64 * BITS_PER_BYTE)
        / BITS_PER_MEGABIT
        / REFRESH_RATE as f64
}

#[inline]
fn bytes_to_megabits(bytes: u64) -> f64 {
    (bytes as f64 * BITS_PER_BYTE)
        / BITS_PER_MEGABIT
}

pub fn get_networks_info(networks: &Networks) -> Vec<NetworkInfo> {
    let mut interfaces: Vec<_> = networks
    .iter()
    .filter(|(_, data)| data.total_received() > 100_000)
    .map(|(interface_name, data)| NetworkInfo {
        name: interface_name.to_string(),

        received: bytes_to_mbps(data.received()),
        transmitted: bytes_to_mbps(data.transmitted()),

        total_received: bytes_to_megabits(data.total_received()),
        total_transmitted: bytes_to_megabits(data.total_transmitted()),
    })
    .collect();

    interfaces.sort_by(|a, b| {
        b.total_received.total_cmp(&a.total_received)
    });

    interfaces
}