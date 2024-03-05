use super::*;
use crate::{
    host,
    winapi::gdi32::{HDC, HGDIOBJ},
    Host, SurfaceOptions,
};
use bitflags::bitflags;
use std::rc::Rc;

pub struct WindowPixels {
    pub surface: Box<dyn host::Surface>,
    pub raw: Box<[[u8; 4]]>,
}
impl WindowPixels {
    pub fn new(host: &mut dyn Host, width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let surface = host.create_surface(&SurfaceOptions {
            width,
            height,
            primary: true,
        });
        let raw = {
            let mut p = Vec::with_capacity(size);
            p.resize(size, [0, 0, 0, 0]);
            p.into_boxed_slice()
        };
        Self { surface, raw }
    }
}

pub struct Window {
    pub hwnd: HWND,
    pub hdc: HDC,
    pub host: Box<dyn host::Window>,
    pub width: u32,
    pub height: u32,
    pub wndclass: Rc<WndClass>,
    pub pixels: Option<WindowPixels>,
    pub need_paint: bool,
    pub style: WindowStyle,
}

impl Window {
    pub fn pixels_mut(&mut self, host: &mut dyn Host) -> &mut WindowPixels {
        match self.pixels {
            Some(ref mut px) => px,
            None => {
                self.pixels = Some(WindowPixels::new(host, self.width, self.height));
                self.pixels.as_mut().unwrap()
            }
        }
    }

    pub fn flush_pixels(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            pixels.surface.write_pixels(&*pixels.raw);
            pixels.surface.show();
        }
    }

    pub fn set_client_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.host.set_size(width, height);
        self.pixels = None;
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
    hbrBackground: HBRUSH,
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
    let name = unsafe { Str16::from_ptr(machine.mem(), lpWndClass.lpszClassName) }.unwrap();
    let wndclass = WndClass {
        name: name.to_string(),
        wndproc: lpWndClass.lpfnWndProc,
        background: lpWndClass.hbrBackground,
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
    hbrBackground: HGDIOBJ,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl memory::Pod for WNDCLASSEXA {}

#[win32_derive::dllexport]
pub fn RegisterClassExA(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXA>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let name = machine
        .mem()
        .slicez(lpWndClassEx.lpszClassName)
        .unwrap()
        .to_ascii()
        .to_string();
    let wndclass = WndClass {
        name,
        wndproc: lpWndClassEx.lpfnWndProc,
        background: lpWndClassEx.hbrBackground,
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
pub enum CreateWindowClassName<'a> {
    Atom(u16),
    Name(&'a Str16),
}
impl<'a> crate::winapi::stack_args::FromArg<'a> for CreateWindowClassName<'a> {
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
    let class_name_wide: String16;
    let class_name = if lpClassName < 0xFFFF {
        CreateWindowClassName::Atom(lpClassName as u16)
    } else {
        let class_name = machine.mem().slicez(lpClassName).unwrap().to_ascii();
        class_name_wide = String16::from(class_name);
        CreateWindowClassName::Name(class_name_wide.as_str16())
    };
    let window_name = String16::from(lpWindowName.unwrap());
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
    lpClassName: CreateWindowClassName<'_>,
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
    let menu = true; // TODO
    let (width, height) = client_size_from_window_size(style, menu, width, height);
    host_win.set_size(width, height);
    let window = Window {
        hwnd,
        hdc: machine.state.gdi32.dcs.add(crate::winapi::gdi32::DC::new(
            crate::winapi::gdi32::DCTarget::Window(hwnd),
        )),
        host: host_win,
        width,
        height,
        wndclass,
        pixels: None,
        need_paint: true,
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
    DispatchMessageA(machine, Some(&msg)).await;

    // Enqueue WM_ACTIVATEAPP message.
    machine.state.user32.messages.push_back(MSG {
        hwnd,
        message: WM::ACTIVATEAPP as u32,
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
    machine.state.user32.windows.iter().next().unwrap().hwnd
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
    machine.state.user32.windows.iter().next().unwrap().hwnd
}

#[win32_derive::dllexport]
pub async fn UpdateWindow(machine: &mut Machine, hWnd: HWND) -> bool {
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
pub fn ShowWindow(_machine: &mut Machine, hWnd: HWND, nCmdShow: Result<SW, u32>) -> bool {
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
pub fn DefWindowProcA(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    if let Ok(msg) = msg {
        match msg {
            WM::PAINT => {
                let window = machine.state.user32.get_window(hWnd).unwrap();
                window.need_paint = false;
            }
            _ => {}
        }
    }
    0
}

#[win32_derive::dllexport]
pub fn DefWindowProcW(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    DefWindowProcA(machine, hWnd, msg, wParam, lParam)
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

#[win32_derive::dllexport]
pub fn SetWindowPos(
    _machine: &mut Machine,
    hWnd: HWND,
    hWndInsertAfter: HWND,
    X: i32,
    Y: i32,
    cx: i32,
    cy: i32,
    uFlags: u32,
) -> bool {
    false
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
    let window = machine.state.user32.get_window(hWnd).unwrap();
    let menu = true; // TODO
    let (width, height) = client_size_from_window_size(window.style, menu, nWidth, nHeight);
    window.set_client_size(width, height);
    true // success
}

#[win32_derive::dllexport]
pub fn GetClientRect(_machine: &mut Machine, hWnd: HWND, lpRect: Option<&mut RECT>) -> bool {
    let rect = lpRect.unwrap();
    *rect = RECT {
        left: 0,
        top: 0,
        right: 640, // TODO: use actual window size
        bottom: 480,
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
        None => {
            log::warn!("TODO: full screen hdc");
            HDC::null()
        }
    }
}

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
