use crate::CPU;
use bitflags::bitflags;
use iced_x86::{Instruction, Register};
use memory::Mem;

bitflags! {
    pub struct EDXFeatures: u32 {
        const MMX = 1 << 23;
    }
}

pub fn cpuid(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    match cpu.regs.get32(Register::EAX) {
        1 => {
            // CPUID_GETFEATURES
            // Just enough to convince heaven7 that we support MMX.
            cpu.regs.set32(Register::EAX, 0);
            cpu.regs.set32(Register::ECX, 0);
            cpu.regs.set32(Register::EDX, EDXFeatures::MMX.bits());
        }
        mode => todo!("cpuid {mode}"),
    }
}
