use sysinfo::{System, SystemExt};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Starting rust-systems-tracker...");
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        println!("=> Total Memory: {} KB", sys.total_memory());
        println!("=> Used Memory: {} KB", sys.used_memory());
        println!("=> Processes running: {}", sys.processes().len());
        
        sleep(Duration::from_secs(5)).await;
    }
}
