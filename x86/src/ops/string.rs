//! Ops that tend to loop with 'rep' prefix, e.g. movs, stos.

use super::math::sub;
use crate::{registers::Flags, x86::CPU};
use iced_x86::Instruction;
use memory::{Extensions, Mem};

/// Width of an operation, e.g. movsb/movsw/movsd.
#[derive(Clone, Copy)]
enum Width {
    Byte = 1,
    Word = 2,
    Dword = 4,
}

/// Looping logic of various 'rep' prefixes, generalized for different instructions.
/// TODO: move all the implementations in this module to use this function.
fn rep(
    cpu: &mut CPU,
    mem: Mem,
    instr: &Instruction,
    size: Width,
    func: impl Fn(&mut CPU, Mem, Width),
) {
    while cpu.regs.ecx > 0 {
        func(cpu, mem, size);
        cpu.regs.ecx -= 1;
        if instr.has_repne_prefix() && cpu.flags.contains(Flags::ZF) {
            break;
        } else if instr.has_repe_prefix() && !cpu.flags.contains(Flags::ZF) {
            break;
        }
    }
}

pub fn cmps(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    assert!(cpu.flags.contains(Flags::DF)); // TODO
    let p1 = cpu.regs.esi;
    let p2 = cpu.regs.edi;
    let count = cpu.regs.ecx;
    if instr.has_repe_prefix() {
        let pos = mem
            .sub(p1, count)
            .as_slice_todo()
            .iter()
            .zip(mem.sub(p2, count).as_slice_todo().iter())
            .position(|(&x, &y)| x == y)
            .map(|pos| pos as u32)
            .unwrap_or(count);
        cpu.regs.esi += pos;
        cpu.regs.edi += pos;
        cpu.regs.ecx -= pos;
        let x = mem.get_pod::<u8>(cpu.regs.esi);
        let y = mem.get_pod::<u8>(cpu.regs.edi);
        sub(x, y, &mut cpu.flags);
    } else {
        cpu.err("unimpl".into());
        return;
    }
}

fn movs(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: u32) {
    let reverse = cpu.flags.contains(Flags::DF);
    let step = if reverse { -(size as i32) as u32 } else { size };
    let mut c = 1u32; // 1 step if no rep prefix
    let counter = if instr.has_rep_prefix() || instr.has_repe_prefix() || instr.has_repne_prefix() {
        &mut cpu.regs.ecx
    } else {
        &mut c
    };
    while *counter > 0 {
        *counter -= 1;

        if cpu.regs.edi >= mem.len() - 8 {
            cpu.err("movs overflow".into());
            return;
        }

        mem.as_mut_slice_todo().copy_within(
            cpu.regs.esi as usize..(cpu.regs.esi + size) as usize,
            cpu.regs.edi as usize,
        );
        cpu.regs.esi = cpu.regs.esi.wrapping_add(step);
        cpu.regs.edi = cpu.regs.edi.wrapping_add(step);
        if instr.has_repe_prefix() || instr.has_repne_prefix() {
            // https://stackoverflow.com/questions/40219519/why-do-repe-and-repne-do-the-same-before-movsb
            // return Err(StepError::Error("movs: unimplemented prefix".into()));
        }
    }
}

pub fn movsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, 4)
}

pub fn movsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, 2)
}

pub fn movsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, 1)
}

fn scas(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: u32) {
    assert!(!cpu.flags.contains(Flags::DF)); // TODO

    let mut c = 1u32; // 1 step if no rep prefix
    let counter = if instr.has_rep_prefix() || instr.has_repe_prefix() || instr.has_repne_prefix() {
        &mut cpu.regs.ecx
    } else {
        &mut c
    };

    while *counter > 0 {
        match size {
            4 => {
                let src = mem.get_pod::<u32>(cpu.regs.edi);
                sub(cpu.regs.eax, src, &mut cpu.flags);
            }
            2 => {
                let src = mem.get_pod::<u16>(cpu.regs.edi);
                sub(cpu.regs.eax as u16, src, &mut cpu.flags);
            }
            1 => {
                let src = mem.get_pod::<u8>(cpu.regs.edi);
                sub(cpu.regs.eax as u8, src, &mut cpu.flags);
            }
            _ => unimplemented!(),
        }
        cpu.regs.edi = cpu.regs.edi.wrapping_add(size);
        *counter = counter.wrapping_sub(1);
        if instr.has_repne_prefix() && cpu.flags.contains(Flags::ZF) {
            break;
        } else if instr.has_repe_prefix() && !cpu.flags.contains(Flags::ZF) {
            break;
        }
    }
}

pub fn scasd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, 4)
}
pub fn scasw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, 2)
}
pub fn scasb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, 1)
}

fn stos_single(cpu: &mut CPU, mem: Mem, size: Width) {
    match size {
        Width::Byte => mem.put::<u8>(cpu.regs.edi, cpu.regs.eax as u8),
        Width::Word => mem.put::<u16>(cpu.regs.edi, cpu.regs.eax as u16),
        Width::Dword => mem.put::<u32>(cpu.regs.edi, cpu.regs.eax),
    }
    if cpu.flags.contains(Flags::DF) {
        cpu.regs.edi -= size as u32;
    } else {
        cpu.regs.edi += size as u32;
    };
}

fn stos(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Width) {
    if instr.has_rep_prefix() {
        rep(cpu, mem, instr, size, stos_single);
    } else {
        stos_single(cpu, mem, size);
    }
}

pub fn stosd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Width::Dword)
}

pub fn stosw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Width::Word)
}

pub fn stosb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Width::Byte)
}

pub fn lods(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: usize) {
    if cpu.flags.contains(Flags::DF) {
        cpu.err("TODO DF".into());
        return;
    }

    assert!(!instr.has_rep_prefix() && !instr.has_repe_prefix() && !instr.has_repne_prefix());
    match size {
        4 => {
            cpu.regs.eax = mem.get_pod::<u32>(cpu.regs.esi);
        }
        2 => {
            let value = mem.get_pod::<u16>(cpu.regs.esi);
            cpu.regs.set16(iced_x86::Register::AX, value);
        }
        1 => {
            let value = mem.get_pod::<u8>(cpu.regs.esi);
            cpu.regs.set8(iced_x86::Register::AL, value);
        }
        _ => unimplemented!("lods size {}", size),
    }
    cpu.regs.esi += size as u32;
}

pub fn lodsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, 4)
}

pub fn lodsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, 2)
}

pub fn lodsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, 1)
}
