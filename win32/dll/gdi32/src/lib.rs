#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;

pub use builtin::DLL;

pub mod bitmap;
mod bitmap_api;
mod dc;
mod draw;
mod object;
mod palette;
mod state;
mod text;
pub use bitmap_api::*;
pub use dc::*;
pub use draw::*;
pub use object::*;
pub use palette::*;
pub use state::*;
pub use text::*;

use win32_system::System;

pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    type SysState = std::cell::RefCell<State>;
    sys.state(&std::any::TypeId::of::<SysState>())
        .downcast_ref::<SysState>()
        .unwrap()
        .borrow_mut()
}
