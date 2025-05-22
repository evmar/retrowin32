use super::int::Int;
use crate::{ops::helpers::*, registers::Flags, x86::CPU};
use iced_x86::Instruction;
use memory::Mem;
use num_traits::ops::overflowing::OverflowingSub;

// pub(crate) for use in the test opcode impl.
pub(crate) fn and<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x & y;
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::OF, false);
    flags.set(Flags::CF, false);
    result
}

/// and: Logical AND
pub fn and_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    let x = rm32(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm16_r16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr);
    let x = rm16(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_r16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr);
    let x = rm16(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    let x = rm16(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// and: Logical AND
pub fn and_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(and(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
fn or<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x | y;
    flags.remove(Flags::OF | Flags::CF);
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::ZF, result.is_zero());
    result
}

/// or: Logical Inclusive OR
pub fn or_rm32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    let x = rm32(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    let x = rm16(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr);
    let x = rm16(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// or: Logical Inclusive OR
pub fn or_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(or(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
fn xor<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let result = x ^ y;
    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is undefined.
    flags.remove(Flags::OF);
    flags.remove(Flags::CF);
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    result
}

/// xor: Logical Exclusive OR
pub fn xor_rm32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    let x = rm32(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_rm16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr);
    let x = rm16(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    let x = rm16(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// xor: Logical Exclusive OR
pub fn xor_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(instr.op1_register());
    let x = rm8(cpu, mem, instr);
    x.set(xor(x.get(), y, &mut cpu.flags));
}

/// dec: Decrement by 1
fn dec<I: Int + num_traits::WrappingSub>(x: I, flags: &mut Flags) -> I {
    // Note this is not sub(1) because CF should be preserved.
    let result = x.wrapping_sub(&I::one());
    flags.set(Flags::OF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::ZF, result.is_zero());
    result
}

/// dec: Decrement by 1
pub fn dec_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(dec(x.get(), &mut cpu.flags));
}

/// dec: Decrement by 1
pub fn dec_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr);
    x.set(dec(x.get(), &mut cpu.flags));
}

/// dec: Decrement by 1
pub fn dec_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr);
    x.set(dec(x.get(), &mut cpu.flags));
}

/// inc: Increment by 1
fn inc<I: Int + num_traits::WrappingAdd>(x: I, flags: &mut Flags) -> I {
    // Note this is not add(1) because CF should be preserved.
    let result = x.wrapping_add(&I::one());
    flags.set(Flags::OF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::ZF, result.is_zero());
    result
}

/// inc: Increment by 1
pub fn inc_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(inc(x.get(), &mut cpu.flags));
}

/// inc: Increment by 1
pub fn inc_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr);
    x.set(inc(x.get(), &mut cpu.flags));
}

/// inc: Increment by 1
pub fn inc_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr);
    x.set(inc(x.get(), &mut cpu.flags));
}

/// neg: Two's Complement Negation
fn neg<I: Int + OverflowingSub>(x: I, flags: &mut Flags) -> I {
    let (res, of) = I::zero().overflowing_sub(&x);
    flags.set(Flags::ZF, res.is_zero());
    flags.set(Flags::CF, !res.is_zero());
    flags.set(Flags::OF, of);
    res
}

/// neg: Two's Complement Negation
pub fn neg_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(neg(x.get(), &mut cpu.flags));
}

/// neg: Two's Complement Negation
pub fn neg_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr);
    x.set(neg(x.get(), &mut cpu.flags));
}

/// neg: Two's Complement Negation
pub fn neg_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr);
    x.set(neg(x.get(), &mut cpu.flags));
}

/// not: One's Complement Negation
pub fn not_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(!x.get())
}

/// not: One's Complement Negation
pub fn not_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr);
    x.set(!x.get())
}

/// not: One's Complement Negation
pub fn not_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr);
    x.set(!x.get())
}
