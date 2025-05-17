use win32_system::System;

pub type HMIDIOUT = u32;

#[win32_derive::dllexport]
pub fn midiOutOpen(
    sys: &dyn System,
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
    sys: &dyn System,
    uDeviceID: u32,
    pmoc: Option<&mut MIDIOUTCAPSA>,
    cbmoc: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutGetNumDevs(sys: &dyn System) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutReset(sys: &dyn System, hmo: HMIDIOUT) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutClose(sys: &dyn System, hmo: HMIDIOUT) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutSetVolume(sys: &dyn System, hmo: HMIDIOUT, dwVolume: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiOutShortMsg(sys: &dyn System, hmo: HMIDIOUT, dwMsg: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn midiInGetNumDevs(sys: &dyn System) -> u32 {
    0
}
