use crate::Machine;

#[win32_derive::dllexport]
pub fn InterlockedIncrement(_machine: &mut Machine, addend: Option<&mut u32>) -> u32 {
    let addend = addend.unwrap();
    *addend += 1;
    *addend
}

#[win32_derive::dllexport]
pub fn InterlockedDecrement(_machine: &mut Machine, addend: Option<&mut u32>) -> u32 {
    todo!()
}
