#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod builtin;
mod clipper;
mod ddraw;
mod ddraw1;
mod ddraw2;
mod ddraw3;
mod ddraw7;
mod palette;
mod types;

pub use builtin::DLL;

pub use ddraw::{State, get_state};
pub use types::*;
pub use win32_winapi::com::GUID;
