use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct CpuCoreInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Serialize, Clone)]
pub struct CpuInfo {
    pub usage: f32,
    pub cores: Vec<CpuCoreInfo>,
    pub info: CpuStaticInfo,
}

#[derive(Serialize, Clone)]
pub struct CpuStaticInfo {
    pub brand: String,
    pub vendor_id: String,
    pub physical_cores: usize,
}