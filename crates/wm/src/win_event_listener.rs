use std::thread::JoinHandle;

use windows::Win32::UI::{
    Accessibility::SetWinEventHook,
    WindowsAndMessaging::{
        DispatchMessageW, EVENT_MAX, EVENT_MIN, GetMessageW, MSG, TranslateMessage,
        WINEVENT_OUTOFCONTEXT,
    },
};

use crate::win_hook_callback::win_event_hook;

pub fn start() -> JoinHandle<()> {
    std::thread::spawn(move || {
        tracing::debug!("weh started");
        unsafe {
            SetWinEventHook(
                EVENT_MIN,
                EVENT_MAX,
                None,
                Some(win_event_hook),
                0,
                0,
                WINEVENT_OUTOFCONTEXT,
            )
        };

        let mut msg: MSG = MSG::default();

        loop {
            tracing::debug!("Waiting for message...");
            // TODO ERROR HANDLING
            let result = unsafe { GetMessageW(&raw mut msg, None, 0, 0) };
            tracing::debug!("GetMessageW returned");

            if result.0 == -1 {
                tracing::debug!("GetMessageW failed");
                break;
            }

            if result.0 == 0 {
                tracing::debug!("Queue exiting");
                break;
            }

            tracing::debug!("Message received: {}", msg.message);
            unsafe {
                let _ = TranslateMessage(&raw const msg);
                DispatchMessageW(&raw const msg);
            }
        }
    })
}
