use std::rc::Rc;

mod arena;
mod bitmap;
pub mod builtin;
mod comctl32;
pub mod ddraw;
mod encoding;
pub mod gdi32;
pub mod kernel32;
mod ntdll;
mod printf;
mod ucrtbase;
pub mod user32;
mod winmm;

pub use kernel32::HFILE;
pub use memory::str16::{Str16, String16};
use win32_system::{Heap, memory::Memory};
pub use win32_winapi::{
    CStr, DWORD, ERROR, HANDLE, HRSRC, HWND, Handles, LPARAM, POINT, RECT, StrExt, WORD,
    WindowsPath,
};

pub struct State {
    scratch: Rc<Heap>,

    pub ddraw: ddraw::State,
    pub dsound: std::cell::RefCell<builtin_dsound::State>,
    pub gdi32: std::cell::RefCell<gdi32::State>,
    pub kernel32: kernel32::State,
    pub user32: user32::State,
    pub winmm: winmm::State,
}

impl State {
    pub fn new(memory: &mut Memory, kernel32: kernel32::State) -> Self {
        let scratch = memory.new_heap(0x1000, "winapi scratch".into());

        State {
            scratch,
            ddraw: ddraw::State::default(),
            dsound: Default::default(),
            gdi32: Default::default(),
            kernel32,
            user32: user32::State::default(),
            winmm: winmm::State::default(),
        }
    }
}
