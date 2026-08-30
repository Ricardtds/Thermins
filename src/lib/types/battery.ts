export interface DynamicBatteryInfo {
    id: string;
    state: string;
    energy: number;
    energyRate: number;
    timeToEmpty: number | null;
    timeToFull: number | null;
    temperature: number | null;
    voltage: number;
}

export interface StaticBatteryInfo {
    id: string;
    vendor: string | null;
    model: string | null;
    serialNumber: string | null;
    technology: string;
    cycleCount: number | null;
    energyFull: number;
    energyFullDesign: number;
    stateOfHealth: number;
}
