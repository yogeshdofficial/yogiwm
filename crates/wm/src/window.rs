use color_eyre::eyre::{self, Ok};
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::GetWindowTextW};

#[derive(Debug, Clone, Copy)]
pub struct SendableHwnd(pub HWND);
unsafe impl Send for SendableHwnd {}

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub hwnd: SendableHwnd,
}

impl Window {
    pub fn new(hwnd: SendableHwnd) -> Self {
        Self { hwnd }
    }

    pub fn title(self) -> eyre::Result<String> {
        let mut text: [u16; 512] = [0; 512];
        let length: i32 = unsafe { GetWindowTextW(self.hwnd.0, &mut text) };
        Ok(String::from_utf16_lossy(&text[..length as usize]))
    }
}
