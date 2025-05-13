mod arena;
pub mod builtin;
pub mod kernel32;
mod ntdll;
mod ucrtbase;
mod winmm;

pub use kernel32::HFILE;
pub use memory::str16::{Str16, String16};
pub use win32_winapi::{
    CStr, DWORD, ERROR, HANDLE, HRSRC, HWND, Handles, LPARAM, POINT, RECT, StrExt, WORD,
    WindowsPath,
};

pub struct State {
    pub ddraw: std::cell::RefCell<builtin_ddraw::State>,
    pub dsound: std::cell::RefCell<builtin_dsound::State>,
    pub gdi32: std::cell::RefCell<builtin_gdi32::State>,
    pub kernel32: kernel32::State,
    pub user32: std::cell::RefCell<builtin_user32::State>,
    pub winmm: winmm::State,
}

impl State {
    pub fn new(kernel32: kernel32::State) -> Self {
        State {
            ddraw: Default::default(),
            dsound: Default::default(),
            gdi32: Default::default(),
            kernel32,
            user32: Default::default(),
            winmm: winmm::State::default(),
        }
    }
}
