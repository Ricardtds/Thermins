export interface DiskInfo {
    available_space: number;
    filesystem: string;
    kind: string;
    mount_point: string;
    name: string;
    read_bytes: number;
    read_only: boolean;
    removable: boolean;
    total_read_bytes: number;
    total_space: number;
    total_written_bytes: number;
    written_bytes: number;
}