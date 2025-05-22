use super::int::Int;
use crate::{ops::helpers::*, registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::Mem;
use num_traits::{
    Signed,
    ops::overflowing::{OverflowingMul, OverflowingSub},
};

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

/// shl: Shift
fn shl<I: Int + num_traits::WrappingShl>(x: I, y: u8, flags: &mut Flags) -> I {
    let y = y % 32;
    if y == 0 {
        return x;
    }

    // Carry is the highest bit that will be shifted out.
    let cf = (x >> (I::bits() - y as usize) & I::one()).is_one();
    let val = x.wrapping_shl(y.as_usize() as u32);
    flags.set(Flags::CF, cf);
    let msb = val.high_bit().is_one();
    flags.set(Flags::SF, msb);
    // Note: OF only defined for 1-bit rotates.
    // "For left shifts, the OF flag is set to 0 if the mostsignificant bit of the result is the
    // same as the CF flag (that is, the top two bits of the original operand were the same) [...]"
    flags.set(
        Flags::OF,
        x.shr(I::bits() - 1).is_one() ^ (x.shr(I::bits() - 2) & I::one()).is_one(),
    );
    flags.set(Flags::ZF, val.is_zero());

    val
}

/// shl: Shift
pub fn shl_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm32(cpu, mem, instr);
    x.set(shl(x.get(), y, &mut cpu.flags));
}

/// shl: Shift
pub fn shl_rm32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm32(cpu, mem, instr);
    x.set(shl(x.get(), y, &mut cpu.flags));
}

/// shl: Shift
pub fn shl_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm16(cpu, mem, instr);
    x.set(shl(x.get(), y, &mut cpu.flags));
}

/// shl: Shift
pub fn shl_rm8_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm8(cpu, mem, instr);
    x.set(shl(x.get(), y, &mut cpu.flags));
}

/// shl: Shift
pub fn shl_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(shl(x.get(), y, &mut cpu.flags));
}

/// shl: Shift
pub fn shl_rm8_1(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr);
    x.set(shl(x.get(), 1, &mut cpu.flags));
}

/// shld: Double Precision Shift Left
fn shld(x: Arg<u32>, y: u32, count: u8, flags: &mut Flags) {
    let count = count % 32;
    if count == 0 {
        return;
    }
    let val = x.get();
    // "CF flag is filled with the last bit shifted out of the destination operand"
    flags.set(Flags::CF, ((val >> (32 - count)) & 1) != 0);
    if count == 1 {
        // "OF flag is set if a sign change occurred"
        flags.set(Flags::OF, (val >> 31) != ((val >> 30) & 1));
    }
    x.set((val << count) | (y >> (32 - count)));
}

/// shld: Double Precision Shift Left
pub fn shld_rm32_r32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let count = instr.immediate8();
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    shld(x, y, count, &mut cpu.flags);
}

/// shld: Double Precision Shift Left
pub fn shld_rm32_r32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let count = cpu.regs.get8(Register::CL);
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    shld(x, y, count, &mut cpu.flags);
}

/// shr: Shift
fn shr<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    // In all modes but 64 it is correct to mask to 32 bits.
    assert!(I::bits() < 64); // 64 not implemented
    let y = y % 32;

    if y == 0 {
        return x; // Don't affect flags.
    }

    flags.set(Flags::CF, ((x >> (y - 1) as usize) & I::one()).is_one());
    let val = x >> y as usize;
    flags.set(Flags::SF, false); // ?
    flags.set(Flags::ZF, val.is_zero());

    // Note: OF state undefined for shifts > 1 bit.
    flags.set(Flags::OF, x.high_bit().is_one());
    val
}

/// shr: Shift
pub fn shr_rm32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm32(cpu, mem, instr);
    x.set(shr(x.get(), y, &mut cpu.flags));
}

/// shr: Shift
pub fn shr_rm32_1(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr);
    x.set(shr(x.get(), 1, &mut cpu.flags));
}

/// shr: Shift
pub fn shr_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm32(cpu, mem, instr);
    x.set(shr(x.get(), y, &mut cpu.flags));
}

/// shr: Shift
pub fn shr_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm16(cpu, mem, instr);
    x.set(shr(x.get(), y, &mut cpu.flags));
}

/// shr: Shift
pub fn shr_rm16_1(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr);
    x.set(shr(x.get(), 1, &mut cpu.flags));
}

/// shr: Shift
pub fn shr_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(shr(x.get(), y, &mut cpu.flags));
}

/// shr: Shift
pub fn shr_rm8_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm8(cpu, mem, instr);
    x.set(shr(x.get(), y, &mut cpu.flags));
}

