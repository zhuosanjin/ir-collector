use comfy_table::Table;
use std::fmt::Display;
use sysinfo::System;

pub struct SystemInfo {
    hostname: String,
    os_version: String,
    kernel_version: String,
    uptime: String,
    current_time: String,
}

impl SystemInfo {
    pub fn get_info() -> Self {
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let uptime = System::uptime();
        let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Self {
            hostname,
            os_version,
            kernel_version,
            uptime: crate::utils::format_uptime(uptime),
            current_time,
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
                "内核版本",
                "运行时间",
                "当前时间",
            ])
            .add_row(vec![
                &self.hostname,
                &self.os_version,
                &self.kernel_version,
                &self.uptime.to_string(),
                &self.current_time,
            ]);

        write!(f, "{}", table)
    }
}
