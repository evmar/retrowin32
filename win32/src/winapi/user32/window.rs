use super::*;
use crate::{
    host,
    str16::expect_ascii,
    winapi::{
        bitmap::{self, BitmapRGBA32},
        gdi32::HDC,
        stack_args::{ArrayWithSize, FromArg},
        types::{Str16, String16, HWND, POINT, RECT},
    },
    Host, Machine, SurfaceOptions,
};
use bitflags::bitflags;
use memory::{Extensions, Mem};
use std::rc::Rc;

const TRACE_CONTEXT: &'static str = "user32/window";

/*

## Window initalization

A small test app that created a window and printed the messages received
had this sequence of events, where the braced message lists are messages
that were processed during the function call:

CreateWindow() {
    msg: 36 WM_GETMINMAXINFO
    msg: 129 WM_NCCREATE
    msg: 131 WM_NCCALCSIZE
    msg: 1 WM_CREATE
}
ShowWindow() {
    msg: 24 WM_SHOWWINDOW
    msg: 70 WM_WINDOWPOSCHANGING
    msg: 28 WM_ACTIVATEAPP
    msg: 134 WM_NCACTIVATE
    msg: 6 WM_ACTIVATE
    msg: 7 WM_SETFOCUS
    msg: 71 WM_WINDOWPOSCHANGED
    msg: 5 WM_SIZE
    msg: 3 WM_MOVE
}

*/

pub struct WindowPixels {
    pub bitmap: BitmapRGBA32,
}
impl WindowPixels {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let raw = {
            let mut p = Vec::with_capacity(size);
            p.resize(size, [0u8; 4]);
            p.into_boxed_slice()
        };
        Self {
            bitmap: BitmapRGBA32 {
                width,
                height,
                pixels: bitmap::PixelData::Owned(raw),
            },
        }
    }
}

pub struct UpdateRegion {
    /// Whether to erase background in BeginPaint.
    pub erase_background: bool,
    // TODO: rect
}

pub struct Window {
    pub hwnd: HWND,
    pub hdc: HDC,
    pub host: Box<dyn host::Window>,
    pub surface: Box<dyn host::Surface>,
    /// Client area width (not total window width).
    pub width: u32,
    /// Client area height (not total window height).
    pub height: u32,
    pub wndclass: Rc<WndClass>,
    pub pixels: Option<WindowPixels>,
    pub dirty: Option<UpdateRegion>,
    pub style: WindowStyle,
}

impl Window {
    fn ensure_pixels(&mut self) -> &mut WindowPixels {
        match self.pixels {
            Some(ref mut px) => px,
            None => {
                self.pixels = Some(WindowPixels::new(self.width, self.height));
                self.pixels.as_mut().unwrap()
            }
        }
    }

    pub fn bitmap_mut<'a>(&mut self) -> &mut BitmapRGBA32 {
        &mut self.ensure_pixels().bitmap
    }

    pub fn flush_pixels(&mut self, mem: Mem) {
        if let Some(pixels) = &mut self.pixels {
            self.surface
                .write_pixels(&pixels.bitmap.pixels.as_slice(mem));
            self.surface.show();
        }
    }

    pub fn set_client_size(&mut self, host: &mut dyn Host, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.host.set_size(width, height);
        self.surface = host.create_surface(
            self.hwnd.to_raw(),
            &host::SurfaceOptions {
                width,
                height,
                primary: true,
            },
        );
        self.pixels = None; // recreate lazily
    }
}

pub struct WndClass {
    pub name: String,
    pub wndproc: u32,
    pub background: HBRUSH,
}

