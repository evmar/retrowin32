use crate::{
    Machine,
    winapi::{HRSRC, Str16, kernel32::HMODULE},
};
use std::ops::Range;

pub use win32_system::resource::{ResourceKey, find_resource};

pub struct ResourceHandle(Range<u32>);

#[win32_derive::dllexport]
pub fn FindResourceA(
    machine: &mut Machine,
    hModule: HMODULE,
    lpName: ResourceKey<&str>,
    lpType: ResourceKey<&str>,
) -> HRSRC {
    let name = lpName.to_string16();
    let type_ = lpType.to_string16();
    FindResourceW(machine, hModule, name.as_ref(), type_.as_ref())
}

#[win32_derive::dllexport]
pub fn FindResourceW(
    machine: &mut Machine,
    hModule: HMODULE,
    lpName: ResourceKey<&Str16>,
    lpType: ResourceKey<&Str16>,
) -> HRSRC {
    match find_resource(machine, hModule.to_raw(), lpType, &lpName) {
        None => HRSRC::null(),
        Some(mem) => machine
            .state
            .kernel32
            .resource_handles
            .add(ResourceHandle(mem)),
    }
}

#[win32_derive::dllexport]
pub fn LoadResource(machine: &mut Machine, hModule: HMODULE, hResInfo: HRSRC) -> u32 {
    hResInfo.to_raw()
}

#[win32_derive::dllexport]
pub fn LockResource(machine: &mut Machine, hResData: HRSRC) -> u32 {
    match machine.state.kernel32.resource_handles.get(hResData) {
        None => 0,
        Some(handle) => handle.0.start,
    }
}

#[win32_derive::dllexport]
pub fn FreeResource(machine: &mut Machine, hResData: u32) -> bool {
    // todo
    true // success
}

#[win32_derive::dllexport]
pub fn SizeofResource(machine: &mut Machine, hModule: HMODULE, hResInfo: HRSRC) -> u32 {
    match machine.state.kernel32.resource_handles.get(hResInfo) {
        None => 0,
        Some(handle) => handle.0.len() as u32,
    }
}
