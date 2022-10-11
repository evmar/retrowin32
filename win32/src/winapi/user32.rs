#![allow(non_snake_case)]

use anyhow::bail;

use super::X86;
use crate::{
    host,
    memory::{Memory, Pod, DWORD, WORD},
    pe, winapi,
    winapi::gdi32,
};

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
    pub fn new() -> Self {
        State {
            resources_base: 0,
            windows: Vec::new(),
        }
    }

    pub fn get_window(&mut self, hwnd: u32) -> &mut Window {
        &mut self.windows[hwnd as usize - 1]
    }
}

fn RegisterClassA(_x86: &mut X86, lpWndClass: u32) -> u32 {
    log::warn!("todo: RegisterClassA({:x})", lpWndClass);
    0
}

fn CreateWindowExA(
    x86: &mut X86,
    dwExStyle: u32,
    lpClassName: u32,
    lpWindowName: u32,
    dwStyle: u32,
    X: u32,
    Y: u32,
    nWidth: u32,
    nHeight: u32,
    hWndParent: u32,
    hMenu: u32,
    hInstance: u32,
    lpParam: u32,
) -> u32 {
    // Possible value of x/y:
    //   let CW_USEDEFAULT: u32 = 0x8000_0000;

    // TODO: we ignore most fields here.
    // hInstance is only relevant when multiple DLLs register classes:
    //   https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873
    log::warn!("CreateWindowExA({dwExStyle:x}, {lpClassName:x}, {lpWindowName:x}, {dwStyle:x}, {X:x}, {Y:x}, {nWidth:x}, {nHeight:x}, {hWndParent:x}, {hMenu:x}, {hInstance:x}, {lpParam:x})");

    let mut x86_win = x86.host.create_window();
    let name = x86.mem[lpWindowName as usize..].read_strz();
    x86_win.set_title(&name);
    x86_win.set_size(nWidth, nHeight);

    let window = Window { host: x86_win };
    x86.state.user32.windows.push(window);
    x86.state.user32.windows.len() as u32
}

fn UpdateWindow(_x86: &mut X86, _hWnd: u32) -> u32 {
    // TODO: this should cause a synchronous WM_PAINT.
    0
}

fn ShowWindow(_x86: &mut X86, _hWnd: u32, _nCmdShow: u32) -> u32 {
    0
}

fn SetFocus(_x86: &mut X86, _hWnd: u32) -> u32 {
    // TODO: supposed to return previous focused hwnd.
    0
}

fn MessageBoxA(x86: &mut X86, _hWnd: u32, lpText: u32, lpCaption: u32, _uType: u32) -> u32 {
    let caption = &x86.mem[lpCaption as usize..].read_strz();
    let text = &x86.mem[lpText as usize..].read_strz();
    x86.host
        .write(format!("MessageBox: {}\n{}", caption, text).as_bytes());
    1 // IDOK
}

fn PeekMessageA(
    _x86: &mut X86,
    lpMsg: u32,
    hWnd: u32,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMs: u32,
) -> u32 {
    log::warn!(
        "PeekMessageA({lpMsg:x}, {hWnd:x}, {wMsgFilterMin:x}, {wMsgFilterMax:x}, {wRemoveMs:x})"
    );
    0 // no messages
}

fn LoadIconA(_x86: &mut X86, _hInstance: u32, _lpIconName: u32) -> u32 {
    0
}

fn LoadCursorA(_x86: &mut X86, _hInstance: u32, _lpCursorName: u32) -> u32 {
    0
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
unsafe impl Pod for BITMAPINFOHEADER {}

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
    if header.biSize.get() as usize != header_size {
        bail!("bad bitmap header");
    }

    let palette_count = match header.biBitCount.get() {
        8 => match header.biClrUsed.get() {
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

    let width = header.biWidth.get();
    let height = header.biHeight.get();
    assert!(pixels.len() as u32 == width * height);

    let pixels: Vec<_> = pixels.iter().map(|&p| palette[p as usize]).collect();

    Ok(Bitmap {
        width,
        height,
        pixels: pixels.into_boxed_slice(),
    })
}

fn LoadImageA(
    x86: &mut X86,
    hInstance: u32,
    name: u32,
    typ: u32,
    _cx: u32,
    _cy: u32,
    fuLoad: u32,
) -> u32 {
    assert!(hInstance == x86.state.kernel32.image_base);
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
                &x86.mem[x86.state.kernel32.image_base as usize..],
                x86.state.user32.resources_base,
                pe::RT_BITMAP,
                name,
            )
            .unwrap();
            let bmp = parse_bitmap(buf).unwrap();
            x86.state.gdi32.objects.push(gdi32::Object::Bitmap(bmp));
            x86.state.gdi32.objects.len() as u32
        }
        _ => {
            log::error!("unimplemented image type {:x}", typ);
            return 0;
        }
    }
}

fn GetSystemMetrics(_x86: &mut X86, nIndex: u32) -> u32 {
    const SM_CXSCREEN: u32 = 0;
    const SM_CYSCREEN: u32 = 1;

    match nIndex {
        SM_CXSCREEN => 640,
        SM_CYSCREEN => 480,
        _ => {
            log::warn!("GetSystemMetrics({nIndex})");
            0
        }
    }
}

winapi!(
    fn RegisterClassA(lpWndClass: u32);
    fn CreateWindowExA(
        dwExStyle: u32,
        lpClassName: u32,
        lpWindowName: u32,
        dwStyle: u32,
        X: u32,
        Y: u32,
        nWidth: u32,
        nHeight: u32,
        hWndParent: u32,
        hMenu: u32,
        hInstance: u32,
        lpParam: u32,
    );
    fn UpdateWindow(hWnd: u32);
    fn ShowWindow(hWnd: u32, nCmdShow: u32);
    fn SetFocus(hWnd: u32);

    fn MessageBoxA(hWnd: u32, lpText: u32, lpCaption: u32, uType: u32);

    fn PeekMessageA(lpMsg: u32, hWnd: u32, wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMs: u32);

    fn LoadIconA(hInstance: u32, lpIconName: u32);
    fn LoadCursorA(hInstance: u32, lpCursorName: u32);
    fn LoadImageA(hInstance: u32, name: u32, typ: u32, cx: u32, cy: u32, fuLoad: u32);
    fn GetSystemMetrics(nIndex: u32);
);
