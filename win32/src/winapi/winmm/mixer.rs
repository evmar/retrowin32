use super::MMRESULT;
use crate::Machine;

pub type HMIXEROBJ = u32;

#[win32_derive::dllexport]
pub fn mixerOpen(
    _machine: &mut Machine,
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
    _machine: &mut Machine,
    hmxobj: u32,
    pmxlc: u32,
    fdwControls: u32,
) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerClose(_machine: &mut Machine, hmx: u32) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerSetControlDetails(
    _machine: &mut Machine,
    hmxobj: u32, //HMIXEROBJ,
    pmxcd: u32,  //&MIXERCONTROLDETAILS,
    fdwDetails: u32,
) -> MMRESULT {
    todo!();
}

#[win32_derive::dllexport]
pub fn mixerGetControlDetailsA(_machine: &mut Machine) -> MMRESULT {
    todo!()
}

#[win32_derive::dllexport]
pub fn mixerGetLineInfoA(
    _machine: &mut Machine,
    hmxobj: HMIXEROBJ,
    pmxl: u32, //Option<&mut MIXERLINEA>,
    fdwInfo: u32,
) -> MMRESULT {
    todo!();
}
