use std::fmt::Display;

use sysinfo::System;

pub struct ProcessCollector {
    pub processes: Vec<ProcessInfo>,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub run_time: String,
}

impl ProcessCollector {
    pub fn get_info(sys: &mut System) -> Self {
        let mut processes: Vec<ProcessInfo> = Vec::new();

        for (pid, process) in sys.processes() {
            let rt = process.run_time();
            let total_uptime = System::uptime();

            let run_time_str = if rt > total_uptime + 3600 {
                "N/A".to_string()
            } else {
                crate::utils::format_uptime(rt)
            };

            let process_info = ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                run_time: run_time_str,
            };

            processes.push(process_info);
        }

        Self { processes }
    }
}

impl Display for ProcessCollector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = comfy_table::Table::new();
        table.set_header(vec![
            "PID",
            "应用程序",
            "CPU使用率",
            "内存使用率",
            "运行时间",
        ]);

        for process in &self.processes {
            table.add_row(vec![
                &process.pid.to_string(),
                &process.name,
                &format!("{:.2}%", process.cpu_usage),
                &format!("{:.2}MB", process.memory_usage as f32 / 1024.0 / 1024.0),
                &process.run_time.to_string(),
            ]);
        }

        write!(f, "{}", table)
    }
}
