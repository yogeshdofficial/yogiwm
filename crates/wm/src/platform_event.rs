use strum_macros::Display;

use crate::{
    win_event::WinEvent::{self, ObjectUncloaked, SystemMoveSizeEnd},
    window::Window,
};

#[derive(Debug, Display, Clone, Copy)]
pub enum PlatformEvent {
    Window(WindowEvent),
}

#[derive(Debug, Display, Clone, Copy)]
pub enum WindowEvent {
    // MouseCapture(Window),
    Focused(Window),
    Hidden(Window),
    MovedOrResized(Window),
    Minimized(Window),
    MinimizeEnded(Window),
    Shown(Window),
    TitleChanged(Window),
    Destroyed(Window),
}

impl WindowEvent {
    pub fn from_win_event(win_event: WinEvent, window: Window) -> Option<Self> {
        match win_event {
            // WinEvent::SystemCaptureStart | WinEvent::SystemCaptureEnd => {
            //     Option::from(Self::MouseCapture(window))
            // }
            WinEvent::ObjectDestroy => Option::from(Self::Destroyed(window)),
            WinEvent::SystemForeground => Option::from(Self::Focused(window)),
            WinEvent::ObjectHide | WinEvent::ObjectCloaked => Option::from(Self::Hidden(window)),
            WinEvent::ObjectLocationChange => Option::from(Self::MovedOrResized(window)),
            WinEvent::SystemMinimizeStart => Option::from(Self::Minimized(window)),
            WinEvent::SystemMinimizeEnd => Option::from(Self::MinimizeEnded(window)),
            WinEvent::SystemMoveSizeStart | SystemMoveSizeEnd => {
                Option::from(Self::MovedOrResized(window))
            }
            WinEvent::ObjectShow | ObjectUncloaked => Option::from(Self::Shown(window)),
            WinEvent::ObjectNameChange => Option::from(Self::TitleChanged(window)),
        }
    }
    pub fn window(&self) -> &Window {
        match self {
            Self::Focused(w)
            | Self::Hidden(w)
            | Self::MovedOrResized(w)
            | Self::Minimized(w)
            | Self::MinimizeEnded(w)
            | Self::Shown(w)
            | Self::TitleChanged(w)
            | Self::Destroyed(w) => w,
        }
    }
}
