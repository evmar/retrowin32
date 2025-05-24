use super::helpers::*;
use crate::{CPU, registers::Flags};
use iced_x86::{Instruction, Register};
use memory::Mem;

/// seta: Set Byte on Condition
pub fn seta_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (!cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// setae: Set Byte on Condition
pub fn setae_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::CF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// set: Set Byte on Condition
pub fn setb_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::CF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// setbe: Set Byte on Condition
pub fn setbe_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::CF) || cpu.flags.contains(Flags::ZF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// sete: Set Byte on Condition
pub fn sete_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::ZF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// setg: Set Byte on Condition
pub fn setg_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (!cpu.flags.contains(Flags::ZF)
        && (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF))) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// setl: Set Byte on Condition
pub fn setl_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// setle: Set Byte on Condition
pub fn setle_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::ZF)
        || (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF))) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// set: Set Byte on Condition
pub fn setne_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::ZF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// setge: Set Byte on Condition
pub fn setge_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// sets: Set Byte on Condition
pub fn sets_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::SF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

/// pushfd: Push EFLAGS Register Onto the Stack
pub fn pushfd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    push(cpu, mem, cpu.flags.bits());
}

/// pushfw: Push EFLAGS Register Onto the Stack
pub fn pushfw(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let value = (cpu.flags.bits() & 0x0000_FFFF) as u16;
    push16(cpu, mem, value);
}

/// popfd: Pop Stack Into EFLAGS Register
pub fn popfd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let value = pop(cpu, mem) & 0x0000_FFFF;
    cpu.flags = Flags::from_bits(value).unwrap_or_else(|| panic!("invalid flags {:#x}", value));
}

/// popfw: Pop Stack Into EFLAGS Register
pub fn popfw(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let prev = Flags::from_bits(cpu.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(pop16(cpu, mem) as u32).unwrap();
    cpu.flags = prev.union(new);
}

/// sahf: Store AH Into Flags
pub fn sahf(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    // This constructs flags from the AH register, but only specific flags.
    let flags = Flags::from_bits(cpu.regs.get8(Register::AH) as u32).unwrap();
    cpu.flags.set(Flags::CF, flags.contains(Flags::CF));
    // cpu.flags.set(Flags::PF, flags.contains(Flags::PF));
    // cpu.flags.set(Flags::AF, flags.contains(Flags::AF));
    cpu.flags.set(Flags::ZF, flags.contains(Flags::ZF));
    cpu.flags.set(Flags::OF, flags.contains(Flags::OF));
}

/// lahf: Load Status Flags Into AH Register
pub fn lahf(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.regs.set8(Register::AH, cpu.flags.bits() as u8);
}

/// salc: Set AL on Carry
pub fn salc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.regs.set8(
        iced_x86::Register::AL,
        if cpu.flags.contains(Flags::CF) {
            0xFF
        } else {
            0
        },
    );
}

/// std: Set Direction Flag
pub fn std(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::DF);
}

/// cld: Clear Direction Flag
pub fn cld(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::DF);
}

/// stc: Set Carry Flag
pub fn stc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::CF);
}

/// clc: Clear Carry Flag
pub fn clc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::CF);
}

/// cmc: Complement Carry Flag
pub fn cmc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.set(Flags::CF, !cpu.flags.contains(Flags::CF));
}
