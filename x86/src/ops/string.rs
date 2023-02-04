use super::math::sub8;
use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::X86, Error, Result};

pub fn cmps(x86: &mut X86, instr: &Instruction) -> Result<()> {
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
        let x = x86.read_u8(x86.regs.esi);
        let y = x86.read_u8(x86.regs.edi);
        sub8(x86, x, y);
    } else {
        return Err(Error::Error("unimpl".into()));
    }
    Ok(())
}

fn movs(x86: &mut X86, instr: &Instruction, size: usize) -> Result<()> {
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
        return Err(Error::Error("movs: unimplemented prefix".into()));
    } else {
        movs_single();
    };
    x86.regs.edi = dst as u32;
    x86.regs.esi = src as u32;
    Ok(())
}

pub fn movsd(x86: &mut X86, instr: &Instruction) -> Result<()> {
    movs(x86, instr, 4)
}

pub fn movsb(x86: &mut X86, instr: &Instruction) -> Result<()> {
    movs(x86, instr, 1)
}

pub fn scas(x86: &mut X86, instr: &Instruction) -> Result<()> {
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
        return Err(Error::Error("unimpl".into()));
    }
    Ok(())
}

pub fn stosd(x86: &mut X86, instr: &Instruction) -> Result<()> {
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
        return Err(Error::Error("unimpl".into()));
    } else {
        *x86.mem.view_mut::<u32>(dst as u32) = value;
        x86.regs.edi += 4;
    }
    // TODO: does this modify esi?  Sources disagree (!?)
    Ok(())
}

pub fn stosb(x86: &mut X86, instr: &Instruction) -> Result<()> {
    assert!(!x86.regs.flags.contains(Flags::DF)); // TODO

    let dst = x86.regs.edi as usize;
    let value = x86.regs.eax as u8;
    if instr.has_rep_prefix() {
        let count = x86.regs.ecx as usize;
        x86.mem[dst..dst + count].fill(value);
        x86.regs.edi += count as u32;
        x86.regs.ecx = 0;
    } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
        return Err(Error::Error("unimpl".into()));
    } else {
        x86.mem[dst] = value;
        x86.regs.edi += 1;
    }
    Ok(())
}

pub fn lods(x86: &mut X86, instr: &Instruction, size: usize) -> Result<()> {
    if x86.regs.flags.contains(Flags::DF) {
        return Err(Error::Error("TODO DF".into()));
    }

    assert!(!instr.has_rep_prefix() && !instr.has_repe_prefix() && !instr.has_repne_prefix());
    match size {
        4 => {
            x86.regs.eax = x86.read_u32(x86.regs.esi);
        }
        1 => {
            let value = x86.read_u8(x86.regs.esi);
            x86.regs.set8(iced_x86::Register::AL, value);
        }
        _ => unimplemented!("lods size {}", size),
    }
    x86.regs.esi += size as u32;
    Ok(())
}

pub fn lodsd(x86: &mut X86, instr: &Instruction) -> Result<()> {
    lods(x86, instr, 4)
}

pub fn lodsb(x86: &mut X86, instr: &Instruction) -> Result<()> {
    lods(x86, instr, 1)
}
