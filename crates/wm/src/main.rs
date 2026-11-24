
use color_eyre::eyre::Result;
use tracing_subscriber::EnvFilter;

use crate::{
    platform_event::PlatformEvent, win_event_listener::start,
    window_event_handler::handle_window_event,
};

mod event_channel;
mod platform_event;
mod win_event;
mod win_event_listener;
mod win_hook_callback;
mod window;
mod window_event_handler;
mod wm;

fn main() -> Result<()> {
    color_eyre::install()?;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .pretty()
        .with_target(false)
        .init();

    // start_wm();
    let rx = event_channel::initialize();

    let _ = start();

    // let wm = Arc::new(Mutex::new(WM::new()));

    while let Ok(event) = rx.recv() {
        match event {
            PlatformEvent::Window(window_event) => handle_window_event(window_event),
        }
        // tracing::trace!(?event, "PlatformEvent received in event loop");
    }

    Ok(())
}
// fn start_wm() {}
