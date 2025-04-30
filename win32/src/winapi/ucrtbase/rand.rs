use crate::System;

// MSDN: "Calling rand before any call to srand generates the same sequence as calling srand with seed passed as 1."
static mut RAND_STATE: u32 = 1;

#[win32_derive::dllexport(cdecl)]
pub fn srand(sys: &mut dyn System, seed: u32) {
    unsafe {
        RAND_STATE = seed % (1 << 31);
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn rand(sys: &mut dyn System) -> u32 {
    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    unsafe {
        RAND_STATE = ((RAND_STATE.wrapping_mul(134775813)).wrapping_add(1)) % (1 << 31);
        RAND_STATE
    }
}