fn register_class(machine: &mut Machine, wndclass: WndClass) -> u32 {
    let atom = machine.state.user32.wndclasses.len() as u32 + 1;
    machine.state.user32.wndclasses.push(Rc::new(wndclass));
    atom
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WNDCLASSA {
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: u32,
    lpszMenuName: u32,
    lpszClassName: u32,
}
unsafe impl memory::Pod for WNDCLASSA {}

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

#[win32_derive::dllexport]
pub fn RegisterClassW(machine: &mut Machine, lpWndClass: Option<&WNDCLASSA>) -> u32 {
    // TODO: calling the *W variants tags the windows as expecting wide messages(!).
    let lpWndClass = lpWndClass.unwrap();
    let name =
        unsafe { Str16::from_nul_term_ptr(machine.mem(), lpWndClass.lpszClassName) }.unwrap();
    let background = unsafe { BrushOrColor::from_arg(machine.mem(), lpWndClass.hbrBackground) };
    let wndclass = WndClass {
        name: name.to_string(),
        wndproc: lpWndClass.lpfnWndProc,
        background: background.to_brush(machine),
    };
    register_class(machine, wndclass)
}

#[repr(C, packed)]
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
    hbrBackground: u32,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl memory::Pod for WNDCLASSEXA {}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WNDCLASSEXW {
    cbSize: u32,
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: u32,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl memory::Pod for WNDCLASSEXW {}

#[win32_derive::dllexport]
pub fn RegisterClassExA(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXA>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let name = expect_ascii(machine.mem().slicez(lpWndClassEx.lpszClassName)).to_string();
    let wndclass = WndClass {
        name,
        wndproc: lpWndClassEx.lpfnWndProc,
        background: unsafe { BrushOrColor::from_arg(machine.mem(), lpWndClassEx.hbrBackground) }
            .to_brush(machine),
    };
    register_class(machine, wndclass)
}

#[win32_derive::dllexport]
pub fn RegisterClassExW(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXW>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let name = unsafe { Str16::from_nul_term_ptr(machine.mem(), lpWndClassEx.lpszClassName) }
        .unwrap()
        .to_string();
    let wndclass = WndClass {
        name,
        wndproc: lpWndClassEx.lpfnWndProc,
        background: unsafe { BrushOrColor::from_arg(machine.mem(), lpWndClassEx.hbrBackground) }
            .to_brush(machine),
    };
    register_class(machine, wndclass)
}

bitflags! {
    pub struct WindowStyle: u32 {
        const POPUP           = 0x80000000;
        const CHILD           = 0x40000000;
        const MINIMIZE        = 0x20000000;
        const VISIBLE         = 0x10000000;
        const DISABLED        = 0x08000000;
        const CLIPSIBLINGS    = 0x04000000;
        const CLIPCHILDREN    = 0x02000000;
        const MAXIMIZE        = 0x01000000;
        const BORDER          = 0x00800000;
        const DLGFRAME        = 0x00400000;
        const VSCROLL         = 0x00200000;
        const HSCROLL         = 0x00100000;
        const SYSMENU         = 0x00080000;
        const THICKFRAME      = 0x00040000;
        const GROUP           = 0x00020000;
        const TABSTOP         = 0x00010000;
        const HREDRAW         = 0x00000002; // CS_HREDRAW
        const VREDRAW         = 0x00000001; // CS_VREDRAW
    }
}
impl TryFrom<u32> for WindowStyle {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        WindowStyle::from_bits(value).ok_or(value)
    }
}

bitflags! {
    pub struct WindowStyleEx: u32 {
        // todo
    }
}
impl TryFrom<u32> for WindowStyleEx {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        WindowStyleEx::from_bits(value).ok_or(value)
    }
}

#[derive(Debug)]
pub enum CreateWindowClassName<'a, Str: ?Sized> {
    Atom(u16),
    Name(&'a Str),
}
impl<'a> crate::winapi::stack_args::FromArg<'a> for CreateWindowClassName<'a, str> {
    unsafe fn from_arg(mem: memory::Mem<'a>, arg: u32) -> Self {
        if arg <= 0xFFFF {
            CreateWindowClassName::Atom(arg as u16)
        } else {
            CreateWindowClassName::Name(<Option<&str>>::from_arg(mem, arg).unwrap())
        }
    }
}
impl<'a> crate::winapi::stack_args::FromArg<'a> for CreateWindowClassName<'a, Str16> {
    unsafe fn from_arg(mem: memory::Mem<'a>, arg: u32) -> Self {
        if arg <= 0xFFFF {
            CreateWindowClassName::Atom(arg as u16)
        } else {
            CreateWindowClassName::Name(<Option<&Str16>>::from_arg(mem, arg).unwrap())
        }
    }
}

