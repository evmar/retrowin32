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
mod state;
mod timer;
mod window;
mod wndclass;

pub use builtin::DLL;

pub use menu::HMENU;
pub use message::{MSG, WM};
pub use misc::HINSTANCE;
// Used by comctl32.
pub use misc::{TRACKMOUSEEVENT, TrackMouseEvent};
pub use resource::HBRUSH;
pub use state::{State, get_state};
pub use window::activate_window;

pub use win32_system::resource::ResourceKey;
