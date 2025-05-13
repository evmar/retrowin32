#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod builtin;

pub use builtin::DLL;

pub use builtin_user32::TRACKMOUSEEVENT;
use win32_system::System;

#[win32_derive::dllexport(ordinal = 17)]
pub fn InitCommonControls(sys: &dyn System) {}

#[win32_derive::dllexport]
pub fn _TrackMouseEvent(sys: &mut dyn System, lpEventTrack: Option<&mut TRACKMOUSEEVENT>) -> bool {
    builtin_user32::TrackMouseEvent(sys, lpEventTrack)
}
