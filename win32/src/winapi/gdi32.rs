#![allow(non_snake_case)]

use crate::{winapi, X86};

fn GetStockObject(_x86: &mut X86, _i: u32) -> u32 {
    0
}

winapi!(
    fn GetStockObject(i: u32);
);
