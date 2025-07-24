use super::int::Int;
use crate::{CPU, ops::helpers::*, registers::Flags};
use iced_x86::Instruction;
use memory::Mem;

fn add<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I, flags: &mut Flags) -> I {
    addc(x, y, I::zero(), flags)
}

fn addc<I: Int + num_traits::ops::wrapping::WrappingAdd>(x: I, y: I, z: I, flags: &mut Flags) -> I {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let result = x.wrapping_add(&y.wrapping_add(&z));
    flags.set(Flags::CF, result < x || (result == x && !z.is_zero()));
    flags.set(Flags::ZF, result.is_zero());
    flags.set(Flags::SF, result.high_bit().is_one());
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  0  1
    //   1  1  0
    let of = ((x ^ !y) & (x ^ result)).high_bit().is_one();
    flags.set(Flags::OF, of);
    flags.set(Flags::PF, result.low_byte().count_ones() % 2 == 0);
    result
}

/// add: Add
pub fn add_r32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get32(instr.op1_register());
    let x = rm32(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm32_imm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate32();
    let x = rm32(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    let x = rm32(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm16_imm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate16();
    let x = rm16(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    let x = rm16(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm16_rm16(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm16(cpu, mem, instr);
    let x = rm16(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm8_r8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = cpu.regs.get8(instr.op1_register());
    let x = rm8(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let x = rm8(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// add: Add
pub fn add_r8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let x = rm8(cpu, mem, instr);
    x.set(add(x.get(), y, &mut cpu.flags));
}

/// adc: Add With Carry
pub fn adc_rm32_rm32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let carry = cpu.flags.contains(Flags::CF);
    let x = rm32(cpu, mem, instr);
    x.set(addc(x.get(), y, carry as u32, &mut cpu.flags));
}

/// adc: Add With Carry
pub fn adc_rm32_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    let carry = cpu.flags.contains(Flags::CF);
    let x = rm32(cpu, mem, instr);
    x.set(addc(x.get(), y, carry as u32, &mut cpu.flags));
}

/// adc: Add With Carry
pub fn adc_rm16_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    let carry = cpu.flags.contains(Flags::CF);
    let x = rm16(cpu, mem, instr);
    x.set(addc(x.get(), y, carry as u16, &mut cpu.flags));
}

/// adc: Add With Carry
pub fn adc_rm8_rm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm8(cpu, mem, instr);
    let carry = cpu.flags.contains(Flags::CF);
    let x = rm8(cpu, mem, instr);
    x.set(addc(x.get(), y, carry as u8, &mut cpu.flags));
}

/// adc: Add With Carry
pub fn adc_rm8_imm8(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = instr.immediate8();
    let carry = cpu.flags.contains(Flags::CF);
    let x = rm8(cpu, mem, instr);
    x.set(addc(x.get(), y, carry as u8, &mut cpu.flags));
}

/// xadd: Exchange and Add
pub fn xadd_rm32_r32(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    let y = op1_rm32(cpu, mem, instr);
    let x = rm32(cpu, mem, instr);
    let prev = x.get();

    let sum = add(prev, y, &mut cpu.flags);
    cpu.regs.set32(instr.op1_register(), prev);
    x.set(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addc() {
        #[track_caller]
        fn expect_add(x: u8, y: u8, carry: bool, exp: &str) {
            let mut flags = Flags::default();
            let res = addc::<u8>(x, y, carry as u8, &mut flags);
            assert_eq!(exp, format!("{:02X}{}", res, flags.debug_str()));
        }

        expect_add(0x00, 0x00, false, "00 PF ZF");
        expect_add(0x01, 0x01, false, "02");
        expect_add(0x7F, 0x01, false, "80 SF OF");
        expect_add(0x80, 0x80, false, "00 CF PF ZF OF");
        expect_add(0xFF, 0x01, false, "00 CF PF ZF");
        expect_add(0xFF, 0x00, false, "FF PF SF");
        expect_add(0x00, 0xFF, false, "FF PF SF");
        expect_add(0x80, 0x7F, false, "FF PF SF");

        // with carry
        expect_add(0x00, 0x00, true, "01");
        expect_add(0x01, 0x01, true, "03 PF");
        expect_add(0x7F, 0x01, true, "81 PF SF OF");
        expect_add(0x7F, 0x7F, true, "FF PF SF OF");
        expect_add(0x80, 0x80, true, "01 CF OF");
        expect_add(0xFF, 0x01, true, "01 CF");
        expect_add(0xFF, 0x00, true, "00 CF PF ZF");
        expect_add(0x00, 0xFF, true, "00 CF PF ZF");
        expect_add(0x80, 0x7F, true, "00 CF PF ZF");

        // carry only
        expect_add(0xFF, 0xFF, false, "FE CF SF");
        expect_add(0xFF, 0xFF, true, "FF CF PF SF");

        // overflow only
        expect_add(0x7F, 0x01, false, "80 SF OF");
        expect_add(0x7F, 0x01, true, "81 PF SF OF");

        // zero result
        expect_add(0x00, 0x00, false, "00 PF ZF");
        expect_add(0xFF, 0x01, false, "00 CF PF ZF");
        expect_add(0xFF, 0x00, true, "00 CF PF ZF");
        expect_add(0x00, 0xFF, true, "00 CF PF ZF");

        // negative result
        expect_add(0x80, 0x00, false, "80 SF");
        expect_add(0x80, 0x00, true, "81 PF SF");
        expect_add(0x80, 0x80, false, "00 CF PF ZF OF");
        expect_add(0x80, 0x80, true, "01 CF OF");
    }
}
