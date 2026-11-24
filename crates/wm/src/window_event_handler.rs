use crate::platform_event::{
    PlatformEvent::{self, Window},
    WindowEvent,
};

pub fn handle_window_event(window_event: WindowEvent) {
    tracing::info!(
        "Received WindowEvent: {:?} on title:{:?}",
        window_event,
        window_event.window().title()
    );
}
