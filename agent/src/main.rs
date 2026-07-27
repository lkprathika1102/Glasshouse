use sysinfo::{System, ProcessExt, SystemExt};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;
    
    let sockets = get_sockets_info(af_flags, proto_flags).unwrap();

    for socket in sockets {
        if let ProtocolSocketInfo::Tcp(tcp_info) = socket.protocol_socket_info {
            if let Some(pid) = tcp_info.associated_pid {
                let remote_addr = tcp_info.remote_address;
                
                if let Some(process) = sys.process(sysinfo::Pid::from(pid as usize)) {
                    println!(
                        "Process: {} | PID: {} | Remote: {}:{}", 
                        process.name(), 
                        pid, 
                        remote_addr.ip(), 
                        remote_addr.port()
                    );
                }
            }
        }
    }
}