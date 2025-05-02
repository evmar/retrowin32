mod arena;
pub mod builtin;
mod comctl32;
pub mod ddraw;
mod encoding;
pub mod kernel32;
mod ntdll;
mod printf;
mod ucrtbase;
pub mod user32;
mod winmm;

pub use kernel32::HFILE;
pub use memory::str16::{Str16, String16};
pub use win32_winapi::{
    CStr, DWORD, ERROR, HANDLE, HRSRC, HWND, Handles, LPARAM, POINT, RECT, StrExt, WORD,
    WindowsPath,
};

pub struct State {
    pub ddraw: ddraw::State,
    pub dsound: std::cell::RefCell<builtin_dsound::State>,
    pub gdi32: std::cell::RefCell<builtin_gdi32::State>,
    pub kernel32: kernel32::State,
    pub user32: std::cell::RefCell<user32::State>,
    pub winmm: winmm::State,
}

impl State {
    pub fn new(kernel32: kernel32::State) -> Self {
        State {
            ddraw: ddraw::State::default(),
            dsound: Default::default(),
            gdi32: Default::default(),
            kernel32,
            user32: Default::default(),
            winmm: winmm::State::default(),
        }
    }
}
