use iced_x86::Instruction;

use crate::{registers::Flags, x86::X86};

fn add32(x86: &mut X86, x: u32, y: u32) -> u32 {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let (result, carry) = x.overflowing_add(y);
    x86.regs.flags.set(Flags::CF, carry);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  0  1
    //   1  1  0
    let of = ((x ^ !y) & (x ^ result)) >> 31 != 0;
    x86.regs.flags.set(Flags::OF, of);
    result
}

fn add16(x86: &mut X86, x: u16, y: u16) -> u16 {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let (result, carry) = x.overflowing_add(y);
    x86.regs.flags.set(Flags::CF, carry);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x8000 != 0);
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  0  1
    //   1  1  0
    let of = ((x ^ !y) & (x ^ result)) >> 15 != 0;
    x86.regs.flags.set(Flags::OF, of);
    result
}

fn add8(x86: &mut X86, x: u8, y: u8) -> u8 {
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    let (result, carry) = x.overflowing_add(y);
    x86.regs.flags.set(Flags::CF, carry);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x80 != 0);
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  0  1
    //   1  1  0
    let of = ((x ^ !y) & (x ^ result)) >> 7 != 0;
    x86.regs.flags.set(Flags::OF, of);
    result
}

pub fn sub32(x86: &mut X86, x: u32, y: u32) -> u32 {
    let (result, carry) = x.overflowing_sub(y);
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    x86.regs.flags.set(Flags::CF, carry);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
    // Overflow is true exactly when the high (sign) bits are like:
    //   x  y  result
    //   0  1  1
    //   1  0  0
    let of = ((x ^ y) & (x ^ result)) >> 31 != 0;
    x86.regs.flags.set(Flags::OF, of);
    result
}

pub fn sub16(x86: &mut X86, x: u16, y: u16) -> u16 {
    let (result, carry) = x.overflowing_sub(y);
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    x86.regs.flags.set(Flags::CF, carry);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x80 != 0);
    // See comment in sub32.
    let of = ((x ^ y) & (x ^ result)) >> 7 != 0;
    x86.regs.flags.set(Flags::OF, of);
    result
}

pub fn sub8(x86: &mut X86, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_sub(y);
    // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
    x86.regs.flags.set(Flags::CF, carry);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x80 != 0);
    // See comment in sub32.
    let of = ((x ^ y) & (x ^ result)) >> 7 != 0;
    x86.regs.flags.set(Flags::OF, of);
    result
}

pub fn and32(x86: &mut X86, x: u32, y: u32) -> u32 {
    let result = x & y;
    // XXX More flags.
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
    x86.regs.flags.set(Flags::OF, false);
    result
}

pub fn and16(x86: &mut X86, x: u16, y: u16) -> u16 {
    let result = x & y;
    // XXX More flags.
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x8000 != 0);
    x86.regs.flags.set(Flags::OF, false);
    result
}

pub fn and8(x86: &mut X86, x: u8, y: u8) -> u8 {
    let result = x & y;
    // XXX More flags.
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x80 != 0);
    x86.regs.flags.set(Flags::OF, false);
    result
}

fn or32(x86: &mut X86, x: u32, y: u32) -> u32 {
    let result = x | y;
    // XXX More flags.
    x86.regs.flags.set(Flags::ZF, result == 0);
    result
}

fn or16(x86: &mut X86, x: u16, y: u16) -> u16 {
    let result = x | y;
    // XXX More flags.
    x86.regs.flags.set(Flags::ZF, result == 0);
    result
}

fn or8(x86: &mut X86, x: u8, y: u8) -> u8 {
    let result = x | y;
    // XXX More flags.
    x86.regs.flags.set(Flags::ZF, result == 0);
    result
}

fn shl32(x86: &mut X86, x: u32, y: u8) -> u32 {
    // Note: overflowing_shl is not what we want.
    let val = (x as u64).wrapping_shl(y as u32);
    x86.regs.flags.set(Flags::CF, val & (1 << 32) != 0);
    val as u32
}

fn shl8(x86: &mut X86, x: u8, y: u8) -> u8 {
    // Note: overflowing_shl is not what we want.
    let val = (x as u16).wrapping_shl(y as u32);
    x86.regs.flags.set(Flags::CF, val & (1 << 8) != 0);
    val as u8
}

pub fn and_rm32_imm32(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate32();
    x86.rm32_x(instr, |x86, x| and32(x86, x, y));
}

pub fn and_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    x86.rm32_x(instr, |x86, x| and32(x86, x, y));
}

