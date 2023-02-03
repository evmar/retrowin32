use iced_x86::Instruction;

use crate::{registers::Flags, x86::X86, Result};

use super::helpers::*;

pub fn call(x86: &mut X86, instr: &Instruction) -> Result<()> {
    let target = instr.near_branch32();
    if target == 0x00408d65 || target == 0x0040a281 {
        log::warn!("HACK: manually nop'd call at {target:x}");
    } else {
        push(x86, x86.regs.eip);
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn call_rm32(x86: &mut X86, instr: &Instruction) -> Result<()> {
    // call dword ptr [addr]
    let target = op0_rm32(x86, instr);
    push(x86, x86.regs.eip);
    x86_jmp(x86, target)?;
    Ok(())
}

pub fn retnd(x86: &mut X86, _instr: &Instruction) -> Result<()> {
    let addr = pop(x86);
    x86_jmp(x86, addr)?;
    Ok(())
}

pub fn retnd_imm16(x86: &mut X86, instr: &Instruction) -> Result<()> {
    let addr = pop(x86);
    x86_jmp(x86, addr)?;
    x86.regs.esp += instr.immediate16() as u32;
    Ok(())
}

pub fn jmp(x86: &mut X86, instr: &Instruction) -> Result<()> {
    x86_jmp(x86, instr.near_branch32())?;
    Ok(())
}

pub fn jmp_rm32(x86: &mut X86, instr: &Instruction) -> Result<()> {
    let target = op0_rm32(x86, instr);
    x86_jmp(x86, target)?;
    Ok(())
}

pub fn ja(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if !x86.regs.flags.contains(Flags::CF) && !x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jae(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if !x86.regs.flags.contains(Flags::CF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jb(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::CF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jbe(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::CF) || x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn je(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jecxz(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.ecx == 0 {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jne(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if !x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jns(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if !x86.regs.flags.contains(Flags::SF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jg(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if !x86.regs.flags.contains(Flags::ZF)
        && (x86.regs.flags.contains(Flags::SF) == x86.regs.flags.contains(Flags::OF))
    {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jge(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::SF) == x86.regs.flags.contains(Flags::OF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jle(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::ZF)
        || (x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF))
    {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jl(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn js(x86: &mut X86, instr: &Instruction) -> Result<()> {
    if x86.regs.flags.contains(Flags::SF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}
