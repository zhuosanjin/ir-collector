use comfy_table::Table;
use std::fmt::Display;
use sysinfo::System;

pub struct SystemInfo {
    hostname: String,
    os_version: String,
    uptime: String,
    current_time: String,
    cpu_usage: f32,
    memory_usage: f32,
}

impl SystemInfo {
    pub fn get_info(sys: &mut System) -> Self {
        let mut cpu_usage_cal = 0.0;
        for cpu in sys.cpus() {
            cpu_usage_cal += cpu.cpu_usage();
        }

        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let uptime = System::uptime();
        let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let cpu_usage = cpu_usage_cal / sys.cpus().len() as f32;
        let memory_usage = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;

        Self {
            hostname,
            os_version,
            uptime: crate::utils::format_uptime(uptime),
            current_time,
            cpu_usage,
            memory_usage,
        }
    }
}

impl Display for SystemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table
            .set_header(vec![
                "主机名称",
                "系统版本",
                "运行时间",
                "当前时间",
                "CPU使用率",
                "内存使用率",
            ])
            .add_row(vec![
                &self.hostname,
                &self.os_version,
                &self.uptime.to_string(),
                &self.current_time,
                &format!("{:.2}%", &self.cpu_usage),
                &format!("{:.2}%", &self.memory_usage),
            ]);

        write!(f, "{}", table)
    }
}
