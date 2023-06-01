#![allow(non_snake_case)]

use super::{
    stack_args::ToX86,
    types::{DWORD, HWND, WORD},
};
use crate::{host, machine::Machine, pe, winapi::gdi32};
use anyhow::bail;
use bitflags::bitflags;
use num_traits::FromPrimitive;
use std::collections::VecDeque;
use std::rc::Rc;
use x86::{Mem, Memory};

const TRACE_CONTEXT: &'static str = "user32";

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MSG {
    hwnd: HWND,
    message: WM,
    wParam: u32,
    lParam: u32,
    time: u32,
    // TODO: struct POINT
    pt_x: u32,
    pt_y: u32,
    lPrivate: u32,
}
unsafe impl x86::Pod for MSG {}

pub struct Window {
    pub host: Box<dyn host::Window>,
    /// Index into State::wndclasses.
    wndclass: Rc<WndClass>,
}

struct WndClass {
    name: String,
    wndproc: u32,
}

pub struct State {
    pub resources: pe::IMAGE_DATA_DIRECTORY,
    wndclasses: Vec<Rc<WndClass>>,
    windows: Vec<Window>,
    messages: VecDeque<MSG>,
}
impl State {
    pub fn get_window(&mut self, hwnd: HWND) -> &mut Window {
        &mut self.windows[hwnd.to_raw() as usize - 1]
    }
}
impl Default for State {
    fn default() -> Self {
        State {
            resources: pe::IMAGE_DATA_DIRECTORY::default(),
            wndclasses: Vec::new(),
            windows: Vec::new(),
            messages: VecDeque::new(),
        }
    }
}

type HICON = u32;
type HINSTANCE = u32;
type HCURSOR = u32;
type HBRUSH = u32;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WNDCLASSA {
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: u32,
    lpszClassName: u32,
}
unsafe impl x86::Pod for WNDCLASSA {}

#[win32_derive::dllexport]
pub fn RegisterClassA(machine: &mut Machine, lpWndClass: Option<&WNDCLASSA>) -> u32 {
    let wndclass = lpWndClass.unwrap();
    let ex = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        style: wndclass.style,
        lpfnWndProc: wndclass.lpfnWndProc,
        cbClsExtra: wndclass.cbClsExtra,
        cbWndExtra: wndclass.cbWndExtra,
        hInstance: wndclass.hInstance,
        hIcon: wndclass.hIcon,
        hCursor: wndclass.hCursor,
        hbrBackground: wndclass.hbrBackground,
        lpszMenuName: wndclass.lpszMenuName,
        lpszClassName: wndclass.lpszClassName,
        hIconSm: 0,
    };
    RegisterClassExA(machine, Some(&ex))
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WNDCLASSEXA {
    cbSize: u32,
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl x86::Pod for WNDCLASSEXA {}

#[win32_derive::dllexport]
pub fn RegisterClassExA(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXA>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let atom = machine.state.user32.wndclasses.len() as u32 + 1;
    let name = machine
        .x86
        .mem
        .slice(lpWndClassEx.lpszClassName..)
        .read_strz()
        .to_string();
    let wndclass = WndClass {
        // atom,
        name,
        wndproc: lpWndClassEx.lpfnWndProc,
    };
    machine.state.user32.wndclasses.push(Rc::new(wndclass));
    atom
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
impl TryFrom<u32> for WindowStyle {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        WindowStyle::from_bits(value).ok_or(value)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum WM {
    CREATE = 0x0001,
    ACTIVATEAPP = 0x001C,
}
impl TryFrom<u32> for WM {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == WM::CREATE as u32 => WM::CREATE,
            x if x == WM::ACTIVATEAPP as u32 => WM::ACTIVATEAPP,
            x => return Err(x),
        })
    }
}

