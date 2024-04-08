//! Ops that tend to loop with 'rep' prefix, e.g. movs, stos.

use super::math::sub;
use crate::{registers::Flags, x86::CPU};
use iced_x86::Instruction;
use memory::{Extensions, Mem};

/// Width of an operation, e.g. movsb/w/d.
#[derive(Clone, Copy)]
enum Size {
    Byte = 1,
    Word = 2,
    Dword = 4,
}

enum Rep {
    REP,
    REPNE,
    REPE,
}

impl Rep {
    fn from_instr(instr: &Instruction) -> Option<Rep> {
        Some(if instr.has_rep_prefix() {
            Rep::REP
        } else if instr.has_repe_prefix() {
            Rep::REPE
        } else if instr.has_repne_prefix() {
            Rep::REPNE
        } else {
            return None;
        })
    }
}

/// Looping logic of various 'rep' prefixes, generalized for different instructions.
/// Note: some instructions do not have varying Reps and it is important to treat them
/// as plain REP; e.g. "REPNE MOVS" just means "REP MOVS".
fn rep(cpu: &mut CPU, mem: Mem, rep: Rep, size: Size, func: impl Fn(&mut CPU, Mem, Size)) {
    while cpu.regs.ecx > 0 {
        func(cpu, mem, size);
        cpu.regs.ecx -= 1;
        match rep {
            Rep::REPE if !cpu.flags.contains(Flags::ZF) => break,
            Rep::REPNE if cpu.flags.contains(Flags::ZF) => break,
            _ => {}
        }
    }
}

fn cmps_single(cpu: &mut CPU, mem: Mem, size: Size) {
    match size {
        Size::Dword => {
            let x = mem.get_pod::<u32>(cpu.regs.esi);
            let y = mem.get_pod::<u32>(cpu.regs.edi);
            sub(x, y, &mut cpu.flags);
        }
        Size::Word => {
            let x = mem.get_pod::<u16>(cpu.regs.esi);
            let y = mem.get_pod::<u16>(cpu.regs.edi);
            sub(x, y, &mut cpu.flags);
        }
        Size::Byte => {
            let x = mem.get_pod::<u8>(cpu.regs.esi);
            let y = mem.get_pod::<u8>(cpu.regs.edi);
            sub(x, y, &mut cpu.flags);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        cpu.regs.edi -= size as u32;
        cpu.regs.esi -= size as u32;
    } else {
        cpu.regs.edi += size as u32;
        cpu.regs.esi += size as u32;
    };
}

fn cmps(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(r) = Rep::from_instr(instr) {
        rep(cpu, mem, r, size, cmps_single);
    } else {
        cmps_single(cpu, mem, size);
    }
}

pub fn cmpsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cmps(cpu, mem, instr, Size::Dword);
}

pub fn cmpsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cmps(cpu, mem, instr, Size::Word);
}

pub fn cmpsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cmps(cpu, mem, instr, Size::Byte);
}

fn movs_single(cpu: &mut CPU, mem: Mem, size: Size) {
    match size {
        Size::Dword => {
            let src = mem.get_pod::<u32>(cpu.regs.esi);
            mem.put::<u32>(cpu.regs.edi, src);
        }
        Size::Word => {
            let src = mem.get_pod::<u16>(cpu.regs.esi);
            mem.put::<u16>(cpu.regs.edi, src);
        }
        Size::Byte => {
            let src = mem.get_pod::<u8>(cpu.regs.esi);
            mem.put::<u8>(cpu.regs.edi, src);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        cpu.regs.edi -= size as u32;
        cpu.regs.esi -= size as u32;
    } else {
        cpu.regs.edi += size as u32;
        cpu.regs.esi += size as u32;
    };
}

fn movs(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(_) = Rep::from_instr(instr) {
        rep(cpu, mem, Rep::REP, size, movs_single);
    } else {
        movs_single(cpu, mem, size);
    }
}

pub fn movsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, Size::Dword)
}

pub fn movsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, Size::Word)
}

pub fn movsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, Size::Byte)
}

fn scas_single(cpu: &mut CPU, mem: Mem, size: Size) {
    match size {
        Size::Dword => {
            let src = mem.get_pod::<u32>(cpu.regs.edi);
            sub(cpu.regs.eax, src, &mut cpu.flags);
        }
        Size::Word => {
            let src = mem.get_pod::<u16>(cpu.regs.edi);
            sub(cpu.regs.eax as u16, src, &mut cpu.flags);
        }
        Size::Byte => {
            let src = mem.get_pod::<u8>(cpu.regs.edi);
            sub(cpu.regs.eax as u8, src, &mut cpu.flags);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        cpu.regs.edi -= size as u32;
    } else {
        cpu.regs.edi += size as u32;
    };
}

fn scas(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(r) = Rep::from_instr(instr) {
        rep(cpu, mem, r, size, scas_single);
    } else {
        scas_single(cpu, mem, size);
    }
}

pub fn scasd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, Size::Dword)
}

pub fn scasw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, Size::Word)
}

pub fn scasb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, Size::Byte)
}

fn stos_single(cpu: &mut CPU, mem: Mem, size: Size) {
    match size {
        Size::Byte => mem.put::<u8>(cpu.regs.edi, cpu.regs.eax as u8),
        Size::Word => mem.put::<u16>(cpu.regs.edi, cpu.regs.eax as u16),
        Size::Dword => mem.put::<u32>(cpu.regs.edi, cpu.regs.eax),
    }
    if cpu.flags.contains(Flags::DF) {
        cpu.regs.edi -= size as u32;
    } else {
        cpu.regs.edi += size as u32;
    };
}

fn stos(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(_) = Rep::from_instr(instr) {
        rep(cpu, mem, Rep::REP, size, stos_single);
    } else {
        stos_single(cpu, mem, size);
    }
}

pub fn stosd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Size::Dword)
}

pub fn stosw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Size::Word)
}

pub fn stosb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Size::Byte)
}

fn lods_single(cpu: &mut CPU, mem: Mem, size: Size) {
    match size {
        Size::Byte => {
            let value = mem.get_pod::<u8>(cpu.regs.esi);
            cpu.regs.set8(iced_x86::Register::AL, value)
        }
        Size::Word => {
            let value = mem.get_pod::<u16>(cpu.regs.esi);
            cpu.regs.set16(iced_x86::Register::AL, value)
        }
        Size::Dword => {
            cpu.regs.eax = mem.get_pod::<u32>(cpu.regs.esi);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        cpu.regs.esi -= size as u32;
    } else {
        cpu.regs.esi += size as u32;
    };
}

fn lods(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(_) = Rep::from_instr(instr) {
        rep(cpu, mem, Rep::REP, size, lods_single);
    } else {
        lods_single(cpu, mem, size);
    }
}

pub fn lodsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, Size::Dword)
}

pub fn lodsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, Size::Word)
}

pub fn lodsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, Size::Byte)
}
