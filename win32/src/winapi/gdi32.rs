#![allow(non_snake_case)]

use crate::{winapi, X86};

pub struct State {
    dcs: Vec<DC>,
}
impl State {
    pub fn new() -> Self {
        State { dcs: Vec::new() }
    }
}

struct DC {}

fn GetStockObject(_x86: &mut X86, _i: u32) -> u32 {
    0
}

fn SelectObject(_x86: &mut X86, _hdc: u32, _hGdiObj: u32) -> u32 {
    // TODO: return previous obj
    0
}

fn GetObjectA(_x86: &mut X86, _handle: u32, _bytes: u32, out: u32) -> u32 {
    0 // fail
}

fn CreateCompatibleDC(x86: &mut X86, hdc: u32) -> u32 {
    assert!(hdc == 0); // null means "compatible with current screen"
    x86.state.gdi32.dcs.push(DC {});
    x86.state.gdi32.dcs.len() as u32
}

fn DeleteDC(_x86: &mut X86, hdc: u32) -> u32 {
    0 // fail
}

winapi!(
    fn GetStockObject(i: u32);
    fn SelectObject(hdc: u32, hGdiObj: u32);
    fn GetObjectA(handle: u32, bytes: u32, out: u32);

    fn CreateCompatibleDC(hdc: u32);
    fn DeleteDC(hdc: u32);
);
