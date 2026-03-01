use std::fmt::Display;

use serde::Serialize;
use sysinfo::{System, Users};

pub struct ProcessCollector {
    pub processes: Vec<ProcessInfo>,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: Option<u32>,
    pub name: String,
    pub exe_path: Option<String>,
    pub cmdline: Vec<String>,
    pub user: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub run_time: String,
}

impl ProcessCollector {
    pub fn get_info(sys: &mut System) -> Self {
        sys.refresh_processes();
        let users = Users::new_with_refreshed_list();

        let mut processes: Vec<ProcessInfo> = Vec::new();

        for (pid, process) in sys.processes() {
            let rt = process.run_time();
            let total_uptime = System::uptime();

            let run_time_str = if rt > total_uptime + 3600 {
                "N/A".to_string()
            } else {
                crate::utils::format_uptime(rt)
            };

            let user = process
                .user_id()
                .and_then(|uid| users.get_user_by_id(uid))
                .map(|u| u.name().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let process_info = ProcessInfo {
                pid: pid.as_u32(),
                ppid: process.parent().map(|p| p.as_u32()),
                name: process.name().to_string(),
                exe_path: process
                    .exe()
                    .map(|path| path.to_string_lossy().into_owned()),
                cmdline: process.cmd().to_vec(),
                user,
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
            "PPID",
            "应用程序",
            // "执行路径",
            // "执行命令",
            "执行用户",
            "CPU使用率",
            "内存使用率",
            "运行时间",
        ]);

        for process in &self.processes {
            table.add_row(vec![
                &process.pid.to_string(),
                &process
                    .ppid
                    .map_or("N/A".to_string(), |ppid| ppid.to_string()),
                &process.name,
                // &process
                //     .exe_path
                //     .as_ref()
                //     .map(|s| s.as_str())
                //     .unwrap_or("N/A")
                //     .to_string(),
                // &process.cmdline.join(" "),
                &process.user,
                &format!("{:.2}%", process.cpu_usage),
                &format!("{:.2}MB", process.memory_usage as f32 / 1024.0 / 1024.0),
                &process.run_time.to_string(),
            ]);
        }

        write!(f, "{}", table)
    }
}
