#![allow(non_snake_case)]

use super::{
    shims::ToX86,
    types::{DWORD, HWND, WORD},
};
use crate::{host, machine::Machine, pe, winapi::gdi32};
use anyhow::bail;
use bitflags::bitflags;
use num_traits::FromPrimitive;
use x86::Memory;

const TRACE: bool = true;

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

pub struct Window {
    pub host: Box<dyn host::Window>,
}

pub struct State {
    pub resources_base: u32,
    windows: Vec<Window>,
}
impl State {
    pub fn get_window(&mut self, hwnd: HWND) -> &mut Window {
        &mut self.windows[hwnd.to_raw() as usize - 1]
    }
}
impl Default for State {
    fn default() -> Self {
        State {
            resources_base: 0,
            windows: Vec::new(),
        }
    }
}

#[win32_derive::dllexport]
pub fn RegisterClassA(_machine: &mut Machine, lpWndClass: u32) -> u32 {
    log::warn!("todo: RegisterClassA({:x})", lpWndClass);
    0
}

#[win32_derive::dllexport]
pub fn RegisterClassExA(_machine: &mut Machine, lpWndClassEx: u32) -> u32 {
    log::warn!("todo: RegisterClassExA({:x})", lpWndClassEx);
    0
}

bitflags! {
    pub struct WindowStyle: u32 {
        const WS_POPUP           = 0x80000000;
        const WS_CHILD           = 0x40000000;
        const WS_MINIMIZE        = 0x20000000;
        const WS_VISIBLE         = 0x10000000;
        const WS_DISABLED        = 0x08000000;
        const WS_CLIPSIBLINGS    = 0x04000000;
        const WS_CLIPCHILDREN    = 0x02000000;
        const WS_MAXIMIZE        = 0x01000000;
        const WS_BORDER          = 0x00800000;
        const WS_DLGFRAME        = 0x00400000;
        const WS_VSCROLL         = 0x00200000;
        const WS_HSCROLL         = 0x00100000;
        const WS_SYSMENU         = 0x00080000;
        const WS_THICKFRAME      = 0x00040000;
        const WS_GROUP           = 0x00020000;
        const WS_TABSTOP         = 0x00010000;
    }
}

#[win32_derive::dllexport]
pub fn CreateWindowExA(
    machine: &mut Machine,
    dwExStyle: u32,
    className: Option<&str>,
    windowName: Option<&str>,
    dwStyle: u32,
    X: u32,
    Y: u32,
    nWidth: u32,
    nHeight: u32,
    hWndParent: HWND,
    hMenu: u32,
    hInstance: u32,
    lpParam: u32,
) -> HWND {
    let style = WindowStyle::from_bits(dwStyle).unwrap();
    // TODO: there are also CreateWindow flags like CW_USEDEFAULT.
    // Possible value of x/y:
    //   let CW_USEDEFAULT: u32 = 0x8000_0000;

    // TODO: we ignore most fields here.
    // hInstance is only relevant when multiple DLLs register classes:
    //   https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873
    log::warn!("CreateWindowExA({dwExStyle:x}, {className:?}, {windowName:?}, {style:?}, {X:x}, {Y:x}, {nWidth:x}, {nHeight:x}, {hWndParent:x}, {hMenu:x}, {hInstance:x}, {lpParam:x})");

    let mut host_win = machine.host.create_window();
    host_win.set_title(windowName.unwrap());
    if nWidth > 0 && nHeight > 0 {
        host_win.set_size(nWidth, nHeight);
    }

    let window = Window { host: host_win };
    machine.state.user32.windows.push(window);
    let hwnd = HWND::from_raw(machine.state.user32.windows.len() as u32);
    hwnd
}

#[win32_derive::dllexport]
pub fn GetForegroundWindow(machine: &mut Machine) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn UpdateWindow(_machine: &mut Machine, hWnd: HWND) -> bool {
    // TODO: this should cause a synchronous WM_PAINT.
    true // success
}

