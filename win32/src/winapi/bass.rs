#![allow(non_snake_case)]

//! bass.dll found in monolife.
//! TODO: DLL loading, eek.

use crate::machine::Machine;

const TRACE: bool = true;

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
pub fn BASS_Start(_machine: &mut Machine) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn BASS_MusicPlay(_machine: &mut Machine, arg1: u32) -> u32 {
    1
}
