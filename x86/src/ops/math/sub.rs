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
    let z = if b { y.wrapping_add(&I::one()) } else { y };
    let (result, borrow) = x.overflowing_sub(&z);
    flags.set(Flags::CF, borrow || (b && z == I::zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  1  1
    //   1  0  0
    let of = ((x ^ y) & (x ^ result)).high_bit().is_one();
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
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
pub fn sbb_rm8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        #[track_caller]
        fn expect_sub(x: u8, y: u8, borrow: bool, exp: &str) {
            let mut flags = Flags::default();
            let res = sbb::<u8>(x, y, borrow, &mut flags);
            assert_eq!(exp, format!("{:02X}{}", res, flags.debug_str()));
        }

        expect_sub(0x00, 0x00, false, "00 PF ZF");
        expect_sub(0x01, 0x01, false, "00 PF ZF");
        expect_sub(0x01, 0x01, true, "FF CF PF SF");
        expect_sub(0x80, 0x01, false, "7F OF");
        expect_sub(0x80, 0x00, true, "7F OF");
        expect_sub(0x80, 0x01, true, "7E PF OF");
        expect_sub(0x80, 0x80, false, "00 PF ZF");
        expect_sub(0x80, 0x80, true, "FF CF PF SF");
        expect_sub(0x00, 0x01, false, "FF CF PF SF");
        expect_sub(0x00, 0x01, true, "FE CF SF");
        expect_sub(0x7F, 0xFF, false, "80 CF SF OF");
        expect_sub(0x7F, 0xFF, true, "7F CF");
        expect_sub(0xFF, 0x01, false, "FE SF");
        expect_sub(0xFF, 0x01, true, "FD SF");
        expect_sub(0xFF, 0xFF, false, "00 PF ZF");
        expect_sub(0xFF, 0xFF, true, "FF CF PF SF");
    }
}
