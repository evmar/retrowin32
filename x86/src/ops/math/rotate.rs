use super::int::Int;
use crate::{ops::helpers::*, registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::Mem;

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