#[win32_derive::dllexport]
pub async fn CreateWindowExA(
    machine: &mut Machine,
    dwExStyle: u32,
    lpClassName: u32,
    lpWindowName: Option<&str>,
    dwStyle: Result<WindowStyle, u32>,
    X: u32,
    Y: u32,
    nWidth: u32,
    nHeight: u32,
    hWndParent: HWND,
    hMenu: u32,
    hInstance: u32,
    lpParam: u32,
) -> HWND {
    if lpClassName < 1 << 16 {
        todo!("numeric wndclass reference");
    }
    let class_name = machine.x86.mem.slice(lpClassName..).read_strz();
    let wndclass = machine
        .state
        .user32
        .wndclasses
        .iter()
        .find(|c| c.name == class_name)
        .unwrap()
        .clone();

    let _style = dwStyle.unwrap();
    const CW_USEDEFAULT: u32 = 0x8000_0000;

    // hInstance is only relevant when multiple DLLs register classes:
    //   https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873

    let mut host_win = machine.host.create_window();
    host_win.set_title(lpWindowName.unwrap());
    if nWidth > 0 && nHeight > 0 {
        let width = if nWidth == CW_USEDEFAULT { 640 } else { nWidth };
        let height = if nHeight == CW_USEDEFAULT {
            480
        } else {
            nHeight
        };
        host_win.set_size(width, height);
    }

    let window = Window {
        host: host_win,
        wndclass,
    };
    machine.state.user32.windows.push(window);
    let hwnd = HWND::from_raw(machine.state.user32.windows.len() as u32);

    // Synchronously dispatch WM_CREATE.
    let msg = MSG {
        hwnd,
        message: WM::CREATE,
        wParam: 0,
        lParam: 0, // TODO: CREATESTRUCT
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    };
    DispatchMessageA(machine, Some(&msg)).await;

    // Enqueue WM_ACTIVATEAPP message.
    machine.state.user32.messages.push_back(MSG {
        hwnd,
        message: WM::ACTIVATEAPP,
        wParam: true as u32, // activating
        lParam: 0,           // TODO: thread id
        time: 0,             // TODO
        pt_x: 0,             // TODO
        pt_y: 0,             // TODO
        lPrivate: 0,
    });

    hwnd
}

