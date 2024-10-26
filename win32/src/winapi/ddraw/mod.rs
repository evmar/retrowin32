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

pub use clipper::*;
pub use ddraw::*;
pub use ddraw1::*;
pub use ddraw2::*;
pub use ddraw3::*;
pub use ddraw7::*;
pub use palette::IDirectDrawPalette;
