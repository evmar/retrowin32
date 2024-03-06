//! Pens, brushes, color.

use super::*;
use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "gdi32/draw";

#[derive(Debug)]
pub struct COLORREF(pub (u8, u8, u8));
impl COLORREF {
    pub fn from_u32(raw: u32) -> Self {
        Self((raw as u8, (raw >> 8) as u8, (raw >> 16) as u8))
    }
    pub fn to_pixel(&self) -> [u8; 4] {
        let (r, g, b) = self.0;
        [r, g, b, 0xff]
    }
}

#[derive(Debug)]
pub struct Pen {
    pub color: COLORREF,
}

#[derive(Debug)]
pub struct Brush {
    pub color: COLORREF,
}

#[win32_derive::dllexport]
pub fn SetBkMode(_machine: &mut Machine, hdc: HDC, mode: i32) -> i32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn SetBkColor(_machine: &mut Machine, hdc: HDC, color: u32) -> u32 {
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
    color: u32,
) -> HGDIOBJ {
    iStyle.unwrap();
    if cWidth != 1 {
        todo!();
    }

    machine.state.gdi32.objects.add(Object::Pen(Pen {
        color: COLORREF::from_u32(color),
    }))
}

#[win32_derive::dllexport]
pub fn MoveToEx(machine: &mut Machine, hdc: HDC, x: u32, y: u32, lppt: Option<&mut POINT>) -> bool {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    if let Some(pt) = lppt {
        *pt = POINT { x: dc.x, y: dc.y };
    }
    dc.x = x;
    dc.y = y;
    true
}

fn ascending(a: u32, b: u32) -> (u32, u32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[win32_derive::dllexport]
pub fn LineTo(machine: &mut Machine, hdc: HDC, x: u32, y: u32) -> bool {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    let hwnd = match dc.target {
        DCTarget::Memory(_) => todo!(),
        DCTarget::Window(hwnd) => hwnd,
        DCTarget::DirectDrawSurface(_) => todo!(),
    };
    let window = machine.state.user32.windows.get_mut(hwnd).unwrap();
    let stride = window.width;
    let pixels = window.pixels_mut(&mut *machine.host);

    let color = match dc.r2 {
        R2::COPYPEN => match machine.state.gdi32.objects.get(dc.pen).unwrap() {
            Object::Pen(pen) => pen.color.to_pixel(),
            _ => todo!(),
        },
        R2::WHITE => [0xff, 0xff, 0xff, 0],
    };

    let (dstX, dstY) = (x, y);
    if dstX == dc.x {
        let (y0, y1) = ascending(dstY, dc.y);
        for y in y0..=y1 {
            pixels[((y * stride) + x) as usize] = color;
        }
        dc.y = dstY;
    } else if dstY == dc.y {
        let (x0, x1) = ascending(dstX, dc.x);
        for x in x0..=x1 {
            pixels[((y * stride) + x) as usize] = color;
        }
        dc.x = dstX;
    } else {
        todo!();
    }
    false // fail
}

#[derive(Debug, Default, win32_derive::TryFromEnum)]
pub enum R2 {
    #[default]
    COPYPEN = 13,
    WHITE = 16,
}

#[win32_derive::dllexport]
pub fn SetROP2(machine: &mut Machine, hdc: HDC, rop2: Result<R2, u32>) -> u32 {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    std::mem::replace(&mut dc.r2, rop2.unwrap()) as u32
}
