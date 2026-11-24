use std::sync::OnceLock;

use crossbeam_channel::{Receiver, Sender};
use tracing::instrument;
use windows::Win32::{
    Foundation::HWND,
    UI::{
        Accessibility::HWINEVENTHOOK,
        WindowsAndMessaging::{
            CHILDID_SELF, EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_NAMECHANGE, OBJID_WINDOW,
        },
    },
};

use crate::{
    event_channel::send,
    platform_event::{PlatformEvent, WindowEvent},
    win_event::WinEvent,
    window::{self, SendableHwnd},
};

// #[instrument]
// returns winmanagerevent
pub extern "system" fn win_event_hook(
    _h_win_event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    id_object: i32,
    id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    if hwnd.0 as isize == 0 || id_object != OBJID_WINDOW.0 || id_child != CHILDID_SELF as i32 {
        return;
    }
    // TODO
    // object.0
    // println!(
    //     "WinEvent received: event={event}, hwnd={hwnd:?}, object={id_object}, child={id_child} optional={:?}",
    //     win_event::WinEvent::try_from(event)
    // );

    // tracing::info!(
    //     "{:?}",
    //     WinManagerEvent::from_win_event(
    //         win_event::WinEvent::try_from(event),
    //         window::Window {
    //             hwnd: hwnd.0 as isize
    //         }
    //     )
    // );
    if event == EVENT_OBJECT_LOCATIONCHANGE || event == EVENT_OBJECT_NAMECHANGE {
        return;
    }

    let Ok(win_event) = WinEvent::try_from(event) else {
        tracing::trace!(?event, "Cannot convert to WinEvent");
        // TODO
        return;
    };

    // TODO

    //TODO
    let Some(window_event) =
        WindowEvent::from_win_event(win_event, window::Window::new(SendableHwnd(hwnd)))
    else {
        tracing::trace!(?win_event, "Unhandled WinEvent");
        return;
    };
    // tracing::debug!(?wm_event, "WMEvent received");
    send(PlatformEvent::Window(window_event));
}
