//! Interfaces expected of the x86 host.

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

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
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

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, Default)]
pub struct FileOptions {
    /// Permit read access.
    pub read: bool,
    /// Permit write access.
    pub write: bool,
    /// Truncate the file to zero length.
    pub truncate: bool,
    /// Create the file if it doesn't exist.
    pub create: bool,
    /// Create the file if it doesn't exist, and fail if it does.
    pub create_new: bool,
}

impl FileOptions {
    pub fn read() -> Self {
        Self {
            read: true,
            ..Default::default()
        }
    }
}

pub trait File: std::io::Read + std::io::Write + std::io::Seek {
    fn stat(&self) -> Result<Stat, u32>;
}

pub trait FindHandle {
    fn next(&mut self) -> Result<Option<FindFile>, u32>;
}

#[derive(Debug, Clone)]
pub struct FindFile {
    pub path: String,
    pub name: String,
    pub stat: Stat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatKind {
    File,
    Directory,
    Symlink,
}

// Times are in hecto-nanoseconds (100ns units) since the
// Windows epoch (1601-01-01 UTC).
#[derive(Debug, Clone)]
pub struct Stat {
    pub kind: StatKind,
    pub size: u64,
    pub atime: u64,
    pub ctime: u64,
    pub mtime: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    None,
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
    pub time: u32, // in units of Host::time()
}

pub trait Host {
    fn exit(&self, code: u32);
    fn time(&self) -> u32;
    fn system_time(&self) -> chrono::DateTime<chrono::Local>;

    /// Get the next pending message, or None if no message waiting.
    fn get_message(&self) -> Option<Message>;

    /// Signal that the emulator is blocked awaiting a message or an (optional) timeout.
    /// Returns true if block() synchronously blocked until the message/timeout happened,
    /// and false otherwise, in which case it's the host's responsibility to call
    /// unblock() when ready.
    fn block(&self, wait: Option<u32>) -> bool;

    fn canonicalize(&self, path: &str) -> Result<String, u32>;
    fn open(&self, path: &str, options: FileOptions) -> Result<Box<dyn File>, u32>;
    fn stat(&self, path: &str) -> Result<Stat, u32>;
    fn find(&self, path: &str) -> Result<Box<dyn FindHandle>, u32>;
    fn log(&self, buf: &[u8]);

    fn create_window(&mut self, hwnd: u32) -> Box<dyn Window>;
    fn create_surface(&mut self, hwnd: u32, opts: &SurfaceOptions) -> Box<dyn Surface>;
}