#[win32_derive::dllexport]
pub async fn CreateWindowExA(
    machine: &mut Machine,
    dwExStyle: Result<WindowStyleEx, u32>,
    lpClassName: CreateWindowClassName<'_, str>,
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
    let class_name_wide: String16;
    let class_name = match lpClassName {
        CreateWindowClassName::Name(name) => {
            class_name_wide = String16::from(name);
            CreateWindowClassName::Name(class_name_wide.as_str16())
        }
        CreateWindowClassName::Atom(a) => CreateWindowClassName::Atom(a),
    };
    let window_name = String16::from(lpWindowName.unwrap_or(""));
    CreateWindowExW(
        machine,
        dwExStyle,
        class_name,
        Some(window_name.as_str16()),
        dwStyle,
        X,
        Y,
        nWidth,
        nHeight,
        hWndParent,
        hMenu,
        hInstance,
        lpParam,
    )
    .await
}

#[win32_derive::dllexport]
pub async fn CreateWindowExW(
    machine: &mut Machine,
    dwExStyle: Result<WindowStyleEx, u32>,
    lpClassName: CreateWindowClassName<'_, Str16>,
    lpWindowName: Option<&Str16>,
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
    let class_name = match lpClassName {
        CreateWindowClassName::Atom(_) => unimplemented!(),
        CreateWindowClassName::Name(name) => name.to_string(),
    };
    let wndclass = match machine
        .state
        .user32
        .wndclasses
        .iter()
        .find(|c| c.name == class_name)
    {
        Some(wndclass) => wndclass.clone(),
        None => {
            log::warn!("unknown wndclass {class_name:?}, using empty");
            Rc::new(WndClass {
                name: class_name,
                wndproc: 0,
                background: HBRUSH::null(),
            })
        }
    };

    let _style = dwStyle.unwrap();
    const CW_USEDEFAULT: u32 = 0x8000_0000;

    // hInstance is only relevant when multiple DLLs register classes:
    //   https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873

    let hwnd = machine.state.user32.windows.reserve();
    let mut host_win = machine.host.create_window(hwnd.to_raw());
    host_win.set_title(&lpWindowName.unwrap().to_string());
    let width = if nWidth == CW_USEDEFAULT { 640 } else { nWidth };
    let height = if nHeight == CW_USEDEFAULT {
        480
    } else {
        nHeight
    };

    let style = dwStyle.unwrap();
    let menu = false; // TODO
    let (width, height) = client_size_from_window_size(style, menu, width, height);
    host_win.set_size(width, height);
    let surface = machine.host.create_surface(
        hwnd.to_raw(),
        &SurfaceOptions {
            width,
            height,
            primary: true,
        },
    );

    let window = Window {
        hwnd,
        hdc: machine.state.gdi32.dcs.add(crate::winapi::gdi32::DC::new(
            crate::winapi::gdi32::DCTarget::Window(hwnd),
        )),
        host: host_win,
        surface,
        width,
        height,
        wndclass,
        pixels: None,
        dirty: Some(UpdateRegion {
            erase_background: true,
        }),
        style,
    };
    machine.state.user32.windows.set(hwnd, window);

    // Synchronously dispatch WM_CREATE.
    let msg = MSG {
        hwnd,
        message: WM::CREATE as u32,
        wParam: 0,
        lParam: 0, // TODO: CREATESTRUCT
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    };
    dispatch_message(machine, &msg).await;

    hwnd
}

#[win32_derive::dllexport]
pub fn DestroyWindow(_machine: &mut Machine, hWnd: HWND) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetDesktopWindow(_machine: &mut Machine) -> HWND {
    HWND::null()
}

#[win32_derive::dllexport]
pub fn GetForegroundWindow(machine: &mut Machine) -> HWND {
    if let Some(window) = machine.state.user32.windows.iter().next() {
        return window.hwnd;
    }
    GetDesktopWindow(machine)
}

#[win32_derive::dllexport]
pub fn SetForegroundWindow(_machine: &mut Machine, hWnd: HWND) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetActiveWindow(machine: &mut Machine) -> HWND {
    match machine.state.user32.windows.iter().next() {
        Some(w) => w.hwnd,
        None => HWND::null(),
    }
}

#[win32_derive::dllexport]
pub fn GetLastActivePopup(machine: &mut Machine) -> HWND {
    machine.state.user32.windows.iter().next().unwrap().hwnd
}