pub fn and_r32_rm32(x86: &mut X86, instr: &Instruction) {
    let reg = instr.op0_register();
    let y = x86.op1_rm32(instr);
    let value = x86.regs.get32(reg) & y;
    x86.regs.set32(reg, value);
}

pub fn and_rm16_imm16(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate16();
    x86.rm16_x(instr, |x86, x| and16(x86, x, y));
}

pub fn and_rm8_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8();
    x86.rm8_x(instr, |x86, x| and8(x86, x, y));
}

pub fn or_r32_rm32(x86: &mut X86, instr: &Instruction) {
    let y = x86.op1_rm32(instr);
    x86.rm32_x(instr, |x86, x| or32(x86, x, y));
}

pub fn or_rm32_imm32(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate32();
    x86.rm32_x(instr, |x86, x| or32(x86, x, y));
}

pub fn or_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    x86.rm32_x(instr, |x86, x| or32(x86, x, y));
}

pub fn or_rm16_imm16(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate16();
    x86.rm16_x(instr, |x86, x| or16(x86, x, y));
}

pub fn or_rm8_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8();
    x86.rm8_x(instr, |x86, x| or8(x86, x, y));
}

pub fn shl_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8();
    x86.rm32_x(instr, |x86, x| shl32(x86, x, y));
}

pub fn shl_rm32_cl(x86: &mut X86, instr: &Instruction) {
    let y = x86.regs.ecx as u8;
    x86.rm32_x(instr, |x86, x| shl32(x86, x, y));
}

pub fn shl_rm8_cl(x86: &mut X86, instr: &Instruction) {
    let y = x86.regs.ecx as u8;
    x86.rm8_x(instr, |x86, x| shl8(x86, x, y));
}

pub fn shr_rm32_1(x86: &mut X86, instr: &Instruction) {
    x86.rm32_x(instr, |_x86, x| x >> 1);
}

pub fn shr_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8() as u32;
    x86.rm32_x(instr, |_x86, x| x >> y);
}

pub fn sar_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8() as u32;
    x86.rm32_x(instr, |_x86, x| (x >> y) | (x & 0x8000_0000));
}

pub fn sar_rm32_cl(x86: &mut X86, instr: &Instruction) {
    let y = x86.regs.ecx as u8;
    x86.rm32_x(instr, |_x86, x| (x >> y) | (x & 0x8000_0000));
}

pub fn ror_rm32_cl(x86: &mut X86, instr: &Instruction) {
    let y = x86.regs.ecx as u8;
    x86.rm32_x(instr, |x86, x| {
        let out = x.rotate_right(y as u32);
        let msb = (out & 0x8000_0000) != 0;
        x86.regs.flags.set(Flags::CF, msb);
        x86.regs
            .flags
            .set(Flags::OF, msb ^ ((out & 04000_0000) != 0));
        out
    });
}

fn xor32(x86: &mut X86, x: u32, y: u32) -> u32 {
    let result = x ^ y;
    // The OF and CF flags are cleared; the SF, ZF, and PF flags are set according to the result. The state of the AF flag is undefined.
    x86.regs.flags.remove(Flags::OF);
    x86.regs.flags.remove(Flags::CF);
    x86.regs.flags.set(Flags::ZF, result == 0);
    x86.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
    result
}

pub fn xor_rm32_rm32(x86: &mut X86, instr: &Instruction) {
    let y = x86.op1_rm32(instr);
    x86.rm32_x(instr, |x86, x| xor32(x86, x, y));
}

pub fn xor_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    x86.rm32_x(instr, |x86, x| xor32(x86, x, y));
}

pub fn xor_rm8_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8();
    x86.rm8_x(instr, |_x86, x| x ^ y);
    // TODO: flags
}

pub fn xor_r8_rm8(x86: &mut X86, instr: &Instruction) {
    let y = x86.op1_rm8(instr);
    x86.rm8_x(instr, |_x86, x| x ^ y);
    // TODO: flags
}

pub fn add_r32_rm32(x86: &mut X86, instr: &Instruction) {
    let reg = instr.op0_register();
    let value = add32(x86, x86.regs.get32(reg), x86.op1_rm32(&instr));
    x86.regs.set32(reg, value);
}

pub fn add_rm32_r32(x86: &mut X86, instr: &Instruction) {
    let y = x86.regs.get32(instr.op1_register());
    x86.rm32_x(instr, |x86, x| add32(x86, x, y));
}

pub fn add_rm32_imm32(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate32();
    x86.rm32_x(instr, |x86, x| add32(x86, x, y));
}

pub fn add_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    x86.rm32_x(instr, |x86, x| add32(x86, x, y));
}

