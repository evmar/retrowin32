//! Interfaces expected of the x86 host.

use wasm_bindgen::prelude::wasm_bindgen;

/// DirectDraw surface.
pub trait Surface {
    fn flip(&self);
    fn bit_blt(&self, dx: u32, xy: u32, other: u32, sx: u32, sy: u32, w: u32, h: u32);
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SurfaceOptions {
    pub width: u32,
    pub height: u32,
    pub primary: bool,
}
impl Default for SurfaceOptions {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            primary: false,
        }
    }
}

/// Floating window.
pub trait Window {
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);
}

pub trait Host {
    fn exit(&self, code: u32);
    fn write(&self, buf: &[u8]) -> usize;
    fn time(&self) -> u32;

    fn create_window(&self) -> Box<dyn Window>;
    fn create_surface(&self, opts: &SurfaceOptions) -> Box<dyn Surface>;
}