#[win32_derive::dllexport]
pub fn FindWindowA(
    machine: &mut Machine,
    lpClassName: Option<&str>,
    lpWindowName: Option<&str>,
) -> HWND {
    match machine.state.user32.windows.iter().find(|_window| {
        // TODO: obey class/window name
        true
    }) {
        Some(window) => window.hwnd,
        None => HWND::null(),
    }
}

#[win32_derive::dllexport]
pub async fn UpdateWindow(machine: &mut Machine, hWnd: HWND) -> bool {
    let window = machine.state.user32.windows.get(hWnd).unwrap();
    if window.dirty.is_none() {
        return true;
    }
    let windowpos_addr = machine.state.scratch.alloc(
        machine.emu.memory.mem(),
        std::mem::size_of::<WINDOWPOS>() as u32,
    );

    let msg = MSG {
        hwnd: hWnd,
        message: WM::PAINT as u32,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    };
    DispatchMessageA(machine, Some(&msg)).await;

    dispatch_message(
        machine,
        &MSG {
            hwnd: hWnd,
            message: WM::WINDOWPOSCHANGED as u32,
            wParam: 0,
            lParam: windowpos_addr,
            time: 0,
            pt_x: 0,
            pt_y: 0,
            lPrivate: 0,
        },
    )
    .await;

    true // success
}

/// nCmdShow passed to ShowWindow().
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum SW {
    HIDE = 0,
    NORMAL = 1,
    SHOWMINIMIZED = 2,
    SHOWMAXIMIZED = 3,
    SHOWNOACTIVATE = 4,
    SHOW = 5,
    MINIMIZE = 6,
    SHOWMINNOACTIVE = 7,
    SHOWNA = 8,
    RESTORE = 9,
    SHOWDEFAULT = 10,
    FORCEMINIMIZE = 11,
}

#[win32_derive::dllexport]
pub async fn ShowWindow(machine: &mut Machine, hWnd: HWND, nCmdShow: Result<SW, u32>) -> bool {
    let windowpos_addr = machine.state.scratch.alloc(
        machine.emu.memory.mem(),
        std::mem::size_of::<WINDOWPOS>() as u32,
    );
    dispatch_message(
        machine,
        &MSG {
            hwnd: hWnd,
            message: WM::ACTIVATEAPP as u32,
            wParam: 0,
            lParam: windowpos_addr,
            time: 0,
            pt_x: 0,
            pt_y: 0,
            lPrivate: 0,
        },
    )
    .await;

    dispatch_message(
        machine,
        &MSG {
            hwnd: hWnd,
            message: WM::ACTIVATE as u32,
            wParam: 1,
            lParam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
            lPrivate: 0,
        },
    )
    .await;

    dispatch_message(
        machine,
        &MSG {
            hwnd: hWnd,
            message: WM::WINDOWPOSCHANGED as u32,
            wParam: 0,
            lParam: windowpos_addr,
            time: 0,
            pt_x: 0,
            pt_y: 0,
            lPrivate: 0,
        },
    )
    .await;

    let previously_visible = true;
    previously_visible
}

#[win32_derive::dllexport]
pub fn SetFocus(_machine: &mut Machine, hWnd: HWND) -> HWND {
    let prev_focused = HWND::null();
    prev_focused
}

#[win32_derive::dllexport]
pub fn GetFocus(machine: &mut Machine) -> HWND {
    machine.state.user32.windows.iter().next().unwrap().hwnd
}

#[win32_derive::dllexport]
pub async fn DefWindowProcA(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    let msg = match msg {
        Ok(msg) => msg,
        Err(_) => return 0, // ignore
    };
    match msg {
        WM::PAINT => {
            let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
            window.dirty = None;
        }
        WM::WINDOWPOSCHANGED => {
            let Window { width, height, .. } = *machine.state.user32.windows.get_mut(hWnd).unwrap();
            let WINDOWPOS { flags, .. } = *machine.mem().view::<WINDOWPOS>(lParam);

            if !flags.contains(SWP::NOSIZE) {
                const SIZE_RESTORED: u32 = 0;
                let msg = MSG {
                    hwnd: hWnd,
                    message: WM::SIZE as u32,
                    wParam: SIZE_RESTORED, // TODO: SIZE_* flags
                    lParam: (height << 16) | width,
                    time: 0,
                    pt_x: 0,
                    pt_y: 0,
                    lPrivate: 0,
                };
                dispatch_message(machine, &msg).await;
            }

            if !flags.contains(SWP::NOMOVE) {
                let x = 0; // TODO
                let y = 0;
                let msg = MSG {
                    hwnd: hWnd,
                    message: WM::MOVE as u32,
                    wParam: 0,
                    lParam: (y << 16) | x,
                    time: 0,
                    pt_x: 0,
                    pt_y: 0,
                    lPrivate: 0,
                };
                dispatch_message(machine, &msg).await;
            }
        }
        _ => {}
    }
    0
}

