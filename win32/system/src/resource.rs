//! Windows resource system.
//! Used by kernel32 and user32.

use memory::Mem;
use std::ops::Range;
use win32_winapi::{HMODULE, Str16, String16, calling_convention::FromArg};

use crate::System;

fn is_intresource(x: u32) -> bool {
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
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if is_intresource(arg) {
            ResourceKey::Id(arg)
        } else {
            ResourceKey::Name(<Option<T>>::from_arg(mem, arg).unwrap())
        }
    }
}

pub fn find_resource<'a>(
    sys: &dyn System,
    hinstance: HMODULE,
    typ: ResourceKey<&Str16>,
    name: &ResourceKey<&Str16>,
) -> Option<Range<u32>> {
    let resources = sys.get_resources(hinstance)?;
    let addr = hinstance.to_raw();
    pe::find_resource(resources, typ.into_pe(), name.into_pe())
        .map(|r| (addr + r.start)..(addr + r.end))
}
