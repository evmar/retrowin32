use iced_x86::Instruction;

use crate::Mem;
use crate::{registers::Flags, x86::CPU};

use super::math::{and, sub};

use super::helpers::*;

pub fn cmp_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm32(cpu, mem, instr);
    let y = cpu.regs.get32(instr.op1_register());
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = cpu.regs.get32(instr.op0_register());
    let y = op1_rm32(cpu, mem, instr);
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm32_imm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm32(cpu, mem, instr);
    let y = instr.immediate32();
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm32_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm32(cpu, mem, instr);
    let y = instr.immediate8to32() as u32;
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm16_rm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm16(cpu, mem, instr);
    let y = op1_rm16(cpu, mem, instr);
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm16_imm16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm16(cpu, mem, instr);
    let y = instr.immediate16();
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm16_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm16(cpu, mem, instr);
    let y = instr.immediate8to16() as u16;
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm8_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm8(cpu, mem, instr);
    let y = instr.immediate8();
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_rm8_r8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm8(cpu, mem, instr);
    let y = cpu.regs.get8(instr.op1_register());
    sub(x, y, &mut cpu.flags);
}

pub fn cmp_r8_rm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = cpu.regs.get8(instr.op0_register());
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get8(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get::<u8>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    };
    sub(x, y, &mut cpu.flags);
}

pub fn test_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    let (x, flags) = rm32(cpu, mem, instr);
    and(x.get(), y, flags);
}

pub fn test_rm32_imm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let (x, flags) = rm32(cpu, mem, instr);
    and(x.get(), y, flags);
}

pub fn test_rm16_r16(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm16(cpu, mem, instr);
    let y = cpu.regs.get16(instr.op1_register());
    and(x, y, &mut cpu.flags);
}

pub fn test_rm8_r8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm8(cpu, mem, instr);
    let y = cpu.regs.get8(instr.op1_register());
    and(x, y, &mut cpu.flags);
}

pub fn test_rm8_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm8(cpu, mem, instr);
    let y = instr.immediate8();
    and(x, y, &mut cpu.flags);
}

pub fn bt_rm32_r32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm32(cpu, mem, instr);
    let y = op1_rm32(cpu, mem, instr);
    cpu.flags.set(Flags::CF, ((x >> y) & 1) != 0);
}

pub fn bt_rm32_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let x = op0_rm32(cpu, mem, instr);
    let y = instr.immediate8() % 32;
    cpu.flags.set(Flags::CF, ((x >> y) & 1) != 0);
}

pub fn btr_rm32_imm8(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = instr.immediate8() % 32;
    let (x, flags) = rm32(cpu, mem, instr);
    flags.set(Flags::CF, ((x.get() >> y) & 1) != 0);
    x.set(x.get() & !(1 << y))
}

pub fn bsr_r32_rm32(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let (x, flags) = rm32(cpu, mem, instr);
    flags.set(Flags::ZF, y == 0);
    for i in 31..0 {
        if y & (1 << i) != 0 {
            x.set(i)
        }
    }
}
