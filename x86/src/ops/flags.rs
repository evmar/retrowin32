use super::helpers::*;
use crate::{registers::Flags, CPU};
use iced_x86::{Instruction, Register};
use memory::Mem;

pub fn seta_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (!cpu.flags.contains(Flags::CF) && !cpu.flags.contains(Flags::ZF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setae_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::CF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setb_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::CF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setbe_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::CF) || cpu.flags.contains(Flags::ZF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn sete_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::ZF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setg_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (!cpu.flags.contains(Flags::ZF)
        && (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF))) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setl_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setle_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::ZF)
        || (cpu.flags.contains(Flags::SF) != cpu.flags.contains(Flags::OF))) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setne_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = !cpu.flags.contains(Flags::ZF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn setge_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = (cpu.flags.contains(Flags::SF) == cpu.flags.contains(Flags::OF)) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn sets_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = cpu.flags.contains(Flags::SF) as u8;
    let x = rm8(cpu, mem, instr);
    x.set(value);
}

pub fn pushfd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    push(cpu, mem, cpu.flags.bits());
}

pub fn pushfw(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let value = (cpu.flags.bits() & 0x0000_FFFF) as u16;
    push16(cpu, mem, value);
}

pub fn popfd(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let value = pop(cpu, mem) & 0x0000_FFFF;
    cpu.flags = Flags::from_bits(value).unwrap_or_else(|| panic!("invalid flags {:#x}", value));
}

pub fn popfw(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let prev = Flags::from_bits(cpu.flags.bits() & 0xFFFF_0000).unwrap();
    let new = Flags::from_bits(pop16(cpu, mem) as u32).unwrap();
    cpu.flags = prev.union(new);
}

pub fn sahf(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    let ah = cpu.regs.get8(Register::AH);
    cpu.flags = Flags::from_bits((cpu.flags.bits() & 0xFFFF_FF00) | ah as u32).unwrap();
}

pub fn lahf(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.regs.set8(Register::AH, cpu.flags.bits() as u8);
}

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

pub fn std(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::DF);
}

pub fn cld(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::DF);
}

pub fn stc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.insert(Flags::CF);
}

pub fn clc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.remove(Flags::CF);
}

pub fn cmc(cpu: &mut CPU, _mem: Mem, _instr: &Instruction) {
    cpu.flags.set(Flags::CF, !cpu.flags.contains(Flags::CF));
}
