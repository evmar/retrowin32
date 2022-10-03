#![allow(non_snake_case)]

use crate::{winapi, X86};

fn DirectDrawCreateEx(
    _x86: &mut X86,
    _lpGuid: u32,
    _lplpDD: u32,
    _iid: u32,
    _pUnkOuter: u32,
) -> u32 {
    1 // fail
}

winapi!(
    fn DirectDrawCreateEx(lpGuid: u32, lplpDD: u32, iid: u32, pUnkOuter: u32);
);
