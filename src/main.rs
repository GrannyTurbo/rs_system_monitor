use std::io;
use tui::{backend::CrosstermBackend, Terminal};
use sysinfo::{
    Components, Disks, Networks, System,
};

fn main() -> Result<(), io::Error> {

    let mut sys = System::new_all();
    sys.refresh_all();

    println!("system:");

    // RAM and swap information:
    let memory: f64 = (sys.used_memory() as f64) / (sys.total_memory()) as f64 ;
    let memory = (memory * 100.0) as u64;

    let swap: f64 = (sys.used_swap() as f64) / (sys.total_swap()) as f64 ;
    let swap = (swap * 100.0) as u64;

    println!("  => memory: {memory}%");
    println!("  => swap: {swap}%");

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
}
