use crate::Machine;

pub type HMIDIOUT = u32;

#[win32_derive::dllexport]
pub fn midiOutOpen(
    _machine: &mut Machine,
    phmo: Option<&mut HMIDIOUT>,
    uDeviceID: u32,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: u32, /* MIDI_WAVE_OPEN_TYPE */
) -> u32 {
    todo!()
}

pub type MIDIOUTCAPSA = u32; // TODO

#[win32_derive::dllexport]
pub fn midiOutGetDevCapsA(
    _machine: &mut Machine,
    uDeviceID: u32,
    pmoc: Option<&mut MIDIOUTCAPSA>,
    cbmoc: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutGetNumDevs(_machine: &mut Machine) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutReset(_machine: &mut Machine, hmo: HMIDIOUT) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutClose(_machine: &mut Machine, hmo: HMIDIOUT) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutSetVolume(_machine: &mut Machine, hmo: HMIDIOUT, dwVolume: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutShortMsg(_machine: &mut Machine, hmo: HMIDIOUT, dwMsg: u32) -> u32 {
    todo!()
}
