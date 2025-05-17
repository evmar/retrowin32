#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;
mod dialog;
mod keyboard;
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
pub use keyboard::*;
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
            desktop_window: HWND::null(),
            active_window: HWND::null(),
            messages: MessageQueue::default(),
            timers: Timers::default(),
        }
    }
}

impl State {
    pub fn desktop_window(&mut self) -> HWND {
        if self.desktop_window.is_null() {
            let wndclass = Rc::new(RefCell::new(WndClass {
                name: "Desktop".to_string(),
                style: CS::empty(),
                wndproc: 0,
                background: HBRUSH::null(),
                wnd_extra: 0,
            }));
            self.desktop_window = self.windows.add(Rc::new(RefCell::new(Window {
                id: 0,
                typ: WindowType::Desktop,
                width: 640,
                height: 480,
                wndclass,
                window_style: WS::empty(),
                other_style: 0,
                show_cmd: SW::SHOW,
                user_data: 0,
                extra: None,
            })));
        }
        self.desktop_window
    }
}

pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    type SysState = std::cell::RefCell<State>;
    sys.state(&std::any::TypeId::of::<SysState>())
        .downcast_ref::<SysState>()
        .unwrap()
        .borrow_mut()
}
