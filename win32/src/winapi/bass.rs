//! This module stubs bass.dll as found in monolife.
//! monolife ships its own bass.dll, but I wrote this module before
//! retrowin32 could load external dlls.
//!
//! Today retrowin32 is capable of loading the dll, but it appears
//! to be packed with some packer that fails when we load it.

#![allow(non_snake_case)]

use super::kernel32;
use crate::machine::Machine;

/// Hack: time since BASS_Start etc. was called.
static mut T: u32 = 0;

const TRACE_CONTEXT: &'static str = "bass";

#[win32_derive::dllexport]
pub fn BASS_Init(_machine: &mut Machine, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn BASS_MusicLoad(
    _machine: &mut Machine,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn BASS_Start(machine: &mut Machine) -> u32 {
    unsafe {
        T = kernel32::GetTickCount(machine);
    }
    1
}

#[win32_derive::dllexport]
pub fn BASS_MusicPlay(machine: &mut Machine, arg1: u32) -> u32 {
    unsafe {
        T = kernel32::GetTickCount(machine);
    }
    1
}

#[win32_derive::dllexport]
pub fn BASS_ChannelGetPosition(machine: &mut Machine, mode: u32) -> u32 {
    let dur = kernel32::GetTickCount(machine) - unsafe { T };
    match mode {
        0 => {
            // BASS_POS_BYTE
            (dur as f32 * 44.1) as u32 // 44.1khz
        }
        1 => {
            log::error!("unimpl: BASS_POS_MUSIC_ORDER");
            0
        }
        _ => 0,
    }
}

#[win32_derive::dllexport]
pub fn BASS_MusicSetPositionScaler(_machine: &mut Machine, arg1: u32, arg2: u32) -> u32 {
    1
}
