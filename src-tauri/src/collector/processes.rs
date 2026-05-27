use sysinfo::System;

use crate::models::processes::ProcessInfo;

pub fn get_process_info(
    sys: &System
) -> Vec<ProcessInfo> {

    let mut processes: Vec<ProcessInfo> =
        sys.processes()
            .iter()
            .filter(|(_, process)| {
                process.cpu_usage() > 0.1
            })
            .map(|(pid, process)| ProcessInfo {
                id: pid.to_string(),

                name: process.name()
                    .to_string_lossy()
                    .into_owned(),

                working_directory: process.cwd()
                    .map(|p| {
                        p.to_string_lossy()
                            .into_owned()
                    })
                    .unwrap_or_default(),

                cpu_usage: process.cpu_usage(),

                memory_usage: process.memory(),
            })
            .collect();

    processes.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap()
    });

    processes.truncate(200);

    processes
}