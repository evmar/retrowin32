#![allow(non_snake_case)]

mod dialog;
mod menu;
mod message;
mod misc;
mod paint;
mod resource;
mod timer;
mod window;

use super::handle::Handles;
use super::types::HWND;

pub use super::gdi32::HDC;
pub use super::kernel32::ResourceKey;
pub use dialog::*;
pub use menu::*;
pub use message::*;
pub use misc::*;
pub use paint::*;
pub use resource::*;
pub use timer::*;
pub use window::*;

#[derive(Default)]
pub struct State {
    wndclasses: Vec<std::rc::Rc<WndClass>>,
    pub user_window_message_count: u32,
    pub windows: Handles<HWND, Window>,
    messages: std::collections::VecDeque<MSG>,
    timers: Timers,
}
