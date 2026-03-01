use std::fmt::Display;

use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, get_sockets_info};

pub struct NetworkCollector {
    pub network_connects: Vec<NetworkInfo>,
}

pub struct NetworkInfo {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub status: String,
    pub pid: Vec<u32>,
}

impl NetworkCollector {
    pub fn get_info() -> Result<Self, Box<dyn std::error::Error>> {
        let mut network_connects: Vec<NetworkInfo> = Vec::new();

        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets_info = get_sockets_info(af_flags, proto_flags)?;

        for si in sockets_info {
            match si.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp_si) => {
                    let local_address = format!("{}:{}", tcp_si.local_addr, tcp_si.local_port);
                    let remote_address = format!("{}:{}", tcp_si.remote_addr, tcp_si.remote_port);
                    let status = format!("{:?}", tcp_si.state);

                    let network_info = NetworkInfo {
                        protocol: "tcp".to_string(),
                        local_address,
                        remote_address,
                        status,
                        pid: si.associated_pids.to_vec(),
                    };

                    network_connects.push(network_info);
                }
                ProtocolSocketInfo::Udp(udp_si) => {
                    let local_address = format!("{}:{}", udp_si.local_addr, udp_si.local_port);
                    let remote_address = "*:*".to_string();
                    let status = "N/A".to_string();

                    let network_info = NetworkInfo {
                        protocol: "tcp".to_string(),
                        local_address,
                        remote_address,
                        status,
                        pid: si.associated_pids.to_vec(),
                    };

                    network_connects.push(network_info);
                }
            }
        }

        Ok(Self { network_connects })
    }
}

impl Display for NetworkCollector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = comfy_table::Table::new();
        table.set_header(vec![
            "Protocol",
            "Local Address",
            "Remote Address",
            "Status",
            "PID",
        ]);

        for network_info in &self.network_connects {
            let pids_str = network_info
                .pid
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            table.add_row(vec![
                &network_info.protocol,
                &network_info.local_address,
                &network_info.remote_address,
                &network_info.status,
                &pids_str,
            ]);
        }
        write!(f, "{}", table)
    }
}
