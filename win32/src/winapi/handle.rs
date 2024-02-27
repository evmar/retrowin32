//! Structures for working with HWND, HDC, etc. Windows handles.

use std::{collections::HashMap, marker::PhantomData};

/// A more type-safe wrapper for HWND, HDC, etc. Windows handles.
/// The <T> parameter is purely a nominal type and not used for carrying any data.
///
/// Nullability: following windows, a given HWND can be null.  We don't attempt to work with
/// Option<HWND> instead etc. for two reasons:
/// 1. Many Windows APIs are not especially clear on nullability.
/// 2. Handles can be either null or invalid, two different states!
#[derive(Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct HANDLE<T> {
    pub raw: u32,
    marker: PhantomData<*const T>,
}

// It's not clear why, but I need explicit impls of Clone/Copy rather than deriving them.
impl<T> std::clone::Clone for HANDLE<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw,
            marker: PhantomData,
        }
    }
}
impl<T> std::marker::Copy for HANDLE<T> {}
impl<T> Default for HANDLE<T> {
    fn default() -> Self {
        Self {
            raw: 0,
            marker: PhantomData,
        }
    }
}

unsafe impl<T: 'static> memory::Pod for HANDLE<T> {}

impl<T> HANDLE<T> {
    pub const fn from_raw(raw: u32) -> Self {
        HANDLE {
            raw,
            marker: PhantomData,
        }
    }
    pub fn to_raw(&self) -> u32 {
        self.raw
    }

    // Handles have both null and invalid states, whoopsie.
    // https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    pub fn null() -> Self {
        Self::from_raw(0)
    }
    pub fn invalid() -> Self {
        Self::from_raw(-1i32 as u32)
    }
    pub fn is_null(&self) -> bool {
        self.raw == 0
    }
    pub fn is_invalid(&self) -> bool {
        self.raw == -1i32 as u32
    }
}

impl<T> std::fmt::Debug for HANDLE<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("HANDLE({:x})", self.raw))
    }
}

impl<T> std::fmt::LowerHex for HANDLE<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.raw.fmt(f)
    }
}

impl<'a, T> crate::winapi::stack_args::FromArg<'a> for HANDLE<T> {
    unsafe fn from_arg(_mem: memory::Mem, arg: u32) -> Self {
        Self::from_raw(arg)
    }
}

impl<T> crate::winapi::stack_args::ToX86 for HANDLE<T> {
    fn to_raw(&self) -> u32 {
        self.raw
    }
}

pub trait Handle: Clone + Copy {
    fn from_raw(raw: u32) -> Self;
    fn to_raw(&self) -> u32;
}
impl<T> Handle for HANDLE<T> {
    fn from_raw(raw: u32) -> Self {
        <HANDLE<T>>::from_raw(raw)
    }
    fn to_raw(&self) -> u32 {
        self.raw
    }
}

/// Maintains a mapping of HANDLE -> V, vending out new handles.
pub struct Handles<H: Handle, V> {
    map: HashMap<u32, V>,
    next: H,
}

impl<H: Handle, V> Default for Handles<H, V> {
    fn default() -> Self {
        Handles {
            map: HashMap::default(),
            next: H::from_raw(1),
        }
    }
}

impl<H: Handle, V> Handles<H, V> {
    pub fn reserve(&mut self) -> H {
        let handle = self.next;
        self.next = H::from_raw(self.next.to_raw() + 1);
        handle
    }

    pub fn set(&mut self, handle: H, t: V) {
        self.map.insert(handle.to_raw(), t);
    }

    pub fn add(&mut self, t: V) -> H {
        let handle = self.reserve();
        self.set(handle, t);
        handle
    }

    pub fn get(&self, handle: H) -> Option<&V> {
        self.map.get(&handle.to_raw())
    }

    pub fn get_mut(&mut self, handle: H) -> Option<&mut V> {
        self.map.get_mut(&handle.to_raw())
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }
}
