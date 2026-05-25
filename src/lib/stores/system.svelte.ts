import type { SystemSnapshot }
    from "$lib/types/system";

export const systemState =
    $state<SystemSnapshot>({
        cpu: {
            usage: 0,

            cores: [],

            info: {
                brand: "",
                vendor_id: "",
                physical_cores: 0,
            },
        },

        memory: {
            total: 0,
            used: 0,
            usage_percent: 0,
        },
        uptime: 0,
    });