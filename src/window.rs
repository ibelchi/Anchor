#[cfg(windows)]
use windows_sys::Win32::Foundation::HWND;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetWindowLongW, GetWindowRect, SetLayeredWindowAttributes,
    SetWindowLongW, GWL_EXSTYLE, LWA_ALPHA, WS_EX_LAYERED,
};

pub fn setup() -> Option<isize> {
    #[cfg(windows)]
    unsafe {
        let title: Vec<u16> = "anchor".encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
        
        if hwnd != std::ptr::null_mut() {
            return Some(hwnd as isize);
        }
    }
    None
}

pub fn set_window_transparent(hwnd: isize, opacity: f32) {
    #[cfg(windows)]
    unsafe {
        if hwnd == 0 {
            return;
        }
        let hwnd = hwnd as HWND;
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        
        // Always set WS_EX_LAYERED but NOT WS_EX_TRANSPARENT
        let new_style = ex_style | WS_EX_LAYERED;
        
        SetWindowLongW(hwnd, GWL_EXSTYLE, new_style as i32);
        
        let alpha = (opacity * 255.0) as u8;
        SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
    }
}

pub fn get_window_screen_rect(hwnd: isize) -> Option<(f32, f32, f32, f32)> {
    #[cfg(windows)]
    unsafe {
        if hwnd == 0 { return None; }
        let mut rect = std::mem::zeroed();
        GetWindowRect(hwnd as HWND, &mut rect);
        return Some((
            rect.left as f32,
            rect.top as f32,
            rect.right as f32,
            rect.bottom as f32,
        ));
    }
    #[cfg(not(windows))]
    None
}
