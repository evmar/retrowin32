pub mod builtin;
pub mod kernel32;
mod ntdll;

pub use kernel32::file::HFILE;

pub struct State {
    pub ddraw: std::cell::RefCell<builtin_ddraw::State>,
    pub dsound: std::cell::RefCell<builtin_dsound::State>,
    pub gdi32: std::cell::RefCell<builtin_gdi32::State>,
    pub kernel32: kernel32::State,
    pub user32: std::cell::RefCell<builtin_user32::State>,
    pub winmm: std::cell::RefCell<builtin_winmm::State>,
}

impl State {
    pub fn new(kernel32: kernel32::State) -> Self {
        State {
            ddraw: Default::default(),
            dsound: Default::default(),
            gdi32: Default::default(),
            kernel32,
            user32: Default::default(),
            winmm: Default::default(),
        }
    }
}
