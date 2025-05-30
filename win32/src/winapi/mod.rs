pub mod builtin;
mod ntdll;

#[derive(Default)]
pub struct State {
    pub ddraw: std::cell::RefCell<builtin_ddraw::State>,
    pub dsound: std::cell::RefCell<builtin_dsound::State>,
    pub gdi32: std::cell::RefCell<builtin_gdi32::State>,
    pub user32: std::cell::RefCell<builtin_user32::State>,
    pub winmm: std::cell::RefCell<builtin_winmm::State>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
