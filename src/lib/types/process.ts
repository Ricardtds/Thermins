export interface ProcessInfo {
    id: string;
    name: string;
    workingDirectory: string;
    cpuUsage: number;
    memoryUsage: number;
    virtualMemory: number;
    cmd: string[];
    parentId: number | null;
    startTime: number;
    runTime: number;
    userId: string | null;
}