#[win32_derive::dllexport]
pub fn ShowWindow(_machine: &mut Machine, hWnd: HWND, _nCmdShow: u32) -> bool {
    let previously_visible = true;
    previously_visible
}

#[win32_derive::dllexport]
pub fn SetFocus(_machine: &mut Machine, hWnd: HWND) -> HWND {
    let prev_focused = HWND::null();
    prev_focused
}

bitflags! {
    pub struct MessageBoxFlags: u32 {
        // None implemented yet.
    }
}

#[win32_derive::dllexport]
pub fn MessageBoxA(
    machine: &mut Machine,
    hWnd: HWND,
    lpText: Option<&str>,
    lpCaption: Option<&str>,
    uType: u32,
) -> u32 {
    // Check flags here to panic on any unexpected flags.
    let _flags = MessageBoxFlags::from_bits(uType).unwrap();
    machine.host.write(
        format!(
            "MessageBox: {}\n{}",
            lpCaption.unwrap_or("Error"),
            lpText.unwrap_or("")
        )
        .as_bytes(),
    );
    1 // IDOK
}

#[win32_derive::dllexport]
pub fn DialogBoxParamA(
    _machine: &mut Machine,
    hInstance: u32,
    lpTemplateName: u32,
    hWndParent: u32,
    lpDialogFunc: u32,
    dwInitParam: u32,
) -> u32 {
    log::warn!("TODO: DialogBoxParamA({hInstance:x}, {lpTemplateName:x}, {hWndParent:x}, {lpDialogFunc:x}, {dwInitParam:x})");
    // TODO: this should run a nested message loop that will call back into lpDialogFunc,
    // which then will call EndDialog to end the nested message loop and return the value
    // passed to EndDialog.
    // Unfortunately we don't know what value to return here otherwise; it's application specific.
    0
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    _machine: &mut Machine,
    _lpMsg: u32,
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    _wRemoveMs: u32,
) -> bool {
    // log::warn!(
    //     "PeekMessageA({lpMsg:x}, {hWnd:x}, {wMsgFilterMin:x}, {wMsgFilterMax:x}, {wRemoveMs:x})"
    // );
    let messages_available = false;
    messages_available
}

#[win32_derive::dllexport]
pub fn GetMessageA(
    _machine: &mut Machine,
    _lpMsg: u32,
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
) -> bool {
    true // do not quit
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_machine: &mut Machine, _lpMsg: u32) -> bool {
    // TODO: translate key-related messages into enqueuing a WM_CHAR.
    false // no message translated
}

