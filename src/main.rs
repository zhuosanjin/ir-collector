use sysinfo::System;

use ir_collector::collector::{network, process, system_info, user};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let process = process::ProcessCollector::get_info(&mut sys);
    println!("{}", process);

    let network_connections = network::NetworkCollector::get_info()?;
    println!("{}", network_connections);

    let info = system_info::SystemInfo::get_info();
    println!("{}", info);

    let users = user::UsersInfo::get_info();
    println!("{}", users);

    Ok(())
}
