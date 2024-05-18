//! Interfaces expected of the x86 host.

use wasm_bindgen::prelude::wasm_bindgen;

/// DirectDraw surface.
pub trait Surface {
    /// Write RGBA pixel data directly.
    /// Used for copying an image to the surface via GDI calls, and for Lock/Unlock pixel writes.
    fn write_pixels(&mut self, pixels: &[[u8; 4]]);

    /// Show the this surface as the foreground.  Called by ::Flip().
    fn show(&mut self);

    // TODO: the trait object here means we end up needing to cast, but the alternative
    // isn't object safe, bleh.
    fn bit_blt(&mut self, dx: u32, dy: u32, src: &dyn Surface, sx: u32, sy: u32, w: u32, h: u32);
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
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
    fn fullscreen(&mut self);
}

pub trait File {
    /// Just file size for now, but maybe we'll need more(?)
    fn info(&self) -> u32;
    fn seek(&mut self, ofs: u32) -> bool;
    fn read(&mut self, buf: &mut [u8], len: &mut u32) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub struct MouseMessage {
    pub down: bool,
    pub button: MouseButton,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub enum MessageDetail {
    Quit,
    Mouse(MouseMessage),
}

#[derive(Debug)]
pub struct Message {
    pub hwnd: u32,
    pub detail: MessageDetail,
}

pub trait Host {
    fn exit(&self, code: u32);
    fn time(&self) -> u32;

    /// Get the next pending message, or None if no message waiting.
    fn get_message(&self) -> Option<Message>;

    /// Signal that the emulator is blocked awaiting a message or an (optional) timeout.
    /// Returns true if block() synchronously blocked until the message/timeout happened,
    /// and false otherwise, in which case it's the host's responsibility to call
    /// unblock() when ready.
    fn block(&self, wait: Option<u32>) -> bool;

    fn open(&self, path: &str) -> Box<dyn File>;
    fn write(&self, buf: &[u8]) -> usize;

    fn create_window(&mut self, hwnd: u32) -> Box<dyn Window>;
    fn create_surface(&mut self, hwnd: u32, opts: &SurfaceOptions) -> Box<dyn Surface>;
}
