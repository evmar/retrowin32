#![allow(non_snake_case)]

use crate::winapi::kernel32::HMODULE;
use crate::winapi::types::HRSRC;
use crate::winapi::user32::HINSTANCE;
use crate::{
    pe,
    winapi::{
        calling_convention::FromArg,
        kernel32,
        types::{Str16, String16},
    },
    Machine,
};
use memory::Mem;
use std::ops::Range;

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

pub struct ResourceHandle(Range<u32>);

/// ResourceKey is the type of queries into the Windows resources system, including
/// e.g. LoadResource() as well as LoadBitmap() etc.
/// It's parameterized over the type of name to handle both A() and W() variants.
#[derive(Debug)]
pub enum ResourceKey<T> {
    Id(u32),
    Name(T),
}

impl<T> ResourceKey<T> {
    pub fn map_name<'a, R>(&'a self, f: impl Fn(&'a T) -> R) -> ResourceKey<R> {
        match *self {
            ResourceKey::Id(id) => ResourceKey::Id(id),
            ResourceKey::Name(ref name) => ResourceKey::Name(f(name)),
        }
    }
}

impl ResourceKey<&str> {
    pub fn to_string16(&self) -> ResourceKey<String16> {
        self.map_name(|name| String16::from(name))
    }
}

impl ResourceKey<String16> {
    pub fn as_ref<'a>(&'a self) -> ResourceKey<&'a Str16> {
        self.map_name(|name| name.as_str16())
    }
}

impl ResourceKey<&Str16> {
    pub fn into_pe(&self) -> pe::ResourceName {
        match *self {
            ResourceKey::Id(id) => pe::ResourceName::Id(id),
            ResourceKey::Name(name) => pe::ResourceName::Name(name),
        }
    }
}

impl<'a, T> FromArg<'a> for ResourceKey<T>
where
    Option<T>: FromArg<'a>,
{
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if IS_INTRESOURCE(arg) {
            ResourceKey::Id(arg)
        } else {
            ResourceKey::Name(<Option<T>>::from_arg(mem, arg).unwrap())
        }
    }
}

pub fn find_resource<'a>(
    kernel32: &kernel32::State,
    mem: Mem<'a>,
    hInstance: HINSTANCE,
    typ: ResourceKey<&Str16>,
    name: &ResourceKey<&Str16>,
) -> Option<Range<u32>> {
    let image = mem.slice(hInstance..);
    if hInstance == kernel32.image_base {
        let section = kernel32.resources.as_slice(image)?;
        pe::find_resource(section, typ.into_pe(), name.into_pe())
            .map(|r| (hInstance + r.start)..(hInstance + r.end))
    } else {
        let dll = kernel32.dlls.get(&HMODULE::from_raw(hInstance))?;
        match dll.module.resources.clone() {
            None => return None,
            Some(resources) => {
                let section = resources.as_slice(image)?;
                pe::find_resource(section, typ.into_pe(), name.into_pe())
                    .map(|r| (hInstance + r.start)..(hInstance + r.end))
            }
        }
    }
}

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
    match find_resource(
        &machine.state.kernel32,
        machine.mem(),
        hModule.to_raw(),
        lpType,
        &lpName,
    ) {
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
pub fn SizeofResource(machine: &mut Machine, hModule: HMODULE, hResInfo: HRSRC) -> u32 {
    match machine.state.kernel32.resource_handles.get(hResInfo) {
        None => 0,
        Some(handle) => handle.0.len() as u32,
    }
}
