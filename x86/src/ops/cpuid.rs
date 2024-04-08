use crate::CPU;
use bitflags::bitflags;
use iced_x86::Instruction;
use memory::Mem;

bitflags! {
    pub struct EDXFeatures: u32 {
        const MMX = 1 << 23;
    }
}

pub fn cpuid(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    match cpu.regs.eax {
        1 => {
            // CPUID_GETFEATURES
            // Just enough to convince heaven7 that we support MMX.
            cpu.regs.eax = 0;
            cpu.regs.ecx = 0;
            cpu.regs.edx = EDXFeatures::MMX.bits();
        }
        mode => todo!("cpuid {mode}"),
    }
}
