#[cfg(windows)]
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, FindWindowW, GetWindowLongW, GetWindowRect, RegisterClassW, SetLayeredWindowAttributes,
    SetWindowLongW, SetWindowPos, ShowWindow, CS_HREDRAW, CS_VREDRAW, GWL_EXSTYLE, HWND_TOPMOST, LWA_ALPHA, SWP_NOACTIVATE,
    SW_SHOWNA, WNDCLASSW, WM_RBUTTONDOWN, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT, WS_POPUP,
};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

pub static RCLICK_DETECTED: AtomicBool = AtomicBool::new(false);
pub static OVERLAY_HWND: AtomicIsize = AtomicIsize::new(0);

#[cfg(windows)]
unsafe extern "system" fn overlay_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_RBUTTONDOWN => {
            RCLICK_DETECTED.store(true, Ordering::Relaxed);
            return 0;
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub fn create_overlay_window() -> isize {
    #[cfg(windows)]
    unsafe {
        let class_name: Vec<u16> = "anchor_overlay".encode_utf16().chain(std::iter::once(0)).collect();

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(overlay_wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: std::ptr::null_mut(),
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wnd_class);

        let ex_style = WS_EX_LAYERED | WS_EX_NOACTIVATE;
        let hwnd = CreateWindowExW(
            ex_style,
            class_name.as_ptr(),
            std::ptr::null(),
            WS_POPUP,
            0, 0, 80, 40,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        if hwnd != std::ptr::null_mut() {
            SetLayeredWindowAttributes(hwnd, 0, 1, LWA_ALPHA);
            ShowWindow(hwnd, SW_SHOWNA);
            OVERLAY_HWND.store(hwnd as isize, Ordering::Relaxed);
            return hwnd as isize;
        }
    }
    0
}

pub fn update_overlay_position(overlay_hwnd: isize, main_hwnd: isize) {
    #[cfg(windows)]
    unsafe {
        if overlay_hwnd == 0 || main_hwnd == 0 {
            return;
        }

        let mut rect = std::mem::zeroed();
        GetWindowRect(main_hwnd as HWND, &mut rect);

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let center_x = rect.left + width / 2;
        let center_y = rect.top + height / 2;

        SetWindowPos(
            overlay_hwnd as HWND,
            HWND_TOPMOST,
            center_x - 40,
            center_y - 20,
            80,
            40,
            SWP_NOACTIVATE,
        );
    }
}

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

pub fn set_window_transparent(hwnd: isize, transparent: bool, opacity: f32) {
    #[cfg(windows)]
    unsafe {
        if hwnd == 0 {
            return;
        }
        let hwnd = hwnd as HWND;
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        
        let new_style = if transparent {
            ex_style | WS_EX_LAYERED | WS_EX_TRANSPARENT
        } else {
            (ex_style | WS_EX_LAYERED) & !WS_EX_TRANSPARENT
        };
        
        SetWindowLongW(hwnd, GWL_EXSTYLE, new_style as i32);
        
        let alpha = (opacity * 255.0) as u8;
        SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
    }
}
