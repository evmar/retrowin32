#![allow(non_snake_case)]

use crate::winapi::kernel32::HMODULE;
use crate::winapi::stack_args::ToX86;
use crate::winapi::user32::HINSTANCE;
use crate::{
    pe,
    winapi::{
        kernel32,
        stack_args::FromArg,
        types::{Str16, String16},
    },
    Machine,
};
use memory::Mem;

const TRACE_CONTEXT: &'static str = "kernel32/resource";

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

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
    name: ResourceKey<&Str16>,
) -> Option<Mem<'a>> {
    let image = mem.slice(hInstance..);
    if hInstance == kernel32.image_base {
        let section = kernel32.resources.as_slice(image.as_slice_todo())?;
        Some(image.slice(pe::find_resource(section, typ.into_pe(), name.into_pe())?))
    } else {
        let dll = kernel32.dlls.get(&HMODULE::from_raw(hInstance))?;
        match dll.dll.resources.clone() {
            None => return None,
            Some(resources) => {
                let section = resources.as_slice(image.as_slice_todo())?;
                Some(image.slice(pe::find_resource(section, typ.into_pe(), name.into_pe())?))
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
) -> u32 {
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
) -> u32 {
    match find_resource(
        &machine.state.kernel32,
        machine.mem(),
        hModule.to_raw(),
        lpType,
        lpName,
    ) {
        None => 0,
        Some(mem) => mem.offset_from(machine.mem()),
    }
}

#[win32_derive::dllexport]
pub fn LoadResource(_machine: &mut Machine, hModule: u32, hResInfo: u32) -> u32 {
    hResInfo
}

#[win32_derive::dllexport]
pub fn LockResource(_machine: &mut Machine, hResData: u32) -> u32 {
    hResData
}
