use crate::System;

#[win32_derive::dllexport]
pub fn InterlockedIncrement(sys: &dyn System, addend: Option<&mut u32>) -> u32 {
    let addend = addend.unwrap();
    *addend += 1;
    *addend
}

#[win32_derive::dllexport]
pub fn InterlockedDecrement(sys: &dyn System, addend: Option<&mut u32>) -> u32 {
    todo!()
}
