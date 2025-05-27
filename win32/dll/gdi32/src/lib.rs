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

pub use dc::{DC, DCTarget, HDC};
pub use draw::{Brush, COLORREF, fill_rect};
pub use object::{HGDIOBJ, LOWEST_HGDIOBJ, Object};
pub use palette::PALETTEENTRY;
pub use state::{GDIHandles, State, get_state};
