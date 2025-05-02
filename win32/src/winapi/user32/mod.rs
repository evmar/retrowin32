#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;
mod dialog;
mod menu;
mod message;
mod misc;
mod paint;
mod rect;
mod resource;
mod timer;
mod window;
mod wndclass;

pub use builtin::DLL;

use super::{HWND, Handles};
use std::{cell::RefCell, rc::Rc};
use win32_system::System;

pub use super::kernel32::ResourceKey;
pub use builtin_gdi32::HDC;
pub use dialog::*;
pub use menu::*;
pub use message::*;
pub use misc::*;
pub use paint::*;
pub use rect::*;
pub use resource::*;
pub use timer::*;
pub use window::*;
pub use wndclass::*;

#[derive(Default)]
pub struct State {
    wndclasses: WndClasses,

    /// Count of user-registered message ids, see RegisterWindowMessageA.
    pub user_window_message_count: u32,

    pub windows: Handles<HWND, Rc<RefCell<Window>>>,
    active_window: HWND,

    messages: MessageQueue,
    timers: Timers,
}

pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    type SysState = std::cell::RefCell<State>;
    sys.state(&std::any::TypeId::of::<SysState>())
        .downcast_ref::<SysState>()
        .unwrap()
        .borrow_mut()
}
