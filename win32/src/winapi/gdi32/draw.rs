//! Pens, brushes, color.

use super::{DCTarget, Object, HDC, HGDIOBJ};
use crate::{
    machine::Machine,
    winapi::{POINT, RECT},
};

/// COLORREF is a u32 containing RGB0, modeled specially here because there is the
/// invalid marker value CLR_INVALID=0xffffffff.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct COLORREF(u32);

impl COLORREF {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(u32::from_le_bytes([r, g, b, 0]))
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        let [r, g, b, _] = self.0.to_le_bytes();
        [r, g, b]
    }

    pub fn to_pixel(&self) -> [u8; 4] {
        let [r, g, b, _] = self.0.to_le_bytes();
        [r, g, b, 0xff]
    }

    pub fn white() -> Self {
        Self::from_rgb(0xff, 0xff, 0xff)
    }
}

impl std::fmt::Debug for COLORREF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == CLR_INVALID {
            return write!(f, "CLR_INVALID");
        }
        let [r, g, b, _] = self.0.to_le_bytes();
        write!(f, "COLORREF(#{r:02x}{g:02x}{b:02x})")
    }
}

impl<'a> crate::calling_convention::FromArg<'a> for COLORREF {
    fn from_arg(_mem: memory::Mem<'a>, arg: u32) -> Self {
        COLORREF(arg)
    }
}

impl Into<crate::calling_convention::ABIReturn> for COLORREF {
    fn into(self) -> crate::calling_convention::ABIReturn {
        self.0.into()
    }
}

pub const CLR_INVALID: COLORREF = COLORREF(0xffff_ffff);

#[derive(Debug)]
pub struct Pen {
    pub color: COLORREF,
}

#[derive(Debug)]
pub struct Brush {
    pub color: Option<COLORREF>,
}