/// shrd: Double Precision Shift Right
fn shrd(x: Arg<u32>, y: u32, count: u8, flags: &mut Flags) {
    let count = count % 32;
    if count == 0 {
        return;
    }
    let val = x.get();
    // "CF flag is filled with the last bit shifted out of the destination operand"
    flags.set(Flags::CF, ((val >> (count - 1)) & 1) != 0);
    if count == 1 {
        // "OF flag is set if a sign change occurred"
        flags.set(Flags::OF, (val >> 31) != (y & 1));
    }
    x.set((val >> count) | (y << (32 - count)));
}

/// shrd: Double Precision Shift Right
pub fn shrd_rm32_r32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let count = instr.immediate8();
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    shrd(x, y, count, &mut cpu.flags);
}

/// shrd: Double Precision Shift Right
pub fn shrd_rm32_r32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let count = cpu.regs.get8(Register::CL);
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    shrd(x, y, count, &mut cpu.flags);
}

/// sar: Shift
fn sar<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    if y == 0 {
        return x;
    }
    flags.set(Flags::CF, x.shr(y as usize - 1).bitand(I::one()).is_one());
    // Note: OF only defined for 1-bit rotates.
    flags.set(Flags::OF, false);
    // There's a random "u32" type in the num-traits signed_shr signature, so cast here.
    let result = x.signed_shr(y as u32);

    flags.set(Flags::SF, result.high_bit().is_one());
    flags.set(Flags::ZF, result.is_zero());
    result
}

/// sar: Shift
pub fn sar_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm32(cpu, mem, instr);
    x.set(sar(x.get(), y, &mut cpu.flags));
}

/// sar: Shift
pub fn sar_rm32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm32(cpu, mem, instr);
    x.set(sar(x.get(), y, &mut cpu.flags));
}

/// sar: Shift
pub fn sar_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(sar(x.get(), y, &mut cpu.flags));
}

/// sar: Shift
pub fn sar_rm8_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm8(cpu, mem, instr);
    x.set(sar(x.get(), y, &mut cpu.flags));
}

/// rol: Rotate
fn rol<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    if y == 0 {
        return x;
    }
    let result = x.rotate_left(y as u32);
    let carry = (result & I::one()).is_one();
    flags.set(Flags::CF, carry);
    // Note: OF only defined for 1-bit rotates.
    flags.set(Flags::OF, carry ^ (result.high_bit()).is_one());
    result
}

/// rol: Rotate
pub fn rol_rm32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm32(cpu, mem, instr);
    x.set(rol(x.get(), y, &mut cpu.flags));
}

/// rol: Rotate
pub fn rol_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm32(cpu, mem, instr);
    x.set(rol(x.get(), y, &mut cpu.flags));
}

/// rol: Rotate
pub fn rol_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm16(cpu, mem, instr);
    x.set(rol(x.get(), y, &mut cpu.flags));
}

/// rol: Rotate
pub fn rol_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(rol(x.get(), y, &mut cpu.flags));
}

/// rol: Rotate
pub fn rol_rm8_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm8(cpu, mem, instr);
    x.set(rol(x.get(), y, &mut cpu.flags));
}

/// ror: Rotate
fn ror<I: Int>(x: I, y: u8, flags: &mut Flags) -> I {
    if y == 0 {
        return x;
    }
    let result = x.rotate_right(y as u32);
    let msb = result.high_bit().is_one();
    flags.set(Flags::CF, msb);
    // Note: OF only defined for 1-bit rotates.
    flags.set(Flags::OF, msb ^ (result >> (I::bits() - 2)).is_one());
    result
}

/// ror: Rotate
pub fn ror_rm32_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm32(cpu, mem, instr);
    x.set(ror(x.get(), y, &mut cpu.flags));
}

/// ror: Rotate
pub fn ror_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm32(cpu, mem, instr);
    x.set(ror(x.get(), y, &mut cpu.flags));
}

/// ror: Rotate
pub fn ror_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(ror(x.get(), y, &mut cpu.flags));
}

/// ror: Rotate
pub fn ror_rm8_cl(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(Register::CL);
    let x = rm8(cpu, mem, instr);
    x.set(ror(x.get(), y, &mut cpu.flags));
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

/// Shared impl of mul_rmXX.  The trick is to pass in a higher width int,
/// e.g. x as u32 for the 16-bit mul, so there is enough space in the result.
fn mul<I: Int>(x: I, y: I, flags: &mut Flags) -> I {
    let res = x.mul(y);
    let tophalf = res.shr(I::bits() / 2);
    flags.set(Flags::OF, !tophalf.is_zero());
    flags.set(Flags::CF, !tophalf.is_zero());
    res
}

/// mul: Unsigned Multiply
pub fn mul_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = rm32(cpu, mem, instr).get();
    let x = cpu.regs.get32(Register::EAX);
    let res = mul(x as u64, y as u64, &mut cpu.flags);
    set_edx_eax(cpu, res);
}

/// mul: Unsigned Multiply
pub fn mul_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = rm16(cpu, mem, instr).get();
    let x = cpu.regs.get16(Register::AX);
    let res = mul(x as u32, y as u32, &mut cpu.flags);
    set_dx_ax(cpu, res);
}

