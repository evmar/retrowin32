use super::math::sub8;
use anyhow::bail;
use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::X86};

pub fn cmps(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    assert!(x86.regs.flags.contains(Flags::DF)); // TODO
    let p1 = x86.regs.esi as usize;
    let p2 = x86.regs.edi as usize;
    let count = x86.regs.ecx as usize;
    if instr.has_repe_prefix() {
        let pos = x86.mem[p1..p1 + count]
            .iter()
            .zip(x86.mem[p2..p2 + count].iter())
            .position(|(&x, &y)| x == y)
            .unwrap_or(count);
        x86.regs.esi += pos as u32;
        x86.regs.edi += pos as u32;
        x86.regs.ecx -= pos as u32;
        sub8(x86, x86.read_u8(x86.regs.esi), x86.read_u8(x86.regs.edi));
    } else {
        bail!("unimpl");
    }
    Ok(())
}

pub fn movs(x86: &mut X86, instr: &Instruction, size: usize) -> anyhow::Result<()> {
    let reverse = x86.regs.flags.contains(Flags::DF);
    let step = if reverse {
        -(size as isize) as usize
    } else {
        size
    };
    let mut dst = x86.regs.edi as usize;
    let mut src = x86.regs.esi as usize;

    let mut movs_single = || {
        x86.mem.copy_within(src..src + size, dst);
        src = src.wrapping_add(step);
        dst = dst.wrapping_add(step);
    };
    if instr.has_rep_prefix() {
        let count = x86.regs.ecx;
        for _ in 0..count {
            movs_single();
        }
        x86.regs.ecx = 0;
    } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
        bail!("movs: unimplemented prefix");
    } else {
        movs_single();
    };
    x86.regs.edi = dst as u32;
    x86.regs.esi = src as u32;
    Ok(())
}

pub fn scas(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    assert!(x86.regs.flags.contains(Flags::DF)); // TODO
    let src = x86.regs.edi as usize;
    let value = x86.regs.eax as u8;
    let count = x86.regs.ecx as usize;
    if instr.has_repne_prefix() {
        let pos = x86.mem[src..src + count]
            .iter()
            .position(|&c| c == value)
            .unwrap_or(count);
        x86.regs.edi += pos as u32;
        x86.regs.ecx -= pos as u32;
        sub8(
            x86,
            x86.regs.get8(iced_x86::Register::AL),
            x86.regs.get8(iced_x86::Register::DL),
        );
    } else {
        bail!("unimpl");
    }
    Ok(())
}

pub fn stosd(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    let mut dst = x86.regs.edi as usize;
    let value = x86.regs.eax;

    if instr.has_rep_prefix() {
        let count = x86.regs.ecx as usize;
        let reverse = x86.regs.flags.contains(Flags::DF);
        if reverse && count > 1 {
            dst -= (count - 1) * 4;
        }
        let mem: &mut [u32] = unsafe {
            let mem = &mut x86.mem[dst..];
            std::slice::from_raw_parts_mut(mem.as_ptr() as *mut u32, count)
        };
        mem.fill(value);
        if reverse {
            x86.regs.edi -= count as u32 * 4;
        } else {
            x86.regs.edi += count as u32 * 4;
        }
        x86.regs.ecx = 0;
    } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
        bail!("unimpl");
    } else {
        *x86.mem.view_mut::<u32>(dst as u32) = value;
        x86.regs.edi += 4;
    }
    // TODO: does this modify esi?  Sources disagree (!?)
    Ok(())
}

pub fn stosb(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    assert!(!x86.regs.flags.contains(Flags::DF)); // TODO

    let dst = x86.regs.edi as usize;
    let value = x86.regs.eax as u8;
    if instr.has_rep_prefix() {
        let count = x86.regs.ecx as usize;
        x86.mem[dst..dst + count].fill(value);
        x86.regs.edi += count as u32;
        x86.regs.ecx = 0;
    } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
        bail!("unimpl");
    } else {
        x86.mem[dst] = value;
        x86.regs.edi += 1;
    }
    Ok(())
}

pub fn lods(x86: &mut X86, instr: &Instruction) -> anyhow::Result<()> {
    assert!(x86.regs.flags.contains(Flags::DF)); // TODO

    assert!(!instr.has_rep_prefix() && !instr.has_repe_prefix() && !instr.has_repne_prefix());
    x86.regs.eax = x86.read_u32(x86.regs.esi);
    x86.regs.esi += 4;
    Ok(())
}
