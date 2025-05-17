use win32_system::System;

use super::MMRESULT;

pub type HMIXEROBJ = u32;

#[win32_derive::dllexport]
pub fn mixerOpen(
    sys: &dyn System,
    phmx: u32, //Option<&mut HMIXER>,
    uMxId: u32,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: u32,
) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerGetLineControlsA(
    sys: &dyn System,
    hmxobj: u32,
    pmxlc: u32,
    fdwControls: u32,
) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerClose(sys: &dyn System, hmx: u32) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerSetControlDetails(
    sys: &dyn System,
    hmxobj: u32, //HMIXEROBJ,
    pmxcd: u32,  //&MIXERCONTROLDETAILS,
    fdwDetails: u32,
) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerGetControlDetailsA(sys: &dyn System) -> MMRESULT {
    todo!()
}

#[win32_derive::dllexport]
pub fn mixerGetLineInfoA(
    sys: &dyn System,
    hmxobj: HMIXEROBJ,
    pmxl: u32, //Option<&mut MIXERLINEA>,
    fdwInfo: u32,
) -> MMRESULT {
    todo!();
}