#[win32_derive::dllexport]
pub async fn DefWindowProcW(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    DefWindowProcA(machine, hWnd, msg, wParam, lParam).await
}

/// Compute window rectangle from client rectangle.
fn window_rect(rect: &mut RECT, style: WindowStyle, menu: bool) {
    const CAPTION: i32 = 19;
    rect.top = rect.top - CAPTION;
    rect.left = rect.left;
    rect.right = rect.right;
    rect.bottom = rect.bottom;
    if menu {
        rect.top -= 19;
    }
    if style.contains(WindowStyle::BORDER) {
        const BORDER: i32 = 1;
        rect.top -= BORDER;
        rect.left -= BORDER;
        rect.right += BORDER;
        rect.bottom += BORDER;
    }
    if style.contains(WindowStyle::THICKFRAME) {
        const FRAME: i32 = 4;
        rect.top -= FRAME;
        rect.left -= FRAME;
        rect.right += FRAME;
        rect.bottom += FRAME;
    }
}

fn client_size_from_window_size(
    style: WindowStyle,
    menu: bool,
    width: u32,
    height: u32,
) -> (u32, u32) {
    let mut r = RECT::default();
    window_rect(&mut r, style, menu);
    (
        std::cmp::max(width as i32 - (r.right - r.left), 64) as u32,
        std::cmp::max(height as i32 - (r.bottom - r.top), 64) as u32,
    )
}

#[win32_derive::dllexport]
pub fn AdjustWindowRect(
    machine: &mut Machine,
    lpRect: Option<&mut RECT>,
    dwStyle: Result<WindowStyle, u32>,
    bMenu: bool,
) -> bool {
    AdjustWindowRectEx(
        machine,
        lpRect,
        dwStyle,
        bMenu,
        Result::Ok(WindowStyleEx::empty()),
    )
}

#[win32_derive::dllexport]
pub fn AdjustWindowRectEx(
    _machine: &mut Machine,
    lpRect: Option<&mut RECT>,
    dwStyle: Result<WindowStyle, u32>,
    bMenu: bool,
    dwExStyle: Result<WindowStyleEx, u32>,
) -> bool {
    window_rect(lpRect.unwrap(), dwStyle.unwrap(), bMenu);
    true
}

