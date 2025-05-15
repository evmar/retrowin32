//! Interfaces expected of the host platform, to e.g. open files or create windows.

pub use win32_winapi::{ERROR, RECT, WindowsPath, WindowsPathBuf};

/// Drawing surface, corresponding to window contents or DirectDraw surfaces.
pub trait Surface {
    /// Write pixel data.
    /// TODO: currently data is rgba32, but non-web implementation could handle other formats
    /// in principle.
    fn write_pixels(&self, pixels: &[u8]);

    /// Show the this surface as the foreground.  Called by ::Flip().
    fn show(&self);

    // TODO: the trait object here means we end up needing to cast, but the alternative
    // isn't object safe, bleh.
    fn bit_blt(&self, dst_rect: &RECT, src: &dyn Surface, src_rect: &RECT);
}

#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Debug, Default)]
pub struct SurfaceOptions {
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub primary: bool,
}

/// Floating window.
pub trait Window {
    fn set_title(&self, title: &str);
    fn set_size(&self, width: u32, height: u32);
    fn fullscreen(&self);
}

#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
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
    fn stat(&self) -> Result<Stat, ERROR>;
    fn set_len(&self, len: u64) -> Result<(), ERROR>;
}

pub trait ReadDir {
    fn next(&mut self) -> Result<Option<ReadDirEntry>, ERROR>;
}

#[derive(Debug, Clone)]
pub struct ReadDirEntry {
    pub name: String,
    pub stat: Stat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatKind {
    File,
    Directory,
    Symlink,
}

// Times are in nanoseconds relative to the Unix epoch.
#[derive(Debug, Clone)]
pub struct Stat {
    pub kind: StatKind,
    pub size: u64,
    pub atime: i64,
    pub ctime: i64,
    pub mtime: i64,
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

pub type AudioCallback = Box<dyn Fn() + Send + Sync>;

pub trait Audio {
    fn write(&mut self, buf: &[u8]);
    fn pos(&mut self) -> usize;
}

pub trait FileSystem {
    /// Retrieves the absolute (Windows-style) path of the current working directory.
    fn current_dir(&self) -> Result<WindowsPathBuf, ERROR>;
    /// Open a file at the given (Windows-style) path.
    fn open(&self, path: &WindowsPath, options: FileOptions) -> Result<Box<dyn File>, ERROR>;
    /// Retrieve file or directory metadata at the given (Windows-style) path.
    fn stat(&self, path: &WindowsPath) -> Result<Stat, ERROR>;
    /// Iterate the contents of a directory at the given (Windows-style) path.
    fn read_dir(&self, path: &WindowsPath) -> Result<Box<dyn ReadDir>, ERROR>;
    /// Create a new directory at the given (Windows-style) path.
    fn create_dir(&self, path: &WindowsPath) -> Result<(), ERROR>;
    /// Remove a file at the given (Windows-style) path.
    fn remove_file(&self, path: &WindowsPath) -> Result<(), ERROR>;
    /// Remove a directory at the given (Windows-style) path.
    fn remove_dir(&self, path: &WindowsPath) -> Result<(), ERROR>;
}

/// Interface expected of the host platform, implemented by e.g. the browser or SDL.
pub trait Host: FileSystem {
    /// Get an arbitrary time counter, measured in milliseconds.
    fn ticks(&self) -> u32;
    fn system_time(&self) -> chrono::DateTime<chrono::Local>;

    /// Get the next pending message, or None if no message waiting.
    fn get_message(&self) -> Option<Message>;

    /// Signal that the emulator is blocked awaiting a message or an (optional) timeout.
    /// Returns true if block() synchronously blocked until the message/timeout happened,
    /// and false otherwise, in which case it's the host's responsibility to call
    /// unblock() when ready.
    fn block(&self, wait: Option<u32>) -> bool;

    fn stdout(&self, buf: &[u8]);

    fn create_window(&self, hwnd: u32) -> Box<dyn Window>;
    fn create_surface(&self, hwnd: u32, opts: &SurfaceOptions) -> Box<dyn Surface>;

    fn init_audio(&self, sample_rate: u32, callback: AudioCallback) -> Box<dyn Audio>;
}
