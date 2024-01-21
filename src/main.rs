use std::io;
use tui::{backend::CrosstermBackend, Terminal};
use sysinfo::{
    Components, Disks, Networks, System,
};

fn main() -> Result<(), io::Error> {

    //initialise tui
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sys = System::new_all();
    sys.refresh_all();

    // RAM and swap information:
    let memory: f64 = (sys.used_memory() as f64) / (sys.total_memory()) as f64 ;
    let memory = (memory * 100.0) as u64;

    let swap: f64 = (sys.used_swap() as f64) / (sys.total_swap()) as f64 ;
    let swap = (swap * 100.0) as u64;

    Ok(())
}
