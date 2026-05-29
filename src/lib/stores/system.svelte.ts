import type { DynamicSystemSnapshot, StaticSystemSnapshot }
    from "$lib/types/system";

export const dynamicSystemState =
    $state<DynamicSystemSnapshot>({
        batteries: [],
        energyRate: 0,
        cpu: {

            cores: [],
            usage: 0
        },
        memory: {
            freeMemory: 0,
            usagePercent: 0,
            used: 0,
            usedSwap: 0
        },

        disks: [],

        components: [],

        processes: [],

        networks: [],

        uptime: 0,

        refreshRate: 0,
    });

export const staticSystemState =
    $state<StaticSystemSnapshot>({
        batteries: [],
        host: {
            hostName: "",
            kernelVersion: "",
            osVersion: "",
            name: ""
        },
        cpu: {
            brand: "",
            physicalCores: 0,
            vendorId: "",
        },
        memory: {
            total: 0,
            totalSwap: 0
        },

        disks: []

    });