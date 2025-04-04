use crate::CPU;
use bitflags::bitflags;
use iced_x86::{Instruction, Register};
use memory::Mem;

bitflags! {
    pub struct EDXFeatures: u32 {
        const MMX = 1 << 23;
    }
}

/// cpuid: CPU Identification
pub fn cpuid(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    match cpu.regs.get32(Register::EAX) {
        0 => {
            // basic information
            cpu.regs.set32(Register::EAX, /* Pentium */ 0x2);
            // "GenuineIntel"
            cpu.regs.set32(Register::EBX, u32::from_le_bytes(*b"Genu"));
            cpu.regs.set32(Register::EDX, u32::from_le_bytes(*b"ineI"));
            cpu.regs.set32(Register::ECX, u32::from_le_bytes(*b"ntel"));
        }
        1 => {
            // CPUID_GETFEATURES
            // Just enough to convince heaven7 that we support MMX.
            cpu.regs.set32(Register::EAX, 0);
            cpu.regs.set32(Register::ECX, 0);
            cpu.regs.set32(Register::EDX, EDXFeatures::MMX.bits());
        }
        0x8000_0000 => {
            // maximum extended function
            cpu.regs.set32(Register::EAX, 0);
        }
        mode => todo!("cpuid {mode:x}"),
    }
}
