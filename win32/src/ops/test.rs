use iced_x86::Instruction;

use crate::{registers::Flags, x86::X86};

use super::math::{and16, and32, and8, sub16, sub32, sub8};

pub fn cmp_rm32_r32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm32(instr);
    let y = x86.regs.get32(instr.op1_register());
    sub32(x86, x, y);
    Ok(())
}

pub fn cmp_r32_rm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.regs.get32(instr.op0_register());
    let y = x86.op1_rm32(instr);
    sub32(x86, x, y);
    Ok(())
}

pub fn cmp_rm32_imm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm32(instr);
    let y = instr.immediate32();
    sub32(x86, x, y);
    Ok(())
}

pub fn cmp_rm32_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm32(instr);
    let y = instr.immediate8to32() as u32;
    sub32(x86, x, y);
    Ok(())
}

pub fn cmp_rm16_rm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm16(instr);
    let y = x86.op1_rm16(instr);
    sub16(x86, x, y);
    Ok(())
}

pub fn cmp_rm16_imm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm16(instr);
    let y = instr.immediate16();
    sub16(x86, x, y);
    Ok(())
}

pub fn cmp_rm16_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm16(instr);
    let y = instr.immediate8to16() as u16;
    sub16(x86, x, y);
    Ok(())
}

pub fn cmp_rm8_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm8(instr);
    let y = instr.immediate8();
    sub8(x86, x, y);
    Ok(())
}

pub fn cmp_rm8_r8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm8(instr);
    let y = x86.regs.get8(instr.op1_register());
    sub8(x86, x, y);
    Ok(())
}

pub fn cmp_r8_rm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.regs.get8(instr.op0_register());
    let y = match instr.op1_kind() {
        iced_x86::OpKind::Register => x86.regs.get8(instr.op1_register()),
        iced_x86::OpKind::Memory => x86.read_u8(x86.addr(instr)),
        _ => unreachable!(),
    };
    sub8(x86, x, y);
    Ok(())
}

pub fn test_rm32_r32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm32(instr);
    let y = x86.regs.get32(instr.op1_register());
    and32(x86, x, y);
    Ok(())
}

pub fn test_rm32_imm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm32(instr);
    let y = instr.immediate32();
    and32(x86, x, y);
    Ok(())
}

pub fn test_rm16_r16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm16(instr);
    let y = x86.regs.get16(instr.op1_register());
    and16(x86, x, y);
    Ok(())
}

pub fn test_rm8_r8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm8(instr);
    let y = x86.regs.get8(instr.op1_register());
    and8(x86, x, y);
    Ok(())
}

pub fn test_rm8_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm8(instr);
    let y = instr.immediate8();
    and8(x86, x, y);
    Ok(())
}

pub fn bt_rm32_imm8(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let x = x86.op0_rm32(instr);
    let y = instr.immediate8() % 32;
    x86.regs.flags.set(Flags::CF, ((x >> y) & 1) != 0);
    Ok(())
}
