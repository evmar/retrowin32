use crate::Machine;

#[win32_derive::dllexport(cdecl)]
pub fn __setusermatherr(machine: &mut Machine, pf: u32) {
    todo!();
}

#[win32_derive::dllexport]
pub const _adjust_fdiv: &'static str = "_adjust_fdiv";

#[win32_derive::dllexport]
pub const _acmdln: &'static str = "_acmdln";

#[win32_derive::dllexport(cdecl)]
pub fn _ftol(machine: &mut Machine) -> u64 {
    // Argument is passed in st(0), and returned via edx:eax.
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
pub fn sqrt(machine: &mut Machine, x: f64) -> f64 {
    x.sqrt()
}

#[win32_derive::dllexport(cdecl)]
pub fn sin(machine: &mut Machine, x: f64) -> f64 {
    x.sin()
}

#[win32_derive::dllexport(cdecl)]
pub fn cos(machine: &mut Machine, x: f64) -> f64 {
    x.cos()
}
