use sysinfo::{System};

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, process) in sys.processes() {
        println!("PID: {}, Name: {}", pid, process.name());
    }
}