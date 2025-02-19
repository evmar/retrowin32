use crate::machine::MemImpl;

mod advapi32;
mod alloc;
mod bass;
mod bitmap;
mod builtin;
mod calling_convention;
mod com;
pub mod ddraw;
mod dinput;
pub mod dsound;
mod error;
pub mod gdi32;
mod handle;
mod heap;
pub mod kernel32;
mod ntdll;
mod ole32;
mod oleaut32;
mod retrowin32_test;
mod shlwapi;
mod types;
mod ucrtbase;
pub mod user32;
mod vcruntime140;
mod version;
mod wininet;
mod winmm;

pub use crate::str16::{Str16, String16};
pub use error::ERROR;
pub use types::*;

pub struct State {
    scratch: heap::Heap,

    pub ddraw: ddraw::State,
    pub dsound: dsound::State,
    pub gdi32: gdi32::State,
    pub kernel32: kernel32::State,
    pub user32: user32::State,
    pub winmm: winmm::State,
}

impl State {
    pub fn new(mem: &mut MemImpl, mut kernel32: kernel32::State) -> Self {
        let scratch = kernel32.new_private_heap(mem, 0x1000, "winapi scratch".into());

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