#[win32_derive::dllexport]
pub fn SetBkMode(_machine: &mut Machine, hdc: HDC, mode: i32) -> i32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn SetBkColor(_machine: &mut Machine, hdc: HDC, color: COLORREF) -> COLORREF {
    CLR_INVALID // fail
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum PS {
    SOLID = 0,
}

#[win32_derive::dllexport]
pub fn CreatePen(
    machine: &mut Machine,
    iStyle: Result<PS, u32>,
    cWidth: u32,
    color: COLORREF,
) -> HGDIOBJ {
    iStyle.unwrap();
    if cWidth != 1 {
        todo!();
    }

    machine.state.gdi32.objects.add(Object::Pen(Pen { color }))
}

#[win32_derive::dllexport]
pub fn MoveToEx(machine: &mut Machine, hdc: HDC, x: i32, y: i32, lppt: Option<&mut POINT>) -> bool {
    let mut dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap().borrow_mut();
    if let Some(pt) = lppt {
        *pt = dc.pos;
    }
    dc.pos = POINT { x, y };
    true
}

#[win32_derive::dllexport]
pub fn LineTo(machine: &mut Machine, hdc: HDC, x: i32, y: i32) -> bool {
    let mut dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow_mut();
    let window = match dc.target {
        DCTarget::Memory(_) => todo!(),
        DCTarget::Window(ref window) => window.borrow(),
        _ => todo!(),
    };

    let color = match dc.rop2 {
        R2::COPYPEN => match machine.state.gdi32.objects.get(dc.pen).unwrap() {
            Object::Pen(pen) => pen.color.to_pixel(),
            _ => todo!(),
        },
        R2::WHITE => COLORREF::white().to_pixel(),
    };

    fn ascending(a: i32, b: i32) -> (u32, u32) {
        let a = a.max(0) as u32;
        let b = b.max(0) as u32;
        if a > b {
            (b, a)
        } else {
            (a, b)
        }
    }

    let bitmap = window.bitmap().clone();
    drop(window);
    let mut bitmap = bitmap.borrow_mut();
    let stride = bitmap.width;
    let pixels = bitmap.as_rgba_mut(machine.memory.mem());
    let (dstX, dstY) = (x, y);
    if dstX == dc.pos.x {
        let x = x.max(0) as u32;
        let (y0, y1) = ascending(dstY, dc.pos.y);
        for y in y0..=y1 {
            pixels[((y * stride) + x) as usize] = color;
        }
        dc.pos.y = dstY;
    } else if dstY == dc.pos.y {
        let (x0, x1) = ascending(dstX, dc.pos.x);
        let y = y.max(0) as u32;
        for x in x0..=x1 {
            pixels[((y * stride) + x) as usize] = color;
        }
        dc.pos.x = dstX;
    } else {
        todo!();
    }
    false // fail
}

/// R2_* describe raster ops, as found in SetROP2.
#[derive(Debug, Default, win32_derive::TryFromEnum)]
pub enum R2 {
    #[default]
    COPYPEN = 13,
    WHITE = 16,
}

#[win32_derive::dllexport]
pub fn SetROP2(machine: &mut Machine, hdc: HDC, rop2: Result<R2, u32>) -> u32 {
    let mut dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow_mut();
    std::mem::replace(&mut dc.rop2, rop2.unwrap()) as u32
}

pub fn fill_rect(machine: &mut Machine, hdc: HDC, _rect: &RECT, color: COLORREF) {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();
    match &dc.target {
        DCTarget::Window(window) => {
            let window = window.borrow();
            // TODO: obey rect
            let mut bitmap = window.bitmap().borrow_mut();
            let pixels = bitmap.as_rgba_mut(machine.memory.mem());
            pixels.fill(color.to_pixel());
        }
        _ => todo!(),
    }
    dc.target.clone().flush(machine);
}

#[win32_derive::dllexport]
pub fn SetPixel(machine: &mut Machine, hdc: HDC, x: u32, y: u32, color: COLORREF) -> COLORREF {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();
    match &dc.target {
        DCTarget::Window(window) => {
            let window = window.borrow();
            if x >= window.width || y >= window.height {
                return CLR_INVALID;
            }
            let stride = window.width;
            let mut bitmap = window.bitmap().borrow_mut();
            let pixels = bitmap.as_rgba_mut(machine.memory.mem());
            pixels[((y * stride) + x) as usize] = color.to_pixel();
        }
        _ => {
            log::warn!("TODO: SetPixel unimplemented");
        }
    }

    // TODO: don't need to flush whole window for just one pixel
    dc.target.clone().flush(machine);

    color
}

#[win32_derive::dllexport]
pub fn GetPixel(machine: &mut Machine, hdc: HDC, x: u32, y: u32) -> COLORREF {
    let dc = machine.state.gdi32.dcs.get(hdc).unwrap().borrow();
    match &dc.target {
        DCTarget::Window(window) => {
            let window = window.borrow();
            let stride = window.width;
            let bitmap = window.bitmap().borrow();
            let pixels = bitmap.as_rgba(machine.memory.mem());
            let pixel = pixels[((y * stride) + x) as usize];
            COLORREF::from_rgb(pixel[0], pixel[1], pixel[2])
        }
        _ => {
            // TODO: actually read
            COLORREF::from_rgb(0, 0, 0)
        }
    }
}

#[win32_derive::dllexport]
pub fn CreateSolidBrush(machine: &mut Machine, color: COLORREF) -> HGDIOBJ {
    machine
        .state
        .gdi32
        .objects
        .add(Object::Brush(Brush { color: Some(color) }))
}

#[win32_derive::dllexport]
pub fn SetBrushOrgEx(
    machine: &mut Machine,
    hdc: HDC,
    x: i32,
    y: i32,
    lppt: Option<&mut POINT>,
) -> bool {
    true // stub
}

#[win32_derive::dllexport]
pub fn PtVisible(_machine: &mut Machine, hdc: HDC, x: i32, y: i32) -> bool {
    true // stub
}

#[win32_derive::dllexport]
pub fn LineDDA(
    _machine: &mut Machine,
    xStart: i32,
    yStart: i32,
    xEnd: i32,
    yEnd: i32,
    lpProc: u32,
    data: u32,
) -> bool {
    true // stub
}
