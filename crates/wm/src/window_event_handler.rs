use crate::platform_event::WindowEvent;

pub fn handle_window_event(window_event: WindowEvent) {
    tracing::info!(
        "Received WindowEvent: {:?} on title:{:?}",
        window_event,
        window_event.window().title()
    );
}