#[win32_derive::dllexport]
pub fn GetForegroundWindow(machine: &mut Machine) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn GetActiveWindow(machine: &mut Machine) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn GetLastActivePopup(machine: &mut Machine) -> HWND {
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

#[win32_derive::dllexport]
pub fn SetCursor(_machine: &mut Machine, hCursor: u32) -> u32 {
    0 // previous: null
}

/*
pub mod MessageBoxFlags {
    pub const ABORTRETRYIGNORE: u32 = 0x00000002;
    pub const CANCELTRYCONTINUE: u32 = 0x00000006;
    pub const HELP: u32 = 0x00004000;
    pub const OK: u32 = 0x00000000;
    pub const OKCANCEL: u32 = 0x00000001;
    pub const RETRYCANCEL: u32 = 0x00000005;
    pub const YESNO: u32 = 0x00000004;
    pub const YESNOCANCEL: u32 = 0x00000003;

    pub const ICONEXCLAMATION: u32 = 0x00000030;
    pub const ICONWARNING: u32 = 0x00000030;
    pub const ICONINFORMATION: u32 = 0x00000040;
    pub const ICONASTERISK: u32 = 0x00000040;
    pub const ICONQUESTION: u32 = 0x00000020;
    pub const ICONSTOP: u32 = 0x00000010;
    pub const ICONERROR: u32 = 0x00000010;
    pub const ICONHAND: u32 = 0x00000010;

    pub const DEFBUTTON1: u32 = 0x00000000;
    pub const DEFBUTTON2: u32 = 0x00000100;
    pub const DEFBUTTON3: u32 = 0x00000200;
    pub const DEFBUTTON4: u32 = 0x00000300;

    pub const APPLMODAL: u32 = 0x00000000;
    pub const SYSTEMMODAL: u32 = 0x00001000;
    pub const TASKMODAL: u32 = 0x00002000;
    pub const DEFAULT_DESKTOP_ONLY: u32 = 0x00020000;
    pub const RIGHT: u32 = 0x00080000;
    pub const RTLREADING: u32 = 0x00100000;
    pub const SETFOREGROUND: u32 = 0x00010000;
    pub const TOPMOST: u32 = 0x00040000;
    pub const SERVICE_NOTIFICATION: u32 = 0x00200000;
}
*/

#[win32_derive::dllexport]
pub fn MessageBoxA(
    machine: &mut Machine,
    hWnd: HWND,
    lpText: Option<&str>,
    lpCaption: Option<&str>,
    uType: u32,
) -> u32 {
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
    hWndParent: HWND,
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

bitflags! {
    pub struct RemoveMsg: u32 {
        const PM_NOREMOVE = 0x0000;
        const PM_REMOVE = 0x0001;
        const PM_NOYIELD = 0x0002;
    }
}
impl TryFrom<u32> for RemoveMsg {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        RemoveMsg::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMsg: Result<RemoveMsg, u32>,
) -> bool {
    // TODO: obey HWND.
    let remove = wRemoveMsg.unwrap();
    let msg = match machine.state.user32.messages.front() {
        Some(msg) => msg,
        None => return false,
    };
    *lpMsg.unwrap() = msg.clone();

    if remove.contains(RemoveMsg::PM_REMOVE) {
        machine.state.user32.messages.pop_front();
    }

    true
}

#[win32_derive::dllexport]
pub fn GetMessageA(
    machine: &mut Machine,
    lpMsg: Option<&mut MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> bool {
    if !PeekMessageA(
        machine,
        lpMsg,
        hWnd,
        wMsgFilterMin,
        wMsgFilterMax,
        Ok(RemoveMsg::PM_REMOVE),
    ) {
        todo!();
    }
    return true;
}

#[win32_derive::dllexport]
pub fn WaitMessage(_machine: &mut Machine) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_machine: &mut Machine, lpMsg: Option<&MSG>) -> bool {
    // TODO: translate key-related messages into enqueuing a WM_CHAR.
    false // no message translated
}

#[win32_derive::dllexport]
pub async fn DispatchMessageA(m: *mut Machine, lpMsg: Option<&MSG>) -> u32 {
    let machine = unsafe { &mut *m };
    let msg = lpMsg.unwrap();
    let window = &machine.state.user32.windows[msg.hwnd.to_raw() as usize - 1];
    // TODO: SetWindowLong can change the wndproc.
    crate::shims::async_call(
        machine,
        window.wndclass.wndproc,
        vec![
            msg.hwnd.to_raw(),
            msg.message as u32,
            msg.wParam,
            msg.lParam,
        ],
    )
    .await;
    0
}

#[win32_derive::dllexport]
pub fn DefWindowProcA(
    _machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
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

fn parse_bitmap(buf: &Mem) -> anyhow::Result<Bitmap> {
    let header = buf.view::<BITMAPINFOHEADER>(0);
    let header_size = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    if header.biSize != header_size {
        bail!("bad bitmap header");
    }

    let palette_count = match header.biBitCount {
        8 => match header.biClrUsed {
            0 => 256,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let palette_buf = buf.slice(header_size..).slice(..palette_count * 4);
    let palette = unsafe {
        std::slice::from_raw_parts(
            palette_buf.as_slice_todo().as_ptr() as *const [u8; 4],
            palette_count as usize,
        )
    };
    let pixels = buf.slice(header_size + (palette_count * 4)..);

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
        pixels
            .as_slice_todo()
            .iter()
            .map(|&p| get_pixel(palette, p))
            .collect()
    } else {
        let mut v = Vec::with_capacity(pixels.len() as usize);
        for y in (0..height).rev() {
            for &p in pixels
                .slice(y * width..)
                .slice(..width)
                .as_slice_todo()
                .iter()
            {
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
    cx: u32,
    cy: u32,
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
                &machine.x86.mem.slice(machine.state.kernel32.image_base..),
                &machine.state.user32.resources,
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