#[win32_derive::dllexport]
pub fn DispatchMessageA(_machine: &mut Machine, _lpMsg: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadIconA(_machine: &mut Machine, _hInstance: u32, _lpIconName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LoadCursorA(_machine: &mut Machine, _hInstance: u32, _lpCursorName: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn ShowCursor(_machine: &mut Machine, _bShow: bool) -> u32 {
    // TODO: increment/decrement refcount
    1 // ref=1
}

#[repr(C)]
#[derive(Debug)]
struct BITMAPINFOHEADER {
    biSize: DWORD,
    biWidth: DWORD,
    biHeight: DWORD,
    biPlanes: WORD,
    biBitCount: WORD,
    biCompression: DWORD,
    biSizeImage: DWORD,
    biXPelsPerMeter: DWORD,
    biYPelsPerMeter: DWORD,
    biClrUsed: DWORD,
    biClrImportant: DWORD,
}
unsafe impl x86::Pod for BITMAPINFOHEADER {}
impl BITMAPINFOHEADER {
    fn width(&self) -> u32 {
        self.biWidth
    }
    fn height(&self) -> u32 {
        // Height is negative if top-down DIB.
        (self.biHeight as i32).abs() as u32
    }
    fn is_top_down(&self) -> bool {
        (self.biHeight as i32) < 0
    }
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Box<[[u8; 4]]>,
}
impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmap")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("pixels", &&self.pixels[0..16])
            .finish()
    }
}

fn parse_bitmap(buf: &[u8]) -> anyhow::Result<Bitmap> {
    let header = buf.view::<BITMAPINFOHEADER>(0);
    let header_size = std::mem::size_of::<BITMAPINFOHEADER>();
    if header.biSize as usize != header_size {
        bail!("bad bitmap header");
    }

    let palette_count = match header.biBitCount {
        8 => match header.biClrUsed {
            0 => 256,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let palette_buf = &buf[header_size..(header_size + palette_count * 4)];
    let palette = unsafe {
        std::slice::from_raw_parts(palette_buf.as_ptr() as *const [u8; 4], palette_count)
    };
    let pixels = &buf[header_size + (palette_count * 4)..];

    let width = header.width();
    let height = header.height();
    assert!(pixels.len() as u32 == width * height);

    // Bitmap pixel data is tricky.
    // - Likely bottom-up (first row of data is bottom row of pixels)
    // - Palette will have 0s for the 4th component, while canvas interprets those 0s
    //   as an alpha channel.  We swap there here for 255 for now.
    // It's plausible some software expects the pixel data within a bitmap to be
    // exactly as in the underlying file and we ought to not monkey with it here,
    // but for now let's just transform it into the form the canvas expects.

    fn get_pixel(palette: &[[u8; 4]], val: u8) -> [u8; 4] {
        let [r, g, b, _] = palette[val as usize];
        [r, g, b, 255]
    }

    let pixels = if header.is_top_down() {
        pixels.iter().map(|&p| get_pixel(palette, p)).collect()
    } else {
        let mut v = Vec::with_capacity(pixels.len());
        for y in (0..height as usize).rev() {
            for &p in pixels[y * width as usize..(y + 1) * width as usize].iter() {
                v.push(get_pixel(palette, p));
            }
        }
        v
    };

    Ok(Bitmap {
        width,
        height,
        pixels: pixels.into_boxed_slice(),
    })
}

#[win32_derive::dllexport]
pub fn LoadImageA(
    machine: &mut Machine,
    hInstance: u32,
    name: u32,
    typ: u32,
    _cx: u32,
    _cy: u32,
    fuLoad: u32,
) -> u32 {
    assert!(hInstance == machine.state.kernel32.image_base);
    if !IS_INTRESOURCE(name) {
        log::error!("unimplemented image name {name:x}");
        return 0;
    }

    if fuLoad != 0 {
        log::error!("unimplemented fuLoad {:x}", fuLoad);
        return 0;
    }

    // TODO: it's unclear whether the width/height is obeyed when loading an image.

    const IMAGE_BITMAP: u32 = 0;
    match typ {
        IMAGE_BITMAP => {
            let buf = pe::get_resource(
                &machine.x86.mem[machine.state.kernel32.image_base as usize..],
                machine.state.user32.resources_base,
                pe::RT_BITMAP,
                name,
            )
            .unwrap();
            let bmp = parse_bitmap(buf).unwrap();
            machine.state.gdi32.objects.push(gdi32::Object::Bitmap(bmp));
            machine.state.gdi32.objects.len() as u32
        }
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return 0;
        }
    }
}

#[derive(Debug, FromPrimitive)]
enum SystemMetric {
    CXSCREEN = 1,
    CYSCREEN = 2,
    CYCAPTION = 4,
    CYBORDER = 6,
    CXFRAME = 32,
    CYFRAME = 33,
}

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_machine: &mut Machine, nIndex: u32) -> u32 {
    let metric = match SystemMetric::from_u32(nIndex) {
        Some(metric) => metric,
        None => {
            log::error!("GetSystemMetrics({nIndex})");
            return 0;
        }
    };
    match metric {
        SystemMetric::CXSCREEN => 640,
        SystemMetric::CYSCREEN => 480,
        SystemMetric::CYCAPTION => 3,
        SystemMetric::CYBORDER => 1,
        SystemMetric::CXFRAME => 8,
        SystemMetric::CYFRAME => 8,
    }
}
