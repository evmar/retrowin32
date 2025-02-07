//! Draw some bitmaps in different ways.

#![windows_subsystem = "windows"]

use windows_sys::Win32::{
    Foundation::{HWND, LRESULT},
    Graphics::Gdi::*,
    UI::WindowsAndMessaging::*,
};

/// Upward-pointing arrow, 16x16 monochrome.
#[rustfmt::skip]
const ARROW: [u8; 16 * 2] = [
    0b0000_0000, 0b0000_0000,
    0b0000_0001, 0b1000_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0111, 0b1110_0000,
    0b0000_1111, 0b1111_0000,
    0b0001_1111, 0b1111_1000,
    0b0011_1111, 0b1111_1100,
    0b0111_1111, 0b1111_1110,
    0b0000_0011, 0b1100_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0011, 0b1100_0000,
    0b0000_0000, 0b0000_0000,
];

fn arrow_rgba() -> Vec<u32> {
    let mut pixels = Vec::new();
    for &b in ARROW.iter() {
        for i in 0..8 {
            let bit = (b >> (7 - i)) & 1;
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

struct State {
    mono: HBITMAP,
    dib: HBITMAP,
    /// Owned by dib; writable pixel data.
    dib_pixels: &'static mut [u32; 16 * 16],
}

impl State {
    unsafe fn new(hdc: HDC) -> Self {
        let mono = Self::create_mono();
        let (dib, dib_pixels) = Self::create_dib(hdc);
        Self {
            mono,
            dib,
            dib_pixels,
        }
    }

    unsafe fn create_dib(hdc: HDC) -> (HBITMAP, &'static mut [u32; 16 * 16]) {
        let mut bits: *mut [u32; 16 * 16] = std::ptr::null_mut();
        let info = BITMAPINFOHEADER {
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
        };
        let bitmap = CreateDIBSection(
            hdc,
            &info as *const _ as *const _,
            DIB_RGB_COLORS,
            &mut bits as *mut *mut _ as *mut *mut _,
            0,
            0,
        );
        let pixels = &mut *bits;
        pixels.copy_from_slice(&arrow_rgba());
        (bitmap, pixels)
    }

    unsafe fn create_mono() -> HBITMAP {
        CreateBitmap(16, 16, 1, 1, ARROW.as_ptr() as *const _)
    }
}

static mut STATE: Option<State> = None;

#[allow(non_snake_case)]
unsafe fn paint_SetDIBitsToDevice(hdc: HDC, y: i32, rgba: &[u32]) {
    struct Cfg {
        startScan: u32,
        invertY: bool,
    }
    let cfgs = [
        Cfg {
            startScan: 0,
            invertY: false,
        },
        Cfg {
            startScan: 6,
            invertY: false,
        },
        Cfg {
            startScan: 0,
            invertY: true,
        },
        Cfg {
            startScan: 6,
            invertY: true,
        },
    ];

    for (i, cfg) in cfgs.iter().enumerate() {
        let info = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 16,
            biHeight: if cfg.invertY { -16 } else { 16 },
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };
        let x = 32 + (i as i32 * 32);
        SetDIBitsToDevice(
            hdc,
            x,
            y,
            16,
            16,
            0,
            // TODO: we don't support ySrc != startScan yet
            cfg.startScan as i32,
            cfg.startScan,
            16,
            rgba.as_ptr() as *const _,
            &info as *const _ as *const BITMAPINFO,
            DIB_RGB_COLORS,
        );
    }
}

#[allow(non_snake_case)]
unsafe fn paint_StretchDIBits(hdc: HDC, y: i32, rgba: &[u32]) {
    let info = BITMAPINFOHEADER {
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
    };

    StretchDIBits(
        hdc,
        32,
        y,
        32,
        32,
        0,
        0,
        16,
        16,
        rgba.as_ptr() as *const _,
        &info as *const _ as *const BITMAPINFO,
        DIB_RGB_COLORS,
        SRCCOPY,
    );
}

#[allow(static_mut_refs)]
unsafe fn paint(hdc: HDC) {
    let state = match STATE {
        Some(ref state) => state,
        None => {
            STATE = Some(State::new(hdc));
            STATE.as_ref().unwrap()
        }
    };

    let hdc_bitmap = CreateCompatibleDC(hdc);

    let mut row = 1;
    SelectObject(hdc_bitmap, state.mono);
    BitBlt(hdc, 32, row * 32, 16, 16, hdc_bitmap, 0, 0, SRCCOPY);
    row += 1;

    SelectObject(hdc_bitmap, state.dib);
    BitBlt(hdc, 32, row * 32, 16, 16, hdc_bitmap, 0, 0, SRCCOPY);
    row += 1;

    let rgba = arrow_rgba();
    paint_SetDIBitsToDevice(hdc, row * 32, &rgba);
    row += 1;

    paint_StretchDIBits(hdc, row * 32, &rgba);

    DeleteDC(hdc_bitmap);
}

#[allow(static_mut_refs)]
unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_LBUTTONDOWN => {
            let s = STATE.as_mut().unwrap();
            for i in 0..32 * 2 {
                s.dib_pixels[i] = 0xff00_00ff;
            }
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
