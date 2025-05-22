use super::helpers::*;
use crate::x86::CPU;
use iced_x86::{Instruction, Register};
use memory::Mem;

/// enter: Make Stack Frame for Procedure Parameters
pub fn enterd_imm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(Register::EBP));
    cpu.regs.set32(Register::EBP, cpu.regs.get32(Register::ESP));
    *cpu.regs.get32_mut(Register::ESP) -= instr.immediate16() as u32;
}

/// leave: High Level Procedure Exit
pub fn leaved(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    cpu.regs.set32(Register::ESP, cpu.regs.get32(Register::EBP));
    let ebp = pop(cpu, mem);
    cpu.regs.set32(Register::EBP, ebp);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn pushd_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // Pushing segment registers is subtle:
    // "If the source operand is a segment register (16 bits) and [...]
    // the operand size is 32-bits, either a zero-extended value is pushed on
    // the stack or the segment selector is written on the stack using a 16-bit move.
    // For the last case, all recent Intel Core and Intel Atom processors perform a
    // 16-bit move, leaving the upper portion of the stack location unmodified."
    // tldr it's a 32-bit move in any normal case.
    let x = cpu.regs.get16(instr.op0_register());
    push(cpu, mem, x as u32);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn pushd_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate8to32() as u32);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn pushd_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, instr.immediate32());
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn push_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    push(cpu, mem, cpu.regs.get32(instr.op0_register()));
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn push_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = rm32(cpu, mem, instr).get();
    push(cpu, mem, value);
}

/// push: Push Word, Doubleword, or Quadword Onto the Stack
pub fn push_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = rm16(cpu, mem, instr).get();
    push16(cpu, mem, value);
}

/// pop: Pop a Value From the Stack
pub fn popd_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    // See discussion in pushd_r16.
    let value = pop(cpu, mem);
    cpu.regs.set16(instr.op0_register(), value as u16);
}

/// pop: Pop a Value From the Stack
pub fn pop_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = pop(cpu, mem);
    let x = rm32(cpu, mem, instr);
    x.set(value);
}

/// pop: Pop a Value From the Stack
pub fn pop_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let value = pop16(cpu, mem);
    let x = rm16(cpu, mem, instr);
    x.set(value);
}

/// pushad: Push All General-Purpose Registers
pub fn pushad(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let esp = cpu.regs.get32(Register::ESP); // get before any pushes

    push(cpu, mem, cpu.regs.get32(Register::EAX));
    push(cpu, mem, cpu.regs.get32(Register::ECX));
    push(cpu, mem, cpu.regs.get32(Register::EDX));
    push(cpu, mem, cpu.regs.get32(Register::EBX));
    push(cpu, mem, esp);
    push(cpu, mem, cpu.regs.get32(Register::EBP));
    push(cpu, mem, cpu.regs.get32(Register::ESI));
    push(cpu, mem, cpu.regs.get32(Register::EDI));
}

/// popad: Pop All General-Purpose Registers
pub fn popad(cpu: &mut CPU, mem: Mem, _instr: &Instruction) {
    let edi = pop(cpu, mem);
    cpu.regs.set32(Register::EDI, edi);
    let esi = pop(cpu, mem);
    cpu.regs.set32(Register::ESI, esi);
    let ebp = pop(cpu, mem);
    cpu.regs.set32(Register::EBP, ebp);
    pop(cpu, mem); // ignore esp
    let ebx = pop(cpu, mem);
    cpu.regs.set32(Register::EBX, ebx);
    let edx = pop(cpu, mem);
    cpu.regs.set32(Register::EDX, edx);
    let ecx = pop(cpu, mem);
    cpu.regs.set32(Register::ECX, ecx);
    let eax = pop(cpu, mem);
    cpu.regs.set32(Register::EAX, eax);
}
