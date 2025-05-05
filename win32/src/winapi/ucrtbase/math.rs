use crate::Machine;
use win32_system::System;

#[win32_derive::dllexport(cdecl)]
pub fn __setusermatherr(sys: &mut dyn System, pf: u32) {
    todo!();
}

#[win32_derive::dllexport]
pub const _adjust_fdiv: &'static str = "_adjust_fdiv";

#[win32_derive::dllexport]
pub const _acmdln: &'static str = "_acmdln";

#[win32_derive::dllexport(cdecl)]
pub fn _ftol(machine: &mut Machine) -> u64 {
    // Argument is passed in st(0), and returned via edx:eax.
    // TODO: implementations online typically set the RC flags of the
    // FPU control register to 11, round towards zero.
    #[cfg(feature = "x86-emu")]
    {
        let cpu = machine.emu.x86.cpu_mut();
        let value = *cpu.fpu.st0() as i64 as u64;
        cpu.fpu.pop();
        value
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        todo!();
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn sqrt(sys: &mut dyn System, x: f64) -> f64 {
    x.sqrt()
}

#[win32_derive::dllexport(cdecl)]
pub fn sin(sys: &mut dyn System, x: f64) -> f64 {
    x.sin()
}

#[win32_derive::dllexport(cdecl)]
pub fn cos(sys: &mut dyn System, x: f64) -> f64 {
    x.cos()
}

#[win32_derive::dllexport(cdecl)]
pub fn floor(sys: &mut dyn System, x: f64) -> f64 {
    x.floor()
}
