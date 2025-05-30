use crate::{
    HBRUSH,
    message::MessageQueue,
    timer::Timers,
    window::{SW, WS, Window, WindowType},
    wndclass::{CS, WndClass, WndClasses},
};
use std::{cell::RefCell, rc::Rc};
use win32_system::{System, generic_get_state};
use win32_winapi::{HWND, Handles};

pub struct State {
    pub wndclasses: WndClasses,

    /// Count of user-registered message ids, see RegisterWindowMessageA.
    pub user_window_message_count: u32,

    pub windows: Handles<HWND, Rc<RefCell<Window>>>,
    pub desktop_window: HWND,
    pub active_window: HWND,

    pub messages: MessageQueue,
    pub timers: Timers,
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

#[inline]
pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    generic_get_state::<State>(sys)
}
