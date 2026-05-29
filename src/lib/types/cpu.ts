interface DynamicCpuCoreInfo {
    name: string;
    usage: number;
    frequency: number;
}

export interface DynamicCpuInfo {
    usage: number;
    cores: DynamicCpuCoreInfo[];
}

export interface StaticCpuInfo {
    brand: string;
    vendorId: string;
    physicalCores: number;
}