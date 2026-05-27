use sysinfo::Networks;
use crate::models::network::NetworkInfo;

pub fn get_networks_info(networks: &Networks) -> Vec<NetworkInfo> {
    networks.iter().map(|(interface_name, data)| NetworkInfo {
        name: interface_name.to_string(),
        transmitted: data.transmitted(),
        received: data.received(),
    }).collect()

}