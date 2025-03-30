use crate::{calling_convention::ArrayOut, winapi::Str16, Machine};

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings(machine: &mut Machine) -> u32 {
    machine.state.kernel32.env
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_machine: &mut Machine, _penv: u32) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(_machine: &mut Machine) -> u32 {
    // CRT startup appears to fallback on non-W version of this if it returns null.
    0
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(_machine: &mut Machine) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    buf: ArrayOut<u8>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableW(
    _machine: &mut Machine,
    name: Option<&Str16>,
    buf: ArrayOut<u16>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableW(
    _machine: &mut Machine,
    lpName: Option<&Str16>,
    lpValue: Option<&Str16>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    value: Option<&str>,
) -> bool {
    true
}
