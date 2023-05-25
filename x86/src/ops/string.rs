use super::math::sub;
use iced_x86::Instruction;

use crate::{
    memory::{Mem, Memory},
    registers::Flags,
    x86::CPU,
};

pub fn cmps(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    assert!(cpu.flags.contains(Flags::DF)); // TODO
    let p1 = cpu.regs.esi;
    let p2 = cpu.regs.edi;
    let count = cpu.regs.ecx;
    if instr.has_repe_prefix() {
        let pos = mem
            .slice(p1..)
            .slice(..count)
            .as_slice_todo()
            .iter()
            .zip(mem.slice(p2..).slice(..count).as_slice_todo().iter())
            .position(|(&x, &y)| x == y)
            .map(|pos| pos as u32)
            .unwrap_or(count);
        cpu.regs.esi += pos;
        cpu.regs.edi += pos;
        cpu.regs.ecx -= pos;
        let x = mem.read_u8(cpu.regs.esi);
        let y = mem.read_u8(cpu.regs.edi);
        sub(x, y, &mut cpu.flags);
    } else {
        cpu.state = Err("unimpl".into());
        return;
    }
}

fn movs(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction, size: u32) {
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
            cpu.state = Err("movs overflow".into());
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

pub fn movsd(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    movs(cpu, mem, instr, 4)
}

pub fn movsw(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    movs(cpu, mem, instr, 2)
}

pub fn movsb(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    movs(cpu, mem, instr, 1)
}

fn scas(cpu: &mut CPU, mem: &Mem, instr: &Instruction, size: u32) {
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
                let src = *mem.view::<u32>(cpu.regs.edi);
                sub(cpu.regs.eax, src, &mut cpu.flags);
            }
            2 => {
                let src = *mem.view::<u16>(cpu.regs.edi);
                sub(cpu.regs.eax as u16, src, &mut cpu.flags);
            }
            1 => {
                let src = *mem.view::<u8>(cpu.regs.edi);
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

pub fn scasd(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    scas(cpu, mem, instr, 4)
}
pub fn scasw(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    scas(cpu, mem, instr, 2)
}
pub fn scasb(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    scas(cpu, mem, instr, 1)
}

pub fn stos(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction, size: u32) {
    assert!(!cpu.flags.contains(Flags::DF)); // TODO

    let mut c = 1u32; // 1 step if no rep prefix
    let counter = if instr.has_rep_prefix() || instr.has_repe_prefix() || instr.has_repne_prefix() {
        &mut cpu.regs.ecx
    } else {
        &mut c
    };

    while *counter > 0 {
        match size {
            1 => *mem.view_mut::<u8>(cpu.regs.edi) = cpu.regs.eax as u8,
            2 => *mem.view_mut::<u16>(cpu.regs.edi) = cpu.regs.eax as u16,
            4 => *mem.view_mut::<u32>(cpu.regs.edi) = cpu.regs.eax,
            _ => unimplemented!(),
        }
        cpu.regs.edi += size;
        *counter -= 1;
    }
    // TODO: does this modify esi?  Sources disagree (!?)
}

pub fn stosd(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    stos(cpu, mem, instr, 4)
}

pub fn stosw(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    stos(cpu, mem, instr, 2)
}

pub fn stosb(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    stos(cpu, mem, instr, 1)
}

pub fn lods(cpu: &mut CPU, mem: &Mem, instr: &Instruction, size: usize) {
    if cpu.flags.contains(Flags::DF) {
        cpu.state = Err("TODO DF".into());
        return;
    }

    assert!(!instr.has_rep_prefix() && !instr.has_repe_prefix() && !instr.has_repne_prefix());
    match size {
        4 => {
            cpu.regs.eax = mem.read_u32(cpu.regs.esi);
        }
        2 => {
            let value = mem.read_u16(cpu.regs.esi);
            cpu.regs.set16(iced_x86::Register::AX, value);
        }
        1 => {
            let value = mem.read_u8(cpu.regs.esi);
            cpu.regs.set8(iced_x86::Register::AL, value);
        }
        _ => unimplemented!("lods size {}", size),
    }
    cpu.regs.esi += size as u32;
}

pub fn lodsd(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    lods(cpu, mem, instr, 4)
}

pub fn lodsw(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    lods(cpu, mem, instr, 2)
}

pub fn lodsb(cpu: &mut CPU, mem: &mut Mem, instr: &Instruction) {
    lods(cpu, mem, instr, 1)
}
