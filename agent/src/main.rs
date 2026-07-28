use sysinfo::{System, Pid};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::net::IpAddr;
use tokio::time::sleep;
use maxminddb::Reader;

#[derive(Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct City {
    location: Option<Location>,
}

#[derive(Serialize)]
struct ConnectionEvent {
    process_name: String,
    pid: u32,
    remote_ip: String,
    remote_port: u16,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    let reader = Reader::open_readfile("GeoLite2-City.mmdb").ok();
    
    loop {
        sys.refresh_all();

        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP;
        
        if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
            for socket in sockets {
                if let ProtocolSocketInfo::Tcp(tcp_info) = socket.protocol_socket_info {
                    if let Some(&pid) = socket.associated_pids.first() {
                        if let Some(process) = sys.process(Pid::from(pid as usize)) {
                            let remote_addr_str = tcp_info.remote_addr.to_string();
                            let mut lat = None;
                            let mut lon = None;

                            if let (Some(ref r), Ok(ip)) = (&reader, remote_addr_str.parse::<IpAddr>()) {
                                if let Ok(city) = r.lookup::<City>(ip) {
                                    if let Some(loc) = city.location {
                                        lat = Some(loc.latitude);
                                        lon = Some(loc.longitude);
                                    }
                                }
                            }

                            let event = ConnectionEvent {
                                process_name: process.name().to_string(),
                                pid,
                                remote_ip: remote_addr_str,
                                remote_port: tcp_info.remote_port,
                                latitude: lat,
                                longitude: lon,
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