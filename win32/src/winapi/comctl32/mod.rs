#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod builtin;

use crate::winapi;
use crate::{Machine, System};
pub use builtin::DLL;

pub use winapi::user32::TRACKMOUSEEVENT;

#[win32_derive::dllexport(ordinal = 17)]
pub fn InitCommonControls(sys: &dyn System) {}

#[win32_derive::dllexport]
pub fn _TrackMouseEvent(machine: &mut Machine, lpEventTrack: Option<&mut TRACKMOUSEEVENT>) -> bool {
    winapi::user32::TrackMouseEvent(machine, lpEventTrack)
}
