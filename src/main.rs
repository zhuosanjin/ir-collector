use sysinfo::{Networks, Pid, System, Users};

use ir_collector::collector::{process, system_info, user};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let info = system_info::SystemInfo::get_info(&mut sys);
    let process = process::ProcessCollector::get_info(&mut sys);
    println!("{}", info);
    println!("{}", process);
    let users = user::UsersInfo::get_info();
    println!("{}", users);

    // let networks = Networks::new_with_refreshed_list();
    // println!("=> networks:");
    // for (interface_name, data) in &networks {
    //     println!(
    //         "{interface_name}: {} MB (down) / {} MB (up)",
    //         data.total_received() / 1024 / 1024,
    //         data.total_transmitted() / 1024 / 1024,
    //     );
    // }
}
