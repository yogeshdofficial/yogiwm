use std::sync::{Arc, Mutex};

use color_eyre::eyre::Result;
use tracing_subscriber::EnvFilter;

use crate::{win_event_listener::start, wm::WM};

mod win_event;
mod win_event_listener;
mod win_hook_callback;
mod win_manager_event;
mod window;
mod wm;

fn main() -> Result<()> {
    color_eyre::install()?;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .pretty()
        .init();

    // start_wm();
    let _ = start().join();
    let wm = Arc::new(Mutex::new(WM::new()));

    Ok(())
}
// fn start_wm() {}
