use super::math::sub;
use iced_x86::Instruction;

use crate::{memory::Memory, registers::Flags, x86::X86, StepError, StepResult};

pub fn cmps(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    assert!(x86.flags.contains(Flags::DF)); // TODO
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
        sub(x, y, &mut x86.flags);
    } else {
        return Err(StepError::Error("unimpl".into()));
    }
    Ok(())
}

fn movs(x86: &mut X86, instr: &Instruction, size: u32) -> StepResult<()> {
    let reverse = x86.flags.contains(Flags::DF);
    let step = if reverse { -(size as i32) as u32 } else { size };
    let mut c = 1u32; // 1 step if no rep prefix
    let counter = if instr.has_rep_prefix() || instr.has_repe_prefix() || instr.has_repne_prefix() {
        &mut x86.regs.ecx
    } else {
        &mut c
    };
    while *counter > 0 {
        *counter -= 1;

        if x86.regs.edi as usize >= x86.mem.len() - 8 {
            return Err(StepError::Error("movs overflow".into()));
        }

        x86.mem.copy_within(
            x86.regs.esi as usize..(x86.regs.esi + size) as usize,
            x86.regs.edi as usize,
        );
        x86.regs.esi = x86.regs.esi.wrapping_add(step);
        x86.regs.edi = x86.regs.edi.wrapping_add(step);
        if instr.has_repe_prefix() || instr.has_repne_prefix() {
            // https://stackoverflow.com/questions/40219519/why-do-repe-and-repne-do-the-same-before-movsb
            // return Err(StepError::Error("movs: unimplemented prefix".into()));
        }
    }
    Ok(())
}

pub fn movsd(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    movs(x86, instr, 4)
}

pub fn movsw(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    movs(x86, instr, 2)
}

pub fn movsb(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    movs(x86, instr, 1)
}

fn scas(x86: &mut X86, instr: &Instruction, size: u32) -> StepResult<()> {
    assert!(x86.flags.contains(Flags::DF)); // TODO

    let mut c = 1u32; // 1 step if no rep prefix
    let counter = if instr.has_rep_prefix() || instr.has_repe_prefix() || instr.has_repne_prefix() {
        &mut x86.regs.ecx
    } else {
        &mut c
    };

    while *counter > 0 {
        match size {
            1 => {
                let src = x86.mem[x86.regs.edi as usize];
                sub(x86.regs.eax as u8, src, &mut x86.flags);
            }
            _ => unimplemented!(),
        }
        x86.regs.edi = x86.regs.edi.wrapping_add(size);
        *counter = counter.wrapping_sub(1);
        if instr.has_repne_prefix() && x86.flags.contains(Flags::ZF) {
            break;
        } else if instr.has_repe_prefix() && !x86.flags.contains(Flags::ZF) {
            break;
        }
    }

    Ok(())
}

pub fn scasb(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    scas(x86, instr, 1)
}

pub fn stosd(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    let mut dst = x86.regs.edi as usize;
    let value = x86.regs.eax;

    if instr.has_rep_prefix() {
        let count = x86.regs.ecx as usize;
        let reverse = x86.flags.contains(Flags::DF);
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
        return Err(StepError::Error("unimpl".into()));
    } else {
        *x86.mem.view_mut::<u32>(dst as u32) = value;
        x86.regs.edi += 4;
    }
    // TODO: does this modify esi?  Sources disagree (!?)
    Ok(())
}

pub fn stosb(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    assert!(!x86.flags.contains(Flags::DF)); // TODO

    let dst = x86.regs.edi as usize;
    let value = x86.regs.eax as u8;
    if instr.has_rep_prefix() {
        let count = x86.regs.ecx as usize;
        x86.mem[dst..dst + count].fill(value);
        x86.regs.edi += count as u32;
        x86.regs.ecx = 0;
    } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
        return Err(StepError::Error("unimpl".into()));
    } else {
        x86.mem[dst] = value;
        x86.regs.edi += 1;
    }
    Ok(())
}

pub fn lods(x86: &mut X86, instr: &Instruction, size: usize) -> StepResult<()> {
    if x86.flags.contains(Flags::DF) {
        return Err(StepError::Error("TODO DF".into()));
    }

    assert!(!instr.has_rep_prefix() && !instr.has_repe_prefix() && !instr.has_repne_prefix());
    match size {
        4 => {
            x86.regs.eax = x86.read_u32(x86.regs.esi);
        }
        2 => {
            let value = x86.read_u16(x86.regs.esi);
            x86.regs.set16(iced_x86::Register::AX, value);
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

pub fn lodsd(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    lods(x86, instr, 4)
}

pub fn lodsw(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    lods(x86, instr, 2)
}

pub fn lodsb(x86: &mut X86, instr: &Instruction) -> StepResult<()> {
    lods(x86, instr, 1)
}
