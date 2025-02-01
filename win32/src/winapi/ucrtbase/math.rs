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
pub fn _ftol(machine: &mut Machine) -> u32 {
    // Argument is passed in st(0), and returned via edx:eax.
    #[cfg(feature = "x86-emu")]
    {
        let cpu = machine.emu.x86.cpu_mut();
        let value = *cpu.fpu.st0() as i64 as u64;
        cpu.fpu.pop();
        cpu.regs.set32(x86::Register::EDX, (value >> 32) as u32);
        value as u32
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        todo!();
    }
}
