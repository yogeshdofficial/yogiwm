use strum_macros::Display;

use crate::{win_event::WinEvent, window::Window};

#[derive(Debug, Display)]
pub enum WinManagerEvent {
    MouseCapture(WinEvent, Window),
}

impl WinManagerEvent {
    pub fn from_win_event(win_event: WinEvent, window: Window) -> Option<Self> {
        match win_event {
            WinEvent::SystemCaptureStart | WinEvent::SystemCaptureEnd => {
                Option::from(WinManagerEvent::MouseCapture(win_event, window))
            }
            _ => None,
        }
    }
}
