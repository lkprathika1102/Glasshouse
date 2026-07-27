use sysinfo::{System, Pid};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
struct ConnectionEvent {
    process_name: String,
    pid: u32,
    remote_ip: String,
    remote_port: u16,
}

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    
    loop {
        sys.refresh_all();

        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP;
        
        if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
            for socket in sockets {
                if let ProtocolSocketInfo::Tcp(tcp_info) = socket.protocol_socket_info {
                    if let Some(pid) = tcp_info.associated_pid {
                        let remote_addr = tcp_info.remote_address;
                        
                        if let Some(process) = sys.process(Pid::from(pid as usize)) {
                            let event = ConnectionEvent {
                                process_name: process.name().to_string(),
                                pid,
                                remote_ip: remote_addr.ip().to_string(),
                                remote_port: remote_addr.port(),
                            };
                            
                            if let Ok(json) = serde_json::to_string(&event) {
                                println!("{}", json);
                            }
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}