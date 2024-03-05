#![allow(non_snake_case)]

use super::{Str16, String16};
use crate::{pe, winapi::stack_args::FromArg, Machine};
use memory::Mem;

const TRACE_CONTEXT: &'static str = "kernel32/resource";

fn IS_INTRESOURCE(x: u32) -> bool {
    x >> 16 == 0
}

#[derive(Debug)]
pub enum ResourceId<T> {
    Id(u32),
    Name(T),
}

impl ResourceId<&Str16> {
    pub fn into_pe(&self) -> pe::ResourceName {
        match *self {
            ResourceId::Id(id) => pe::ResourceName::Id(id),
            ResourceId::Name(name) => pe::ResourceName::Name(name.buf()),
        }
    }
}

impl<'a, T> FromArg<'a> for ResourceId<T>
where
    Option<T>: FromArg<'a>,
{
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if IS_INTRESOURCE(arg) {
            ResourceId::Id(arg)
        } else {
            ResourceId::Name(<Option<T>>::from_arg(mem, arg).unwrap())
        }
    }
}

pub fn find_resource<'a>(
    machine: &'a Machine,
    typ: ResourceId<&Str16>,
    name: ResourceId<&Str16>,
) -> Option<Mem<'a>> {
    let image = machine.mem().slice(machine.state.kernel32.image_base..);
    Some(image.slice(pe::find_resource(
        image,
        &machine.state.kernel32.resources,
        typ.into_pe(),
        name.into_pe(),
    )?))
}

#[win32_derive::dllexport]
pub fn FindResourceA(
    machine: &mut Machine,
    hModule: u32,
    lpName: ResourceId<&str>,
    lpType: ResourceId<&str>,
) -> u32 {
    let namebuf: String16;
    let lpName = match lpName {
        ResourceId::Id(id) => ResourceId::Id(id),
        ResourceId::Name(name) => {
            namebuf = String16::from(name);
            ResourceId::Name(namebuf.as_str16())
        }
    };
    let typebuf: String16;
    let lpType = match lpType {
        ResourceId::Id(id) => ResourceId::Id(id),
        ResourceId::Name(name) => {
            typebuf = String16::from(name);
            ResourceId::Name(typebuf.as_str16())
        }
    };
    FindResourceW(machine, hModule, lpName, lpType)
}

#[win32_derive::dllexport]
pub fn FindResourceW(
    machine: &mut Machine,
    hModule: u32,
    lpName: ResourceId<&Str16>,
    lpType: ResourceId<&Str16>,
) -> u32 {
    match find_resource(machine, lpType, lpName) {
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
