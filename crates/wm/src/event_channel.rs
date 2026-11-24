use std::{cell::OnceCell, sync::OnceLock};

use crossbeam_channel::{Sender, unbounded};

use crate::platform_event::PlatformEvent;

static EVENT_TX: OnceLock<Sender<PlatformEvent>> = OnceLock::new();

pub fn initialize() -> crossbeam_channel::Receiver<PlatformEvent> {
    let (tx, rx) = unbounded();
    EVENT_TX.set(tx).expect("cannot initialize event channel");

    return rx;
}

pub fn send(wm_event: PlatformEvent) {
    if let Some(tx) = EVENT_TX.get() {
        let _ = tx.send(wm_event);
    }
}
