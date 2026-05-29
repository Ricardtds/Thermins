export interface StaticDiskInfo {
    name: string,
    kind: string,
    filesystem: string,
    readOnly: boolean,
    removable: boolean,
    mountPoint: string,
    totalSpace: number,
}

export interface DynamicDiskInfo {
    name: string,
    availableSpace: number,
    totalReadBytes: number,
    totalWrittenBytes: number,
    readBytes: number,
    writtenBytes: number,
}