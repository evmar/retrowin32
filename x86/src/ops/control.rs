use iced_x86::Instruction;

use crate::{registers::Flags, x86::X86, StepResult};

use super::helpers::*;

pub fn call(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    push(x86, x86.regs.eip);
    x86_jmp(x86, instr.near_branch32())
}

pub fn call_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    // call dword ptr [addr]
    let target = op0_rm32(x86, instr);
    push(x86, x86.regs.eip);
    x86_jmp(x86, target)
}

pub fn retnd(x86: &mut X86, _instr: &Instruction) -> StepResult<()> {
    let addr = pop(x86);
    x86_jmp(x86, addr)
}

pub fn retnd_imm16(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let addr = pop(x86);
    x86_jmp(x86, addr)?;
    x86.regs.esp += instr.immediate16() as u32;
    Ok(())
}

pub fn jmp(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86_jmp(x86, instr.near_branch32())
}

pub fn jmp_rm32(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let target = op0_rm32(x86, instr);
    x86_jmp(x86, target)
}

pub fn ja(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if !x86.regs.flags.contains(Flags::CF) && !x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jae(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if !x86.regs.flags.contains(Flags::CF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jb(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::CF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jbe(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::CF) || x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn je(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jecxz(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.ecx == 0 {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jne(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if !x86.regs.flags.contains(Flags::ZF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jns(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if !x86.regs.flags.contains(Flags::SF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jg(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if !x86.regs.flags.contains(Flags::ZF)
        && (x86.regs.flags.contains(Flags::SF) == x86.regs.flags.contains(Flags::OF))
    {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jge(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::SF) == x86.regs.flags.contains(Flags::OF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jle(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::ZF)
        || (x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF))
    {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn jl(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn js(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    if x86.regs.flags.contains(Flags::SF) {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}

pub fn loop_(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    x86.regs.ecx -= 1;
    if x86.regs.ecx != 0 {
        x86_jmp(x86, instr.near_branch32())?;
    }
    Ok(())
}
