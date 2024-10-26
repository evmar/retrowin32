use super::{HGDIOBJ, R2};
use crate::{
    machine::Machine,
    winapi::{
        bitmap::{Bitmap, PixelData, PixelFormat},
        types::{HANDLE, POINT},
        user32::Window,
    },
};
use std::{cell::RefCell, rc::Rc};

pub type HDC = HANDLE<DC>;

/// Target device for a DC.
#[derive(Clone)]
pub enum DCTarget {
    Memory(Rc<RefCell<Bitmap>>),
    DesktopWindow,
    Window(Rc<RefCell<Window>>),
    DirectDrawSurface(u32),
}

impl DCTarget {
    /// If this target is backed by a bitmap, return it.
    pub fn get_bitmap(&self) -> Option<Rc<RefCell<Bitmap>>> {
        match self {
            DCTarget::Memory(bitmap) => Some(bitmap.clone()),
            DCTarget::Window(window) => {
                let window = window.borrow();
                return Some(window.bitmap().clone());
            }
            _ => {
                log::warn!("no bitmap found in DC");
                None
            }
        }
    }

    pub fn flush(&self, machine: &Machine) {
        match self {
            DCTarget::Window(window) => {
                let mut window = window.borrow_mut();
                window.flush_backing_store(machine.emu.memory.mem());
            }
            _ => {}
        }
    }
}

pub struct DC {
    // TODO: it's unclear to me what the representation of a DC ought to be.
    // DirectDraw can also create a DC, and DirectDraw (as a DLL that came
    // later) can't retrofit the DC type with a DirectDraw field.
    // Wine appears to use a vtable (for generic behavior).
    pub target: DCTarget,

    pub rop2: R2,
    pub pos: POINT,

    // The SelectObject() API sets a drawing-related field on the DC and returns the
    // previously selected object of a given type, which means we need a storage field
    // per object type.
    pub bitmap: HGDIOBJ,
    pub brush: HGDIOBJ,
    pub pen: HGDIOBJ,
}

impl DC {
    pub fn new(target: DCTarget) -> Self {
        DC {
            target,
            rop2: R2::default(),
            pos: Default::default(),
            bitmap: Default::default(),
            brush: Default::default(),
            pen: Default::default(),
        }
    }

    pub fn new_memory(machine: &mut Machine) -> Self {
        // MSDN says: "When a memory device context is created, it initially has a 1-by-1 monochrome bitmap selected into it."
        // SkiFree depends on this!
        let bitmap = Bitmap {
            width: 1,
            height: 1,
            format: PixelFormat::Mono,
            pixels: PixelData::Ptr(0, 0),
        };
        let hobj = machine.state.gdi32.objects.add_bitmap(bitmap);
        let mut dc = DC::new(DCTarget::Memory(
            machine
                .state
                .gdi32
                .objects
                .get_bitmap(hobj)
                .unwrap()
                .clone(),
        ));
        dc.bitmap = hobj;
        dc
    }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: HDC) -> HDC {
    let dc = DC::new_memory(machine);
    let handle = machine.state.gdi32.dcs.add_dc(dc);
    handle
}

#[win32_derive::dllexport]
pub fn DeleteDC(_machine: &mut Machine, hdc: u32) -> u32 {
    log::warn!("todo: DeleteDC({hdc:x})");
    0 // fail
}

#[derive(Debug, win32_derive::TryFromEnum)]
#[repr(u32)]
pub enum GetDeviceCapsArg {
    DRIVERVERSION = 0,
    TECHNOLOGY = 2,
    HORZSIZE = 4,
    VERTSIZE = 6,
    HORZRES = 8,
    VERTRES = 10,
    BITSPIXEL = 12,
    PLANES = 14,
    NUMBRUSHES = 16,
    NUMPENS = 18,
    NUMMARKERS = 20,
    NUMFONTS = 22,
    NUMCOLORS = 24,
    PDEVICESIZE = 26,
    CURVECAPS = 28,
    LINECAPS = 30,
    POLYGONALCAPS = 32,
    TEXTCAPS = 34,
    CLIPCAPS = 36,
    RASTERCAPS = 38,
    ASPECTX = 40,
    ASPECTY = 42,
    ASPECTXY = 44,
    LOGPIXELSX = 88,
    LOGPIXELSY = 90,
    SIZEPALETTE = 104,
    NUMRESERVED = 106,
    COLORRES = 108,
    PHYSICALWIDTH = 110,
    PHYSICALHEIGHT = 111,
    PHYSICALOFFSETX = 112,
    PHYSICALOFFSETY = 113,
    SCALINGFACTORX = 114,
    SCALINGFACTORY = 115,
    VREFRESH = 116,
    DESKTOPVERTRES = 117,
    DESKTOPHORZRES = 118,
    BLTALIGNMENT = 119,
}

#[win32_derive::dllexport]
pub fn GetDeviceCaps(
    _machine: &mut Machine,
    hdc: HDC,
    index: Result<GetDeviceCapsArg, u32>,
) -> u32 {
    use GetDeviceCapsArg::*;
    match index.unwrap() {
        HORZRES => 640,
        VERTRES => 480,
        NUMCOLORS => -1i32 as u32, // true color
        RASTERCAPS => 0,           // none
        LOGPIXELSX => 96,
        LOGPIXELSY => 96,
        SIZEPALETTE => 0,
        i => unimplemented!("{i:?}"),
    }
}

#[win32_derive::dllexport]
pub fn GetLayout(_machine: &mut Machine, hdc: HDC) -> u32 {
    0 // LTR
}

#[win32_derive::dllexport]
pub fn SetLayout(_machine: &mut Machine, hdc: HDC, l: u32) -> u32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetDCOrgEx(machine: &mut Machine, hdc: HDC, lpPoint: Option<&mut POINT>) -> bool {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap().borrow();
    if let Some(lpPoint) = lpPoint {
        *lpPoint = dc.pos;
        return true;
    }
    false
}
