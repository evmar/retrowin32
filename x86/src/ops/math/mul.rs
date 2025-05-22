use super::int::Int;
use crate::{ops::helpers::*, registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::Mem;
use num_traits::{Signed, ops::overflowing::OverflowingMul};

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
