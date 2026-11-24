use color_eyre::eyre::{self, Error};
use crossbeam_channel::{Receiver, Sender};

use crate::win_manager_event::WinManagerEvent;

pub struct WM {
    event_tx: Sender<WinManagerEvent>,
    event_rx: Receiver<WinManagerEvent>,
}

impl WM {
    pub fn new() -> eyre::Result<Self> {
        let channel = crossbeam_channel::bounded(20);
        Ok(Self {
            event_rx: channel.1,
            event_tx: channel.0,
        })
    }
}
