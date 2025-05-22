use super::int::Int;
use crate::{ops::helpers::*, registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::Mem;

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
