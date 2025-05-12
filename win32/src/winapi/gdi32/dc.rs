use super::{DCHandles, GDIHandles, HGDIOBJ, R2};
use crate::{
    Machine, System,
    winapi::{
        HANDLE, POINT,
        bitmap::{self, Bitmap, PixelData},
    },
};
use std::{cell::RefCell, rc::Rc};

pub type HDC = HANDLE<DC>;

pub trait DCTarget {
    // used in SelectObject
    fn select_bitmap(&mut self, _bitmap: Rc<RefCell<Bitmap>>) {
        // MSDN: "Bitmaps can only be selected into memory DC's [sic].""
        // But kill the clone attempts to select one onto a window, just before using
        // normal drawing calls to draw the same bitmap onto the same window.
        panic!("select_bitmap not implemented for this target");
    }
    fn get_bitmap(&self, machine: &Machine) -> Rc<RefCell<Bitmap>>;
    fn flush(&self, machine: &Machine);
}

impl DCTarget for Rc<RefCell<Bitmap>> {
    fn get_bitmap(&self, _machine: &Machine) -> Rc<RefCell<Bitmap>> {
        self.clone()
    }

    fn select_bitmap(&mut self, bitmap: Rc<RefCell<Bitmap>>) {
        *self = bitmap;
    }

    fn flush(&self, _machine: &Machine) {
        // In memory only; nothing to flush.
    }
}

pub struct ScreenDCTarget;

/// An empty target device for a DC, placeholder for the screen DC.
impl DCTarget for ScreenDCTarget {
    fn get_bitmap(&self, _machine: &Machine) -> Rc<RefCell<Bitmap>> {
        // We need to return a bitmap here to satisfy CreateCompatibleDC,
        // which only cares about the pixel format.
        let bitmap = Bitmap {
            width: 1,
            height: 1,
            format: bitmap::PixelFormat::RGBA32,
            pixels: PixelData::Ptr(0, 0),
        };
        Rc::new(RefCell::new(bitmap))
    }

    fn flush(&self, _machine: &Machine) {
        // We don't expect any drawing to the screen DC.
        unimplemented!()
    }
}

/// DCs are a combination of some functionality shared across all DC types (e.g. "current position"),
/// and some target-specific functionality made generic via the DCTarget trait.
pub struct DC {
    pub target: Box<dyn DCTarget>,

    pub rop2: R2,
    pub pos: POINT,

    // The SelectObject() API sets a drawing-related field on the DC and returns the
    // previously selected object of a given type, which means we need a storage field
    // per object type.
    pub brush: HGDIOBJ,
    pub pen: HGDIOBJ,
}

impl DC {
    pub fn new(target: Box<dyn DCTarget>) -> Self {
        DC {
            target,
            rop2: R2::default(),
            pos: Default::default(),
            brush: Default::default(),
            pen: Default::default(),
        }
    }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: HDC) -> HDC {
    let format = machine
        .state
        .gdi32
        .dcs
        .get(hdc)
        .unwrap()
        .borrow()
        .target
        .get_bitmap(machine)
        .borrow()
        .format;

    // MSDN says: "When a memory device context is created, it initially has a 1-by-1 monochrome bitmap selected into it."
    // SkiFree depends on this!
    let bitmap = Bitmap {
        width: 1,
        height: 1,
        format,
        pixels: PixelData::Ptr(0, 0),
    };

    let hobj = machine.state.gdi32.objects.add_bitmap(bitmap);
    let bmp = machine
        .state
        .gdi32
        .objects
        .get_bitmap(hobj)
        .unwrap()
        .clone();
    let dc = DC::new(Box::new(bmp));

    let handle = machine.state.gdi32.dcs.add_dc(dc);
    handle
}

#[win32_derive::dllexport]
pub fn DeleteDC(sys: &dyn System, hdc: u32) -> u32 {
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
pub fn GetDeviceCaps(sys: &dyn System, hdc: HDC, index: Result<GetDeviceCapsArg, u32>) -> u32 {
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
pub fn GetLayout(sys: &dyn System, hdc: HDC) -> u32 {
    0 // LTR
}

#[win32_derive::dllexport]
pub fn SetLayout(sys: &dyn System, hdc: HDC, l: u32) -> u32 {
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
