use windows::Win32::{
    Foundation::HWND,
    UI::{
        Accessibility::HWINEVENTHOOK,
        WindowsAndMessaging::{EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_NAMECHANGE},
    },
};

use crate::{win_event, win_manager_event::WinManagerEvent, window};

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
    tracing::info!("{:?}", win_event::WinEvent::try_from(event));
}
