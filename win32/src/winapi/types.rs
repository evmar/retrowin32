//! Types exposed by the Windows API.

use crate::memory::Pod;

use super::shims::{FromX86, ToX86};

pub type WORD = u16;
pub type DWORD = u32;

// Handles like HFILE etc. are just u32s.
// I looked at using PhantomData to declare distinct types but I think using
// a macro generates an equivalent amount of code and it's way less confusing
// to work with.
macro_rules! declare_handle {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq, Clone, Copy)]
        #[repr(transparent)]
        pub struct $name(pub u32);
        unsafe impl Pod for $name {}
        impl FromX86 for $name {
            fn from_raw(raw: u32) -> Self {
                $name(raw)
            }
        }
        impl ToX86 for $name {
            fn to_raw(&self) -> u32 {
                self.0
            }
        }
    };
}

declare_handle!(HFILE);
declare_handle!(HMODULE);
