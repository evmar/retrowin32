use super::{WindowType, HBRUSH, HDC};
use crate::{
    str16::Str16,
    winapi::{
        calling_convention::FromArg,
        gdi32::{
            COLORREF, HGDIOBJ, {self},
        },
        HWND, RECT,
    },
    Machine,
};

#[win32_derive::dllexport]
pub fn InvalidateRect(
    machine: &mut Machine,
    hWnd: HWND,
    lpRect: Option<&RECT>,
    bErase: bool,
) -> bool {
    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    window.add_dirty(bErase);
    true // success
}

#[win32_derive::dllexport]
pub fn ValidateRect(machine: &mut Machine, hWnd: HWND, lpRect: Option<&RECT>) -> bool {
    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    let window = window.expect_toplevel_mut();
    match lpRect {
        Some(_rect) => {
            // TODO: ignored.
        }
        None => window.dirty = None,
    }
    false // fail
}

#[win32_derive::dllexport]
pub fn GetUpdateRect(
    machine: &mut Machine,
    hWnd: HWND,
    lpRect: Option<&mut RECT>,
    bErase: bool,
) -> bool {
    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    let top = window.expect_toplevel_mut();

    if let Some(dirty) = &top.dirty {
        if let Some(rect) = lpRect {
            *rect = RECT {
                left: 0,
                top: 0,
                right: window.width as i32,
                bottom: window.height as i32,
            };
        }
        if bErase {
            todo!(); // need to erase background(?!)
        }
        true
    } else {
        false
    }
}

/// Handle to a region.
pub type HRGN = HGDIOBJ;

#[win32_derive::dllexport]
pub fn InvalidateRgn(machine: &mut Machine, hWnd: HWND, hRgn: HRGN, bErase: bool) -> bool {
    if !hRgn.is_null() {
        todo!("invalidate specific region");
    }
    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    window.add_dirty(bErase);
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
    let rcwindow = machine.state.user32.windows.get(hWnd).unwrap();
    let window = rcwindow.borrow();
    // TODO: take from update region
    let dirty_rect = RECT {
        left: 0,
        top: 0,
        right: window.width as i32,
        bottom: window.height as i32,
    };

    let mut background_drawn = false;

    let WindowType::TopLevel(toplevel) = &window.typ else {
        log::warn!("TODO: BeginPaint for child windows");
        return HDC::null();
    };
    let hdc = machine.state.gdi32.new_window_dc(rcwindow.clone());
    let update = toplevel.dirty.as_ref().unwrap();

    if update.erase_background {
        let background = window.wndclass.borrow().background;
        if let Some(hbrush) = background.clone().to_option() {
            if let gdi32::Object::Brush(brush) = machine.state.gdi32.objects.get(hbrush).unwrap() {
                if let Some(color) = brush.color {
                    drop(window);
                    gdi32::fill_rect(machine, hdc, &dirty_rect, color);
                    background_drawn = true;
                }
            }
        }
    }

    *lpPaint.unwrap() = PAINTSTRUCT {
        hdc,
        fErase: (!background_drawn).into(),
        rcPaint: dirty_rect,
        fRestore: 0,          // reserved
        fIncUpdate: 0,        // reserved
        rgbReserved: [0; 32], // reserved
    };
    hdc
}

#[win32_derive::dllexport]
pub fn EndPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&PAINTSTRUCT>) -> bool {
    let mut window = machine.state.user32.windows.get(hWnd).unwrap().borrow_mut();
    match &mut window.typ {
        WindowType::TopLevel(toplevel) => {
            toplevel.dirty = None;
            window.flush_backing_store(machine.emu.memory.mem());
        }
        _ => {
            log::warn!("TODO: EndPaint for child windows");
        }
    }

    true
}

/// COLOR_xxx for GetSysColor etc.
#[derive(Debug, Eq, PartialEq, win32_derive::TryFromEnum)]
pub enum COLOR {
    SCROLLBAR = 0,
    BACKGROUND = 1,
    ACTIVECAPTION = 2,
    INACTIVECAPTION = 3,
    MENU = 4,
    WINDOW = 5,
    WINDOWFRAME = 6,
    MENUTEXT = 7,
    WINDOWTEXT = 8,
    CAPTIONTEXT = 9,
    ACTIVEBORDER = 10,
    INACTIVEBORDER = 11,
    APPWORKSPACE = 12,
    HIGHLIGHT = 13,
    HIGHLIGHTTEXT = 14,
    BTNFACE = 15,
    BTNSHADOW = 16,
    GRAYTEXT = 17,
    BTNTEXT = 18,
    INACTIVECAPTIONTEXT = 19,
    BTNHIGHLIGHT = 20,
}

#[derive(Debug)]
pub enum BrushOrColor {
    Color(COLOR),
    Brush(HBRUSH),
}

impl<'a> FromArg<'a> for BrushOrColor {
    unsafe fn from_arg(_mem: memory::Mem<'a>, arg: u32) -> Self {
        if arg > 0 && arg < HGDIOBJ::lowest_value() {
            BrushOrColor::Color(COLOR::try_from(arg - 1).unwrap())
        } else {
            BrushOrColor::Brush(HBRUSH::from_raw(arg))
        }
    }
}

impl BrushOrColor {
    pub fn to_brush(&self, machine: &mut Machine) -> HGDIOBJ {
        match self {
            BrushOrColor::Brush(hbr) => *hbr,
            BrushOrColor::Color(c) => {
                let color = match c {
                    COLOR::WINDOW => Some(COLORREF::from_rgb(0xc0, 0xc0, 0xc0)),
                    COLOR::MENU => Some(COLORREF::from_rgb(0xc0, 0xc0, 0xc0)),
                    COLOR::APPWORKSPACE => Some(COLORREF::from_rgb(0x80, 0x80, 0x80)),
                    _ => todo!("{c:?}"),
                };
                machine
                    .state
                    .gdi32
                    .objects
                    .add(gdi32::Object::Brush(gdi32::Brush { color }))
            }
        }
    }
}

#[win32_derive::dllexport]
pub fn FillRect(machine: &mut Machine, hDC: HDC, lprc: Option<&RECT>, hbr: BrushOrColor) -> bool {
    let brush = hbr.to_brush(machine);
    let color = match machine.state.gdi32.objects.get(brush).unwrap() {
        gdi32::Object::Brush(brush) => match brush.color {
            Some(color) => color,
            None => return true,
        },
        _ => unimplemented!(),
    };
    let rect = lprc.unwrap();
    gdi32::fill_rect(machine, hDC, rect, color);
    true
}

#[win32_derive::dllexport]
pub fn FrameRect(_machine: &mut Machine, hDC: HDC, lprc: Option<&RECT>, hbr: HBRUSH) -> bool {
    // TODO
    true
}

#[win32_derive::dllexport]
pub fn DrawTextW(
    _machine: &mut Machine,
    hDC: HDC,
    lpString: Option<&Str16>,
    nCount: i32,
    lpRect: Option<&RECT>,
    uFormat: u32,
) -> i32 {
    log::info!("DrawTextW: {:?}", lpString);
    0
}

#[win32_derive::dllexport]
pub fn InvertRect(_machine: &mut Machine, hDC: HDC, lpr: Option<&RECT>) -> bool {
    todo!()
}
