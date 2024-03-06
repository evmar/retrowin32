use super::*;
use crate::winapi::gdi32;

const TRACE_CONTEXT: &'static str = "user32/paint";

#[win32_derive::dllexport]
pub fn InvalidateRect(
    machine: &mut Machine,
    hWnd: HWND,
    lpRect: Option<&RECT>,
    bErase: bool,
) -> bool {
    let window = machine.state.user32.get_window(hWnd).unwrap();
    window.need_paint = true;
    true // success
}

#[derive(Debug)]
#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: u32,
    rcPaint: RECT,
    fRestore: u32,
    fIncUpdate: u32,
    rgbReserved: [u8; 32],
}
unsafe impl memory::Pod for PAINTSTRUCT {}

#[win32_derive::dllexport]
pub fn BeginPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&mut PAINTSTRUCT>) -> HDC {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    if let Some(hbrush) = window.wndclass.background.to_option() {
        if let gdi32::Object::Brush(brush) = machine.state.gdi32.objects.get(hbrush).unwrap() {
            window
                .pixels_mut(&mut *machine.host)
                .fill(brush.color.to_pixel());
        }
    }
    let hdc = window.hdc;
    *lpPaint.unwrap() = PAINTSTRUCT {
        hdc: hdc,
        fErase: 1, // todo
        rcPaint: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        fRestore: 0,          // reserved
        fIncUpdate: 0,        // reserved
        rgbReserved: [0; 32], // reserved
    };
    hdc
}

#[win32_derive::dllexport]
pub fn EndPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&PAINTSTRUCT>) -> bool {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    window.flush_pixels();
    machine
        .state
        .user32
        .windows
        .get_mut(hWnd)
        .unwrap()
        .need_paint = false;
    true
}
