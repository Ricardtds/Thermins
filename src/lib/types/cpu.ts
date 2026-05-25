export interface CpuCoreInfo {
    name: string;
    usage: number;
    frequency: number;
}

export interface CpuInfo {
    usage: number;
    cores: CpuCoreInfo[];
    info: CpuStaticInfo;
}

export interface CpuStaticInfo {
    brand: string;
    vendor_id: string;
    physical_cores: number;
}