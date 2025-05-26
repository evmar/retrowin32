use crate::winapi::Str16;
use std::ops::Range;
pub use win32_system::{
    System,
    resource::{ResourceKey, find_resource},
};
use win32_winapi::{HANDLE, HMODULE, Handles};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HRSRCT;
pub type HRSRC = HANDLE<HRSRCT>;

pub struct ResourceHandle(Range<u32>);

type State = Handles<HRSRC, ResourceHandle>;

fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    type SysState = std::cell::RefCell<State>;
    sys.state2(&std::any::TypeId::of::<SysState>(), || {
        Box::new(SysState::default())
    })
    .downcast_ref::<SysState>()
    .unwrap()
    .borrow_mut()
}

#[win32_derive::dllexport]
pub fn FindResourceA(
    sys: &dyn System,
    hModule: HMODULE,
    lpName: ResourceKey<&str>,
    lpType: ResourceKey<&str>,
) -> HRSRC {
    let name = lpName.to_string16();
    let type_ = lpType.to_string16();
    FindResourceW(sys, hModule, name.as_ref(), type_.as_ref())
}

#[win32_derive::dllexport]
pub fn FindResourceW(
    sys: &dyn System,
    hModule: HMODULE,
    lpName: ResourceKey<&Str16>,
    lpType: ResourceKey<&Str16>,
) -> HRSRC {
    match find_resource(sys, hModule, lpType, &lpName) {
        None => HRSRC::null(),
        Some(mem) => get_state(sys).add(ResourceHandle(mem)),
    }
}

#[win32_derive::dllexport]
pub fn LoadResource(sys: &dyn System, hModule: HMODULE, hResInfo: HRSRC) -> u32 {
    hResInfo.to_raw()
}

#[win32_derive::dllexport]
pub fn LockResource(sys: &dyn System, hResData: HRSRC) -> u32 {
    match get_state(sys).get(hResData) {
        None => 0,
        Some(handle) => handle.0.start,
    }
}

#[win32_derive::dllexport]
pub fn FreeResource(sys: &dyn System, hResData: u32) -> bool {
    // todo
    true // success
}

#[win32_derive::dllexport]
pub fn SizeofResource(sys: &dyn System, hModule: HMODULE, hResInfo: HRSRC) -> u32 {
    match get_state(sys).get(hResInfo) {
        None => 0,
        Some(handle) => handle.0.len() as u32,
    }
}
