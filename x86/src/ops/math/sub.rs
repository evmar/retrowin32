use super::int::Int;
use crate::{CPU, ops::helpers::*, registers::Flags};
use iced_x86::Instruction;
use memory::Mem;

fn sbb<I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd>(
    x: I,
    y: I,
    b: bool,
    flags: &mut Flags,
) -> I {
    let mut y = y;
    if b {
        y = y.wrapping_add(&I::one());
    }
    let (result, carry) = x.overflowing_sub(&y);
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    flags.set(Flags::CF, carry || (b && y == I::zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  1  1
    //   1  0  0
    let of = !(((x ^ y) & (x ^ result)).high_bit().is_zero());
    flags.set(Flags::OF, of);
    result
}

// pub(crate) for use in the cmp opcode impl.
pub(crate) fn sub<
    I: Int + num_traits::ops::overflowing::OverflowingSub + num_traits::WrappingAdd,
>(
    x: I,
    y: I,
    flags: &mut Flags,
) -> I {
    sbb(x, y, false, flags)
}

/// sub: Subtract
pub fn sub_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    let x = rm32(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr);
    let x = rm16(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    let x = rm16(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sub: Subtract
pub fn sub_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(sub(x.get(), y, &mut cpu.flags));
}

/// sbb: Integer Subtraction With Borrow
pub fn sbb_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let carry = cpu.flags.contains(Flags::CF);
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    x.set(sbb(x.get(), y, carry, &mut cpu.flags));
}

/// sbb: Integer Subtraction With Borrow
pub fn sbb_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let carry = cpu.flags.contains(Flags::CF);
    let y = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(sbb(x.get(), y, carry, &mut cpu.flags));
}

/// sbb: Integer Subtraction With Borrow
pub fn sbb_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let carry = cpu.flags.contains(Flags::CF);
    let y = instr.immediate8to32() as u32;
    let x = rm32(cpu, mem, instr);
    x.set(sbb(x.get(), y, carry, &mut cpu.flags));
}

/// sbb: Integer Subtraction With Borrow
pub fn sbb_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let carry = cpu.flags.contains(Flags::CF);
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    x.set(sbb(x.get(), y, carry, &mut cpu.flags));
}

/// sbb: Integer Subtraction With Borrow
pub fn sbb_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let carry = cpu.flags.contains(Flags::CF);
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(sbb(x.get(), y, carry, &mut cpu.flags));
}

/// sbb: Integer Subtraction With Borrow
pub fn sbb_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let carry = cpu.flags.contains(Flags::CF);
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(sbb(x.get(), y, carry, &mut cpu.flags));
}
