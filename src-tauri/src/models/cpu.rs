use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicCpuCoreInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicCpuInfo {
    pub usage: f32,
    pub cores: Vec<DynamicCpuCoreInfo>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StaticCpuInfo {
    pub brand: String,
    pub vendor_id: String,
    pub physical_cores: usize,
}