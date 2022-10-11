//! Interfaces expected of the x86 host.

/// DirectDraw surface.
pub trait Surface {
    fn flip(&self);
}

/// Floating window.
pub trait Window {
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);

    fn new_surface(&mut self) -> Box<dyn Surface>;
}

pub trait Host {
    fn exit(&self, code: u32);
    fn write(&self, buf: &[u8]) -> usize;
    fn time(&self) -> u32;

    fn create_window(&self) -> Box<dyn Window>;
}