bitflags! {
    pub struct SWP: u32 {
        const ASYNCWINDOWPOS = 0x4000;
        const DEFERERASE = 0x2000;
        const DRAWFRAME = 0x0020;
        const FRAMECHANGED = 0x0020;
        const HIDEWINDOW = 0x0080;
        const NOACTIVATE = 0x0010;
        const NOCOPYBITS = 0x0100;
        const NOMOVE = 0x0002;
        const NOOWNERZORDER = 0x0200;
        const NOREDRAW = 0x0008;
        const NOREPOSITION = 0x0200;
        const NOSENDCHANGING = 0x0400;
        const NOSIZE = 0x0001;
        const NOZORDER = 0x0004;
        const SHOWWINDOW = 0x0040;
    }
}
impl TryFrom<u32> for SWP {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        SWP::from_bits(value).ok_or(value)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct WINDOWPOS {
    pub hwnd: HWND,
    pub hwndInsertAfter: HWND,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub flags: SWP,
}
unsafe impl memory::Pod for WINDOWPOS {}

#[win32_derive::dllexport]
pub async fn SetWindowPos(
    machine: &mut Machine,
    hWnd: HWND,
    hWndInsertAfter: HWND,
    X: i32,
    Y: i32,
    cx: i32,
    cy: i32,
    uFlags: Result<SWP, u32>,
) -> bool {
    let windowpos_addr = machine.state.scratch.alloc(
        machine.emu.memory.mem(),
        std::mem::size_of::<WINDOWPOS>() as u32,
    );
    *machine.mem().view_mut::<WINDOWPOS>(windowpos_addr) = WINDOWPOS {
        hwnd: hWnd,
        hwndInsertAfter: hWndInsertAfter,
        x: X,
        y: Y,
        cx,
        cy,
        flags: uFlags.unwrap(),
    };

    // A trace of winstream.exe had this sequence of synchronous messages:
    // WM_WINDOWPOSCHANGING
    // (WM_ACTIVATEAPP, WM_NCACTIVATE, WM_ACTIVATE)
    // WM_WINDOWPOSCHANGED
    // -> DefWindowProc calls WM_SIZE and WM_MOVE
    let msg = MSG {
        hwnd: hWnd,
        message: WM::WINDOWPOSCHANGED as u32,
        wParam: 0,
        lParam: windowpos_addr,
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    };
    dispatch_message(machine, &msg).await;

    true
}

#[win32_derive::dllexport]
pub fn MoveWindow(
    machine: &mut Machine,
    hWnd: HWND,
    X: u32,
    Y: u32,
    nWidth: u32,
    nHeight: u32,
    bRepaint: bool,
) -> bool {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    let menu = true; // TODO
    let (width, height) = client_size_from_window_size(window.style, menu, nWidth, nHeight);
    window.set_client_size(&mut *machine.host, width, height);
    true // success
}

#[win32_derive::dllexport]
pub fn GetClientRect(machine: &mut Machine, hWnd: HWND, lpRect: Option<&mut RECT>) -> bool {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    let rect = lpRect.unwrap();
    *rect = RECT {
        left: 0,
        top: 0,
        right: window.width as i32,
        bottom: window.height as i32,
    };
    true
}

#[win32_derive::dllexport]
pub fn ClientToScreen(_machine: &mut Machine, hWnd: HWND, lpPoint: Option<&mut POINT>) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn GetWindowDC(_machine: &mut Machine, hWnd: HWND) -> HDC {
    HDC::null()
}

#[win32_derive::dllexport]
pub fn ReleaseDC(_machine: &mut Machine, hwnd: HWND, hdc: HDC) -> bool {
    // Note: there is also DeleteDC; this one is specific for some specific DC types...
    false // fail
}

#[win32_derive::dllexport]
pub fn GetWindowLongA(_machine: &mut Machine, hWnd: HWND, nIndex: i32) -> i32 {
    match nIndex {
        // GWL_STYLE
        -16 => WindowStyle::empty().bits() as i32,

        // GWL_EXSTYLE
        -20 => WindowStyleEx::empty().bits() as i32,

        _ => todo!("GetWindowLong({nIndex})"),
    }
}

#[win32_derive::dllexport]
pub fn GetDC(machine: &mut Machine, hWnd: HWND) -> HDC {
    match hWnd.to_option() {
        Some(hWnd) => {
            let window = machine.state.user32.windows.get(hWnd).unwrap();
            window.hdc
        }
        None => machine.state.gdi32.screen_dc,
    }
}

#[win32_derive::dllexport]
pub fn MapWindowPoints(
    _machine: &mut Machine,
    hWndFrom: HWND,
    hWndTo: HWND,
    lpPoints: ArrayWithSize<POINT>,
) -> i32 {
    if !(hWndFrom.is_null() || hWndTo.is_null()) {
        todo!()
    }
    // Mapping a window to/from desktop coords.
    let delta_x = 0;
    let delta_y = 0;
    (delta_y << 16) | delta_x
}

#[win32_derive::dllexport]
pub fn SetCapture(_machine: &mut Machine, hwnd: HWND) -> HWND {
    HWND::null()
}

#[win32_derive::dllexport]
pub fn ReleaseCapture(_machine: &mut Machine) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn SetWindowTextA(machine: &mut Machine, hWnd: HWND, lpString: Option<&str>) -> bool {
    match machine.state.user32.windows.get_mut(hWnd) {
        Some(window) => {
            window.host.set_title(lpString.unwrap());
            true
        }
        None => {
            log::error!("SetWindowText of non-window?");
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn RegisterWindowMessageW(machine: &mut Machine, lpString: Option<&Str16>) -> u32 {
    let name = lpString.unwrap().to_string();
    machine.state.user32.user_window_message_count += 1;
    WM::USER as u32 + machine.state.user32.user_window_message_count
}
