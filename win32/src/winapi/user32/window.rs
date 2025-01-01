use super::*;
use crate::{
    host,
    winapi::{
        bitmap::{Bitmap, PixelData, PixelFormat},
        gdi32::HDC,
        stack_args::ArrayWithSize,
        types::{Str16, String16, HWND, POINT, RECT},
    },
    Host, Machine, SurfaceOptions,
};
use bitflags::bitflags;
use memory::{Extensions, ExtensionsMut, Mem};
use std::{cell::RefCell, rc::Rc};

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

bitflags! {
    /// Window styles.
    #[derive(win32_derive::TryFromBitflags)]
    pub struct WS: u32 {
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

bitflags! {
    /// Extended window styles.
    #[derive(win32_derive::TryFromBitflags)]
    pub struct WS_EX: u32 {
        // todo
    }
}

pub struct Window {
    /// Identity for tying to Surfaces in the host.
    // TODO: make create_surface a method on Window and remove this.
    pub id: u32,
    pub typ: WindowType,
    /// Client area width (not total window width).
    pub width: u32,
    /// Client area height (not total window height).
    pub height: u32,
    pub wndclass: Rc<RefCell<WndClass>>,
    pub style: WS,
    /// The current show state of the window.
    pub show_cmd: SW,
}

pub enum WindowType {
    TopLevel(WindowTopLevel),
    Child,
}

/// Properties of only top-level windows.
pub struct WindowTopLevel {
    pub host: Box<dyn host::Window>,
    surface: Box<dyn host::Surface>,
    // TODO: CS_OWNDC windows do own a DC, but otherwise they don't.
    // pub hdc: HDC,
    /// Backing store.
    /// Rc so it can be shared within drawing functions.
    backing_store: Rc<RefCell<Bitmap>>,
    pub dirty: Option<Dirty>,
}

impl Window {
    // TODO: expect_toplevel was added to introduce child windows,
    // but many callers just need to handle child windows instead of calling these.
    pub fn expect_toplevel_mut(&mut self) -> &mut WindowTopLevel {
        match &mut self.typ {
            WindowType::TopLevel(win) => win,
            _ => panic!("expected top-level window, see TODO"),
        }
    }

    pub fn bitmap(&self) -> &Rc<RefCell<Bitmap>> {
        match &self.typ {
            WindowType::TopLevel(win) => &win.backing_store,
            _ => panic!("expected top-level window, see TODO"),
        }
    }

    pub fn set_client_size(&mut self, host: &mut dyn Host, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        match &mut self.typ {
            WindowType::TopLevel(w) => {
                w.set_size(host, self.id, width, height);
            }
            _ => {}
        }
    }

    pub fn is_dirty(&self) -> bool {
        match &self.typ {
            WindowType::TopLevel(top) => top.dirty.is_some(),
            _ => false, // TODO
        }
    }

    // TODO: update region
    pub fn add_dirty(&mut self, erase: bool) {
        match &mut self.typ {
            WindowType::TopLevel(top) => {
                let prev = top
                    .dirty
                    .as_ref()
                    .map(|d| d.erase_background)
                    .unwrap_or(false);
                top.dirty = Some(Dirty {
                    erase_background: prev || erase,
                });
            }
            _ => todo!(),
        }
    }

    pub fn flush_backing_store(&mut self, mem: Mem) {
        match self.typ {
            WindowType::TopLevel(ref mut top) => {
                top.flush_backing_store(mem);
            }
            _ => {}
        }
    }
}

pub struct Dirty {
    pub erase_background: bool,
    // TODO: region
}

impl WindowTopLevel {
    fn new(host: &mut dyn Host, hwnd: HWND, title: &str, width: u32, height: u32) -> Self {
        let host_win = host.create_window(hwnd.to_raw());
        host_win.set_title(title);
        host_win.set_size(width, height);

        let surface = host.create_surface(
            hwnd.to_raw(),
            &SurfaceOptions {
                width,
                height,
                bytes_per_pixel: 4,
                primary: true,
            },
        );
        WindowTopLevel {
            host: host_win,
            surface,
            backing_store: Self::create_backing_store(width, height),
            dirty: Some(Dirty {
                erase_background: true,
            }),
        }
    }

    fn set_size(&mut self, host: &mut dyn Host, id: u32, width: u32, height: u32) {
        self.host.set_size(width, height);
        self.surface = host.create_surface(
            id,
            &host::SurfaceOptions {
                width,
                height,
                bytes_per_pixel: 4,
                primary: true,
            },
        );
        self.backing_store = WindowTopLevel::create_backing_store(width, height);
    }

    fn create_backing_store(width: u32, height: u32) -> Rc<RefCell<Bitmap>> {
        Rc::new(RefCell::new(Bitmap {
            width,
            height,
            format: PixelFormat::RGBA32,
            pixels: PixelData::new_owned((width * height * 4) as usize),
        }))
    }

    fn flush_backing_store(&mut self, mem: Mem) {
        let backing_store = self.backing_store.borrow();
        let bytes = backing_store.pixels.bytes(mem);
        self.surface.write_pixels(bytes);

        self.surface.show();
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
    dwExStyle: Result<WS_EX, u32>,
    lpClassName: CreateWindowClassName<'_, str>,
    lpWindowName: Option<&str>,
    dwStyle: Result<WS, u32>,
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
    dwExStyle: Result<WS_EX, u32>,
    lpClassName: CreateWindowClassName<'_, Str16>,
    lpWindowName: Option<&Str16>,
    dwStyle: Result<WS, u32>,
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
        .find(|c| c.borrow().name == class_name)
        .unwrap()
        .clone();

    let style = dwStyle.unwrap();
    const CW_USEDEFAULT: u32 = 0x8000_0000;

    // hInstance is only relevant when multiple DLLs register classes:
    //   https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873

    let hwnd = machine.state.user32.windows.reserve();
    let width = if nWidth == CW_USEDEFAULT { 640 } else { nWidth };
    let height = if nHeight == CW_USEDEFAULT {
        480
    } else {
        nHeight
    };

    let style = dwStyle.unwrap();
    let menu = false; // TODO
    let (width, height) = client_size_from_window_size(style, menu, width, height);

    let typ = if style.contains(WS::CHILD) {
        WindowType::Child
    } else {
        WindowType::TopLevel(WindowTopLevel::new(
            &mut *machine.host,
            hwnd,
            &lpWindowName.unwrap().to_string(),
            width,
            height,
        ))
    };

    let window = Window {
        id: hwnd.to_raw(),
        typ,
        width,
        height,
        wndclass,
        style,
        show_cmd: SW::HIDE,
    };
    machine
        .state
        .user32
        .windows
        .set(hwnd, Rc::new(RefCell::new(window)));

    // Synchronously dispatch WM_CREATE.
    let msg = MSG {
        hwnd,
        message: WM::CREATE as u32,
        wParam: 0,
        lParam: 0, // TODO: CREATESTRUCT
        time: 0,
        pt_x: 0,
        pt_y: 0,
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
    if let Some((hwnd, _)) = machine.state.user32.windows.iter().next() {
        return hwnd;
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
        Some((hwnd, _)) => hwnd,
        None => HWND::null(),
    }
}

#[win32_derive::dllexport]
pub fn GetLastActivePopup(machine: &mut Machine) -> HWND {
    machine.state.user32.windows.iter().next().unwrap().0
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
        Some((hwnd, _)) => hwnd,
        None => HWND::null(),
    }
}

#[win32_derive::dllexport]
pub async fn UpdateWindow(machine: &mut Machine, hWnd: HWND) -> bool {
    let window = machine.state.user32.windows.get(hWnd).unwrap().borrow();
    match &window.typ {
        WindowType::TopLevel(top) => {
            if top.dirty.is_none() {
                return true;
            }
        }
        _ => {
            log::warn!("TODO: UpdateWindow for child windows");
        }
    }
    drop(window);

    let msg = MSG {
        hwnd: hWnd,
        message: WM::PAINT as u32,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };
    dispatch_message(machine, &msg).await;

    true // success
}

bitflags! {
    /// RedrawWindow behavior flags.
    #[derive(win32_derive::TryFromBitflags)]
    pub struct RDW: u32 {
        const INVALIDATE      = 0x0001;
        const INTERNALPAINT   = 0x0002;
        const ERASE           = 0x0004;

        const VALIDATE        = 0x0008;
        const NOINTERNALPAINT = 0x0010;
        const NOERASE         = 0x0020;

        const NOCHILDREN      = 0x0040;
        const ALLCHILDREN     = 0x0080;

        const UPDATENOW       = 0x0100;
        const ERASENOW        = 0x0200;

        const FRAME           = 0x0400;
        const NOFRAME         = 0x0800;
    }
}

#[win32_derive::dllexport]
pub async fn RedrawWindow(
    machine: &mut Machine,
    hWnd: HWND,
    lprcUpdate: Option<&mut RECT>,
    hrgnUpdate: HRGN,
    flags: Result<RDW, u32>,
) -> bool {
    if lprcUpdate.is_some() || !hrgnUpdate.is_null() {
        todo!();
    }

    // TODO: this function has a million flags, ugh.
    // Seems like it's three steps: invalidate/validate, update.

    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    let flags = flags.unwrap();

    if flags.contains(RDW::INVALIDATE) {
        window.add_dirty(flags.contains(RDW::ERASE));
    } else if flags.contains(RDW::VALIDATE) {
        todo!();
    }
    drop(window);

    if flags.contains(RDW::ERASENOW) {
        todo!();
    } else if flags.contains(RDW::UPDATENOW) {
        let msg = MSG {
            hwnd: hWnd,
            message: WM::PAINT as u32,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
        };
        dispatch_message(machine, &msg).await;
    }

    true
}

/// nCmdShow passed to ShowWindow().
#[derive(Copy, Clone, Debug, win32_derive::TryFromEnum)]
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
    // Store the show command for returning from GetWindowPlacement.
    machine
        .state
        .user32
        .windows
        .get(hWnd)
        .unwrap()
        .borrow_mut()
        .show_cmd = nCmdShow.unwrap();

    dispatch_message(
        machine,
        &MSG {
            hwnd: hWnd,
            message: WM::ACTIVATEAPP as u32,
            wParam: true as u32, // activating
            lParam: 0,           // TODO: thread id
            time: 0,
            pt_x: 0,
            pt_y: 0,
        },
    )
    .await;

    const WA_ACTIVE: u32 = 1;
    dispatch_message(
        machine,
        &MSG {
            hwnd: hWnd,
            message: WM::ACTIVATE as u32,
            wParam: WA_ACTIVE,
            lParam: 0, // TODO: previous window hwnd
            time: 0,
            pt_x: 0,
            pt_y: 0,
        },
    )
    .await;

    // TODO: WM_WINDOWPOSCHANGED should pass a WINDOWPOS struct,
    // but the DefWindowProc we provide ignores it and calls WM_MOVE/WM_SIZE directly.
    let windowpos_addr = 0;
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
    machine.state.user32.windows.iter().next().unwrap().0
}

async fn def_window_proc(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    _wParam: u32,
    lParam: u32,
) -> u32 {
    let msg = match msg {
        Ok(msg) => msg,
        Err(_) => return 0, // ignore
    };
    match msg {
        WM::PAINT => {
            let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
            let window = window.expect_toplevel_mut();
            window.dirty = None;
        }
        WM::WINDOWPOSCHANGED => {
            let Window { width, height, .. } =
                *machine.state.user32.windows.get(hWnd).unwrap().borrow();
            let WINDOWPOS { flags, .. } = machine.mem().get_pod::<WINDOWPOS>(lParam);

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
                };
                dispatch_message(machine, &msg).await;
            }
        }
        _ => {}
    }
    0
}

