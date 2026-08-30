use crate::models::processes::ProcessInfo;
use sysinfo::System;

pub fn get_process_info(sys: &System) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .filter(|(_, process)| process.cpu_usage() > 0.1)
        .map(|(pid, process)| ProcessInfo {
            id: pid.to_string(),
            parent_id: process.parent().map(|pid| pid.as_u32()),
            name: process.name().to_string_lossy().into_owned(),
            user_id: process.user_id().map(|uid| uid.to_string()),
            working_directory: process
                .cwd()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default(),

            cpu_usage: process.cpu_usage(),
            start_time: process.start_time(),
            run_time: process.run_time(),
            memory_usage: process.memory(),
            virtual_memory: process.virtual_memory(),

            cmd: process
                .cmd()
                .iter()
                .map(|arg| arg.to_string_lossy().to_string())
                .collect(),
        })
        .collect();

    processes.sort_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage));

    processes.truncate(200);

    processes
}
