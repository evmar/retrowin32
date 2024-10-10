//! Draw some bitmaps in different ways.

#![windows_subsystem = "windows"]

use windows_sys::Win32::{
    Foundation::{HWND, LRESULT},
    Graphics::Gdi::*,
    UI::WindowsAndMessaging::*,
};

struct State {
    bitmap: HBITMAP,
}
static mut STATE: State = State { bitmap: 0 };

/// Upward-pointing arrow, 16x16 monochrome.
const ARROW: [u16; 16] = [
    0b0000_0000_0000_0000,
    0b0000_0001_1000_0000,
    0b0000_0011_1100_0000,
    0b0000_0111_1110_0000,
    //
    0b0000_1111_1111_0000,
    0b0001_1111_1111_1000,
    0b0011_1111_1111_1100,
    0b0111_1111_1111_1110,
    //
    0b0000_0011_1100_0000,
    0b0000_0011_1100_0000,
    0b0000_0011_1100_0000,
    0b0000_0011_1100_0000,
    //
    0b0000_0011_1100_0000,
    0b0000_0011_1100_0000,
    0b0000_0011_1100_0000,
    0b0000_0000_0000_0000,
];

/// CreateBitmap from mono pixels.
// TODO: we don't support mono bitmaps yet.
#[allow(unused)]
unsafe fn create_mono() -> HBITMAP {
    CreateBitmap(16, 16, 1, 1, ARROW.as_ptr() as *const _)
}

fn arrow_rgba() -> Vec<u32> {
    let mut pixels = Vec::new();
    for &b in ARROW.iter() {
        for i in 0..16 {
            let bit = (b >> (15 - i)) & 1;
            let color: [u8; 4] = if bit == 0 {
                [0, 0, 0, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };
            pixels.push(u32::from_le_bytes(color));
        }
    }
    pixels
}

/// CreateDIBSection from ARGB pixels.
#[allow(unused)]
unsafe fn create_dib(hdc: HDC) -> HBITMAP {
    let mut bits: *mut u32 = std::ptr::null_mut();
    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 16,
            biHeight: 16,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }; 1],
    };
    let bitmap = CreateDIBSection(
        hdc,
        &info,
        DIB_RGB_COLORS,
        &mut bits as *mut *mut u32 as *mut *mut _,
        0,
        0,
    );
    let bits = std::slice::from_raw_parts_mut(bits, 16 * 16);
    bits.copy_from_slice(&arrow_rgba());

    bitmap
}

unsafe fn paint(hdc: HDC) {
    if STATE.bitmap == 0 {
        STATE.bitmap = create_dib(hdc);
    }

    let hdc_bitmap = CreateCompatibleDC(hdc);
    SelectObject(hdc_bitmap, STATE.bitmap);

    BitBlt(hdc, 16, 16, 16, 16, hdc_bitmap, 0, 0, SRCCOPY);

    let rgba = arrow_rgba();
    let info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 16,
            biHeight: 16,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }; 1],
    };
    SetDIBitsToDevice(
        hdc,
        48,
        16,
        16,
        16,
        0,
        0,
        0,
        16,
        rgba.as_ptr() as *const _,
        &info,
        DIB_RGB_COLORS,
    );

    StretchDIBits(
        hdc,
        16,
        48,
        32,
        32,
        0,
        0,
        16,
        16,
        rgba.as_ptr() as *const _,
        &info,
        DIB_RGB_COLORS,
        SRCCOPY,
    );

    DeleteDC(hdc_bitmap);
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_LBUTTONDOWN => {
            InvalidateRect(hwnd, std::ptr::null(), 0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            paint(hdc);
            EndPaint(hwnd, &ps);
            0
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

unsafe fn create_window() -> HWND {
    let class_name = b"dibs\0";
    let wc = WNDCLASSA {
        style: 0,
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0,
        hIcon: 0,
        hCursor: 0,
        hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
        lpszMenuName: std::ptr::null(),
        lpszClassName: class_name.as_ptr(),
    };
    RegisterClassA(&wc);

    let hwnd = CreateWindowExA(
        0,
        class_name.as_ptr(),
        b"title\0".as_ptr(),
        WS_OVERLAPPED,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        400,
        300,
        0,
        0,
        0,
        std::ptr::null(),
    );
    if hwnd == 0 {
        panic!("create failed");
    }
    ShowWindow(hwnd, SW_NORMAL);

    hwnd
}

fn main() {
    unsafe {
        create_window();
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}