#[win32_derive::dllexport]
pub async fn DefWindowProcA(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    def_window_proc(machine, hWnd, msg, wParam, lParam).await
}

#[win32_derive::dllexport]
pub async fn DefWindowProcW(
    machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    def_window_proc(machine, hWnd, msg, wParam, lParam).await
}

/// Compute window rectangle from client rectangle.
fn window_rect(rect: &mut RECT, style: WS, menu: bool) {
    const CAPTION: i32 = 19;
    rect.top = rect.top - CAPTION;
    rect.left = rect.left;
    rect.right = rect.right;
    rect.bottom = rect.bottom;
    if menu {
        rect.top -= 19;
    }
    if style.contains(WS::BORDER) {
        const BORDER: i32 = 1;
        rect.top -= BORDER;
        rect.left -= BORDER;
        rect.right += BORDER;
        rect.bottom += BORDER;
    }
    if style.contains(WS::THICKFRAME) {
        const FRAME: i32 = 4;
        rect.top -= FRAME;
        rect.left -= FRAME;
        rect.right += FRAME;
        rect.bottom += FRAME;
    }
}

fn client_size_from_window_size(style: WS, menu: bool, width: u32, height: u32) -> (u32, u32) {
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
    dwStyle: Result<WS, u32>,
    bMenu: bool,
) -> bool {
    AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, Result::Ok(WS_EX::empty()))
}

