#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;
mod dialog;
mod menu;
mod message;
mod misc;
mod paint;
pub mod printf;
mod rect;
mod resource;
mod timer;
mod window;
mod wndclass;

pub use builtin::DLL;

use std::{cell::RefCell, rc::Rc};
use win32_system::System;
use win32_winapi::{HWND, Handles};

pub use builtin_gdi32::HDC;
pub use dialog::*;
pub use menu::*;
pub use message::*;
pub use misc::*;
pub use paint::*;
pub use rect::*;
pub use resource::*;
pub use timer::*;
pub use win32_system::resource::ResourceKey;
pub use window::*;
pub use wndclass::*;

pub struct State {
    wndclasses: WndClasses,

    /// Count of user-registered message ids, see RegisterWindowMessageA.
    pub user_window_message_count: u32,

    pub windows: Handles<HWND, Rc<RefCell<Window>>>,
    desktop_window: HWND,
    active_window: HWND,

    messages: MessageQueue,
    timers: Timers,
}

impl Default for State {
    fn default() -> Self {
        Self {
            wndclasses: WndClasses::default(),
            user_window_message_count: 0,
            // Start window handles at 5, to make accidents more obvious.
            windows: Handles::new(5),
            // Use an arbitrary value for the desktop window, to make accidental accesses more obvious.
            desktop_window: HWND::from_raw(1),
            active_window: HWND::null(),
            messages: MessageQueue::default(),
            timers: Timers::default(),
        }
    }
}

pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    type SysState = std::cell::RefCell<State>;
    sys.state(&std::any::TypeId::of::<SysState>())
        .downcast_ref::<SysState>()
        .unwrap()
        .borrow_mut()
}