pub fn add_rm16_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8to16() as u16;
    x86.rm16_x(instr, |x86, x| add16(x86, x, y));
}

pub fn add_rm8_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8();
    x86.rm8_x(instr, |x86, x| add8(x86, x, y));
}

pub fn sub_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8to32() as u32;
    x86.rm32_x(instr, |x86, x| sub32(x86, x, y));
}

pub fn sub_rm32_imm32(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate32();
    x86.rm32_x(instr, |x86, x| sub32(x86, x, y));
}

pub fn sub_rm32_r32(x86: &mut X86, instr: &Instruction) {
    let y = x86.regs.get32(instr.op1_register());
    x86.rm32_x(instr, |x86, x| sub32(x86, x, y));
}

pub fn sub_r32_rm32(x86: &mut X86, instr: &Instruction) {
    let reg = instr.op0_register();
    let y = x86.op1_rm32(instr);
    let value = sub32(x86, x86.regs.get32(reg), y);
    x86.regs.set32(reg, value);
}

pub fn sub_rm8_imm8(x86: &mut X86, instr: &Instruction) {
    let y = instr.immediate8();
    x86.rm8_x(instr, |x86, x| sub8(x86, x, y));
}

pub fn sbb_r32_rm32(x86: &mut X86, instr: &Instruction) {
    let reg = instr.op0_register();
    let carry = x86.regs.flags.contains(Flags::CF) as u32;
    let y = x86.op1_rm32(instr).wrapping_add(carry);
    let value = sub32(x86, x86.regs.get32(reg), y);
    x86.regs.set32(reg, value);
}

pub fn sbb_r8_rm8(x86: &mut X86, instr: &Instruction) {
    let reg = instr.op0_register();
    let carry = x86.regs.flags.contains(Flags::CF) as u8;
    let y = x86.op1_rm8(instr).wrapping_add(carry);
    let value = sub8(x86, x86.regs.get8(reg), y);
    x86.regs.set8(reg, value);
}

pub fn imul_r32_rm32(x86: &mut X86, instr: &Instruction) {
    let x = x86.regs.get32(instr.op0_register());
    let y = x86.op1_rm32(instr);
    let value = x.wrapping_mul(y);
    x86.regs.set32(instr.op0_register(), value);
}

pub fn imul_r32_rm32_imm32(x86: &mut X86, instr: &Instruction) {
    let x = x86.op1_rm32(instr) as i32;
    let y = instr.immediate32() as i32;
    let value = x.wrapping_mul(y);
    x86.regs.set32(instr.op0_register(), value as u32);
}

pub fn imul_r32_rm32_imm8(x86: &mut X86, instr: &Instruction) {
    let x = x86.op1_rm32(instr) as i32;
    let y = instr.immediate8to32();
    let value = x.wrapping_mul(y);
    x86.regs.set32(instr.op0_register(), value as u32);
}

pub fn idiv_rm32(x86: &mut X86, instr: &Instruction) {
    let x = (((x86.regs.edx as u64) << 32) | (x86.regs.eax as u64)) as i64;
    let y = x86.op0_rm32(instr) as i32 as i64;
    x86.regs.eax = (x / y) as i32 as u32;
    x86.regs.edx = (x % y) as i32 as u32;
    // TODO: flags.
}

pub fn div_rm32(x86: &mut X86, instr: &Instruction) {
    let x = ((x86.regs.edx as u64) << 32) | (x86.regs.eax as u64);
    let y = x86.op0_rm32(instr) as u64;
    x86.regs.eax = (x / y) as u32;
    x86.regs.edx = (x % y) as u32;
    // TODO: flags.
}

pub fn dec_rm32(x86: &mut X86, instr: &Instruction) {
    x86.rm32_x(instr, |x86, x| sub32(x86, x, 1));
}

pub fn inc_rm32(x86: &mut X86, instr: &Instruction) {
    // TODO: flags.  Note that it's not add32(1) because CF should be preserved.
    x86.rm32_x(instr, |_x86, x| x + 1);
}

pub fn inc_rm8(x86: &mut X86, instr: &Instruction) {
    // TODO: flags.  Note that it's not add8(1) because CF should be preserved.
    x86.rm8_x(instr, |_x86, x| x.wrapping_add(1));
}

pub fn neg_rm32(x86: &mut X86, instr: &Instruction) {
    x86.rm32_x(instr, |x86, x| {
        x86.regs.flags.set(Flags::CF, x != 0);
        // TODO: other flags registers.
        -(x as i32) as u32
    });
}

pub fn not_rm32(x86: &mut X86, instr: &Instruction) {
    x86.rm32_x(instr, |_x86, x| !x);
}