#[win32_derive::dllexport]
pub fn AdjustWindowRectEx(
    _machine: &mut Machine,
    lpRect: Option<&mut RECT>,
    dwStyle: Result<WS, u32>,
    bMenu: bool,
    dwExStyle: Result<WS_EX, u32>,
) -> bool {
    window_rect(lpRect.unwrap(), dwStyle.unwrap(), bMenu);
    true
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
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

#[repr(C)]
#[derive(Debug, Clone)]
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
    machine.mem().put_pod::<WINDOWPOS>(
        windowpos_addr,
        WINDOWPOS {
            hwnd: hWnd,
            hwndInsertAfter: hWndInsertAfter,
            x: X,
            y: Y,
            cx,
            cy,
            flags: uFlags.unwrap(),
        },
    );

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
    };
    dispatch_message(machine, &msg).await;

    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    let menu = true; // TODO
    let (width, height) = client_size_from_window_size(window.style, menu, cx as u32, cy as u32);
    window.set_client_size(&mut *machine.host, width, height);

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
    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    let menu = true; // TODO
    let (width, height) = client_size_from_window_size(window.style, menu, nWidth, nHeight);
    window.set_client_size(&mut *machine.host, width, height);
    true // success
}

#[win32_derive::dllexport]
pub fn GetClientRect(machine: &mut Machine, hWnd: HWND, lpRect: Option<&mut RECT>) -> bool {
    let window = machine.state.user32.windows.get(hWnd).unwrap().borrow();
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
pub fn GetWindowRect(machine: &mut Machine, hWnd: HWND, lpRect: Option<&mut RECT>) -> bool {
    let window = machine.state.user32.windows.get(hWnd).unwrap().borrow();

    let mut result = RECT {
        left: 0,
        top: 0,
        right: window.width as i32,
        bottom: window.height as i32,
    };

    let menu = true; // TODO
    window_rect(&mut result, window.style, menu);

    // TODO: this pretends that the window is at 0,0
    let offset_x = -result.left;
    let offset_y = -result.top;

    result.left += offset_x;
    result.right += offset_x;

    result.top += offset_y;
    result.bottom += offset_y;

    let rect = lpRect.unwrap();
    *rect = result;
    true
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WINDOWPLACEMENT {
    length: u32,
    flags: u32,
    showCmd: u32,
    ptMinPosition: POINT,
    ptMaxPosition: POINT,
    rcNormalPosition: RECT,
}
unsafe impl memory::Pod for WINDOWPLACEMENT {}

#[win32_derive::dllexport]
pub fn GetWindowPlacement(
    machine: &mut Machine,
    hWnd: HWND,
    lpwndpl: Option<&mut WINDOWPLACEMENT>,
) -> bool {
    let window = machine.state.user32.windows.get(hWnd).unwrap().borrow();
    let wndpl = lpwndpl.unwrap();

    *wndpl = WINDOWPLACEMENT {
        length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
        flags: 0,
        showCmd: window.show_cmd as u32,
        ptMinPosition: POINT { x: 0, y: 0 },
        ptMaxPosition: POINT { x: 0, y: 0 },
        rcNormalPosition: RECT {
            left: 0,
            top: 0,
            right: window.width as i32,
            bottom: window.height as i32,
        },
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
pub fn ReleaseDC(machine: &mut Machine, hwnd: HWND, hdc: HDC) -> bool {
    // Note: there is also DeleteDC; this one is specifically for GetWindowDC/GetDC.
    // TODO: there is a separate refcount for ReleaseDC, but we don't track it.
    true
}

#[win32_derive::dllexport]
pub fn GetWindowLongA(_machine: &mut Machine, hWnd: HWND, nIndex: i32) -> i32 {
    match nIndex {
        // GWL_STYLE
        -16 => WS::empty().bits() as i32,

        // GWL_EXSTYLE
        -20 => WS_EX::empty().bits() as i32,

        _ => todo!("GetWindowLong({nIndex})"),
    }
}

#[win32_derive::dllexport]
pub fn SetWindowLongA(
    _machine: &mut Machine,
    hWnd: HWND,
    nIndex: u32, /* WINDOW_LONG_PTR_INDEX */
    dwNewLong: i32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDC(machine: &mut Machine, hWnd: HWND) -> HDC {
    match hWnd.to_option() {
        Some(hwnd) => {
            let rcwindow = machine.state.user32.windows.get(hwnd).unwrap();
            let window = rcwindow.borrow();
            match &window.typ {
                WindowType::TopLevel(_) => machine.state.gdi32.new_window_dc(rcwindow.clone()),
                _ => {
                    log::warn!("GetDC for non-top-level window");
                    HDC::null()
                }
            }
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
            let mut window = window.borrow_mut();
            window
                .expect_toplevel_mut()
                .host
                .set_title(lpString.unwrap());
            true
        }
        None => {
            log::error!("SetWindowText of non-window?");
            false
        }
    }
}

/// RegisterWindowMessage returns a unique message ID starting at this value.
const USER_WINDOW_MESSAGE_BASE: u32 = 0xC000;

#[win32_derive::dllexport]
pub fn RegisterWindowMessageA(machine: &mut Machine, lpString: Option<&str>) -> u32 {
    let name = lpString.unwrap().to_string();
    machine.state.user32.user_window_message_count += 1;
    USER_WINDOW_MESSAGE_BASE + machine.state.user32.user_window_message_count
}

#[win32_derive::dllexport]
pub fn RegisterWindowMessageW(machine: &mut Machine, lpString: Option<&Str16>) -> u32 {
    let name = lpString.unwrap().to_string();
    machine.state.user32.user_window_message_count += 1;
    USER_WINDOW_MESSAGE_BASE + machine.state.user32.user_window_message_count
}

#[win32_derive::dllexport]
pub fn GetCapture(machine: &mut Machine) -> HWND {
    todo!();
}

#[win32_derive::dllexport]
pub fn EnableWindow(_machine: &mut Machine, hWnd: HWND, bEnable: bool) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn IsWindowVisible(_machine: &mut Machine, hWnd: HWND) -> bool {
    true
}
