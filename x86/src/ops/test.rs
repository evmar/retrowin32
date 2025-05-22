use super::math::{and, sub};
use crate::x86::CPU;
use iced_x86::Instruction;
use memory::{Extensions, Mem};

use super::helpers::*;

/// cmp: Compare Two Operands
pub fn cmp_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr).get();
    let y = cpu.regs.get32(instr.op1_register());
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get32(instr.op0_register());
    let y = op1_rm32(cpu, mem, instr);
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr).get();
    let y = instr.immediate32();
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr).get();
    let y = instr.immediate8to32() as u32;
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr).get();
    let y = op1_rm16(cpu, mem, instr);
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr).get();
    let y = instr.immediate16();
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr).get();
    let y = instr.immediate8to16() as u16;
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr).get();
    let y = instr.immediate8();
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr).get();
    let y = cpu.regs.get8(instr.op1_register());
    sub(x, y, &mut cpu.flags);
}

/// cmp: Compare Two Operands
pub fn cmp_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get8(instr.op0_register());
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => cpu.regs.get8(instr.op1_register()),
        iced_x86::OpKind::Memory => mem.get_pod::<u8>(x86_addr(cpu, instr)),
        _ => unreachable!(),
    };
    sub(x, y, &mut cpu.flags);
}

/// test: Logical Compare
pub fn test_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    and(x.get(), y, &mut cpu.flags);
}

/// test: Logical Compare
pub fn test_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    and(x.get(), y, &mut cpu.flags);
}

/// test: Logical Compare
pub fn test_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr).get();
    let y = cpu.regs.get16(instr.op1_register());
    and(x, y, &mut cpu.flags);
}

/// test: Logical Compare
pub fn test_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr).get();
    let y = instr.immediate16();
    and(x, y, &mut cpu.flags);
}

/// test: Logical Compare
pub fn test_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr).get();
    let y = cpu.regs.get8(instr.op1_register());
    and(x, y, &mut cpu.flags);
}

/// test: Logical Compare
pub fn test_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr).get();
    let y = instr.immediate8();
    and(x, y, &mut cpu.flags);
}
