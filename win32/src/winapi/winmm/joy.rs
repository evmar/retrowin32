use crate::Machine;

pub type JOYCAPSA = u32;
pub type JOYINFOEX = u32;

#[win32_derive::dllexport]
pub fn joyGetNumDevs(_machine: &mut Machine) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn joyGetDevCapsA(
    _machine: &mut Machine,
    uJoyID: u32,
    pjc: Option<&mut JOYCAPSA>,
    cbjc: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn joyGetPosEx(_machine: &mut Machine, uJoyID: u32, pji: Option<&mut JOYINFOEX>) -> u32 {
    todo!()
}
