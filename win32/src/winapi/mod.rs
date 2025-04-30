use crate::{heap::Heap, memory::Memory};
use std::{cell::RefCell, rc::Rc};

mod advapi32;
mod arena;
mod bass;
mod bitmap;
pub mod builtin;
mod com;
mod comctl32;
pub mod ddraw;
mod dinput;
pub mod dsound;
mod encoding;
pub mod gdi32;
pub mod kernel32;
mod ntdll;
mod ole32;
mod oleaut32;
mod printf;
mod retrowin32_test;
mod shlwapi;
pub mod trace;
mod ucrtbase;
pub mod user32;
mod vcruntime140;
mod version;
mod wininet;
mod winmm;

pub use memory::str16::{Str16, String16};
pub use win32_winapi::*;

pub struct State {
    scratch: Rc<RefCell<Heap>>,

    pub ddraw: ddraw::State,
    pub dsound: dsound::State,
    pub gdi32: gdi32::State,
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
            dsound: dsound::State::default(),
            gdi32: gdi32::State::default(),
            kernel32,
            user32: user32::State::default(),
            winmm: winmm::State::default(),
        }
    }
}
