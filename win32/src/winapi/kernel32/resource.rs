#![allow(non_snake_case)]

use super::Str16;
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

impl<T> ResourceId<T> {
    /// Helper until we implement resources by name.
    pub fn unwrap_id(&self) -> u32 {
        match *self {
            ResourceId::Id(id) => id,
            ResourceId::Name(_) => unimplemented!(),
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
    typ: ResourceId<&str>,
    name: ResourceId<&str>,
) -> Option<Mem<'a>> {
    let image = machine.mem().slice(machine.state.kernel32.image_base..);
    Some(image.slice(pe::find_resource(
        image,
        &machine.state.kernel32.resources,
        typ.unwrap_id(),
        name.unwrap_id(),
    )?))
}

#[win32_derive::dllexport]
pub fn FindResourceW(
    machine: &mut Machine,
    hModule: u32,
    lpName: ResourceId<&Str16>,
    lpType: ResourceId<&Str16>,
) -> u32 {
    let image = machine.mem().slice(machine.state.kernel32.image_base..);
    match pe::find_resource(
        image,
        &machine.state.kernel32.resources,
        lpType.unwrap_id(),
        lpName.unwrap_id(),
    ) {
        None => 0,
        Some(r) => machine.state.kernel32.image_base + r.start,
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
