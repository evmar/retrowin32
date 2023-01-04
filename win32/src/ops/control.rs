use iced_x86::Instruction;

use crate::x86::{Flags, X86};

pub fn call_rel32_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let target = instr.near_branch32();
    if target == 0x00408d65 || target == 0x0040a281 {
        log::warn!("HACK: manually nop'd call at {target:x}");
    } else {
        x86.push(x86.regs.eip);
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn call_rm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    // call dword ptr [addr]
    let target = match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u32(x86.addr(instr)),
        _ => unreachable!(),
    };
    x86.push(x86.regs.eip);
    x86.jmp(target)?;
    Ok(())
}

pub fn retnd(x86: &mut X86, _instr: &Instruction) -> anyhow::Result<()> {
    let addr = x86.pop();
    x86.jmp(addr)?;
    Ok(())
}

pub fn retnd_imm16(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let addr = x86.pop();
    x86.jmp(addr)?;
    x86.regs.esp += instr.immediate16() as u32;
    Ok(())
}

pub fn jmp_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.jmp(instr.near_branch32())?;
    Ok(())
}

pub fn jmp_rel32_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    x86.jmp(instr.near_branch32())?;
    Ok(())
}

pub fn jmp_rm32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let target = match instr.op0_kind() {
        iced_x86::OpKind::Register => x86.regs.get32(instr.op0_register()),
        iced_x86::OpKind::Memory => x86.read_u32(x86.addr(instr)),
        _ => unreachable!(),
    };
    x86.jmp(target)?;
    Ok(())
}

pub fn ja_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if !x86.regs.flags.contains(Flags::CF) && !x86.regs.flags.contains(Flags::ZF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jae_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if !x86.regs.flags.contains(Flags::CF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jb_rel32_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::CF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jbe_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::CF) || x86.regs.flags.contains(Flags::ZF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn je_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::ZF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn je_rel32_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::ZF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jecxz_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.ecx == 0 {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jne_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if !x86.regs.flags.contains(Flags::ZF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jns_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if !x86.regs.flags.contains(Flags::SF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jg_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if !x86.regs.flags.contains(Flags::ZF)
        && (x86.regs.flags.contains(Flags::SF) == x86.regs.flags.contains(Flags::OF))
    {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jge_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::SF) == x86.regs.flags.contains(Flags::OF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jl_rel32_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jle_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::ZF)
        || (x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF))
    {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}

pub fn jl_rel8_32(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    if x86.regs.flags.contains(Flags::SF) != x86.regs.flags.contains(Flags::OF) {
        x86.jmp(instr.near_branch32())?;
    }
    Ok(())
}