/// mul: Unsigned Multiply
pub fn mul_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = rm8(cpu, mem, instr).get();
    let x = cpu.regs.get8(Register::AL);
    let res = mul(x as u16, y as u16, &mut cpu.flags);
    cpu.regs.set16(Register::AX, res);
}

/// imul: Signed Multiply
pub fn imul_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm32(cpu, mem, instr).get() as i32;
    let y = cpu.regs.get32(Register::EAX) as i32;
    let res = (x as i64).wrapping_mul(y as i64) as u64;
    // TODO: flags.
    set_edx_eax(cpu, res);
}

/// imul: Signed Multiply
pub fn imul_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm16(cpu, mem, instr).get() as i16;
    let y = cpu.regs.get16(Register::AX) as i16;
    let res = (x as i32).wrapping_mul(y as i32) as u32;
    // TODO: flags.
    set_dx_ax(cpu, res);
}

/// imul: Signed Multiply
pub fn imul_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = rm8(cpu, mem, instr).get() as i8;
    let y = cpu.regs.get8(Register::AL) as i8;
    let res = (x as i16).wrapping_mul(y as i16) as u16;
    // TODO: flags.
    cpu.regs.set16(Register::AX, res);
}

/// imul: Signed Multiply
fn imul_trunc<I: OverflowingMul + Signed>(x: I, y: I, flags: &mut Flags) -> I {
    let (result, flag) = x.overflowing_mul(&y);

    flags.set(Flags::OF, flag);
    flags.set(Flags::CF, flag);

    result
}

/// imul: Signed Multiply
pub fn imul_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get32(instr.op0_register()) as i32;
    let y = op1_rm32(cpu, mem, instr) as i32;
    let value = imul_trunc(x, y, &mut cpu.flags);
    cpu.regs.set32(instr.op0_register(), value as u32);
}

/// imul: Signed Multiply
pub fn imul_r32_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = op1_rm32(cpu, mem, instr) as i32;
    let y = instr.immediate32() as i32;
    let value = imul_trunc(x, y, &mut cpu.flags);
    cpu.regs.set32(instr.op0_register(), value as u32);
}

/// imul: Signed Multiply
pub fn imul_r32_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = op1_rm32(cpu, mem, instr) as i32;
    let y = instr.immediate8to32();
    let value = imul_trunc(x, y, &mut cpu.flags);
    cpu.regs.set32(instr.op0_register(), value as u32);
}

/// imul: Signed Multiply
pub fn imul_r16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get16(instr.op0_register()) as i16;
    let y = op1_rm16(cpu, mem, instr) as i16;
    let value = imul_trunc(x, y, &mut cpu.flags);
    cpu.regs.set16(instr.op0_register(), value as u16);
}

/// idiv: Signed Divide
pub fn idiv_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_edx_eax(cpu) as i64;
    let y = rm32(cpu, mem, instr).get() as i32 as i64;
    cpu.regs.set32(Register::EAX, (x / y) as i32 as u32);
    cpu.regs.set32(Register::EDX, (x % y) as i32 as u32);
}

/// idiv: Signed Divide
pub fn idiv_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_dx_ax(cpu) as i32;
    let y = rm16(cpu, mem, instr).get() as i16 as i32;
    let quotient = x / y;
    if quotient > 0x7FFF || quotient < -0x8000 {
        panic!("divide error");
    }
    cpu.regs.set16(Register::AX, quotient as i16 as u16);
    cpu.regs.set16(Register::DX, (x % y) as u16);
}

/// idiv: Signed Divide
pub fn idiv_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get16(Register::AX) as i16;
    let y = rm8(cpu, mem, instr).get() as i8 as i16;
    let quotient = x / y;
    if quotient > 0x7F || quotient < -0x80 {
        panic!("divide error");
    }
    let rem = x % y;
    cpu.regs
        .set16(Register::AX, ((rem << 8) as u16) | (quotient as i8 as u16));
}

/// div: Unsigned Divide
pub fn div_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_edx_eax(cpu);
    let y = rm32(cpu, mem, instr).get() as u64;
    cpu.regs.set32(Register::EAX, (x / y) as u32);
    cpu.regs.set32(Register::EDX, (x % y) as u32);
    // No flags.
}

/// div: Unsigned Divide
pub fn div_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = get_dx_ax(cpu);
    let y = rm16(cpu, mem, instr).get() as u32;
    cpu.regs.set32(Register::EAX, ((x / y) as u16) as u32);
    cpu.regs.set32(Register::EDX, ((x % y) as u16) as u32);
    // No flags.
}

/// div: Unsigned Divide
pub fn div_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let x = cpu.regs.get16(Register::AX);
    let y = rm8(cpu, mem, instr).get() as u16;
    cpu.regs
        .set32(Register::EAX, (((x % y) as u32) << 16) | ((x / y) as u32));
    // No flags.
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
