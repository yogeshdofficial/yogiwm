use crossbeam_channel::{Receiver, Sender};

use crate::platform_event::PlatformEvent;

pub struct WM {
    event_tx: Sender<PlatformEvent>,
    event_rx: Receiver<PlatformEvent>,
}

impl WM {
    pub fn new() -> Self {
        let channel = crossbeam_channel::bounded(20);
        Self {
            event_rx: channel.1,
            event_tx: channel.0,
        }
    }
}
