//! Ops that tend to loop with 'rep' prefix, e.g. movs, stos.

use super::math::sub;
use crate::{registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::{Extensions, Mem};

/// Width of an operation, e.g. movsb/w/d.
#[derive(Clone, Copy)]
enum Size {
    Byte = 1,
    Word = 2,
    Dword = 4,
}

#[derive(Debug)]
enum Rep {
    REP,
    REPNE,
    REPE,
}

impl Rep {
    // Note: instr.has_rep_prefix() and instr.has_repe_prefix() are identical(!)
    // because there's a single shared prefix value in the x86 encoding, and whether
    // it's rep or repe depends on the instruction.
    //
    // Either: it's lods/stos etc. and both prefixes map to rep (use ::is_rep),
    // or: it's cmps/scas and the prefixes are repe/repne (use ::from_instr).

    /// Only use this for instructions which always behave as 'rep'.
    fn is_rep(instr: &Instruction) -> bool {
        instr.has_rep_prefix() || instr.has_repne_prefix()
    }

    /// Only use this for cmps/scas.
    fn from_instr(instr: &Instruction) -> Option<Rep> {
        Some(if instr.has_repe_prefix() {
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
    while cpu.regs.get32(Register::ECX) > 0 {
        func(cpu, mem, size);
        *cpu.regs.get32_mut(Register::ECX) -= 1;
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
            let x = mem.get_pod::<u32>(cpu.regs.get32(Register::ESI));
            let y = mem.get_pod::<u32>(cpu.regs.get32(Register::EDI));
            sub(x, y, &mut cpu.flags);
        }
        Size::Word => {
            let x = mem.get_pod::<u16>(cpu.regs.get32(Register::ESI));
            let y = mem.get_pod::<u16>(cpu.regs.get32(Register::EDI));
            sub(x, y, &mut cpu.flags);
        }
        Size::Byte => {
            let x = mem.get_pod::<u8>(cpu.regs.get32(Register::ESI));
            let y = mem.get_pod::<u8>(cpu.regs.get32(Register::EDI));
            sub(x, y, &mut cpu.flags);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        *cpu.regs.get32_mut(Register::EDI) -= size as u32;
        *cpu.regs.get32_mut(Register::ESI) -= size as u32;
    } else {
        *cpu.regs.get32_mut(Register::EDI) += size as u32;
        *cpu.regs.get32_mut(Register::ESI) += size as u32;
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
            let src = mem.get_pod::<u32>(cpu.regs.get32(Register::ESI));
            mem.put_pod::<u32>(cpu.regs.get32(Register::EDI), src);
        }
        Size::Word => {
            let src = mem.get_pod::<u16>(cpu.regs.get32(Register::ESI));
            mem.put_pod::<u16>(cpu.regs.get32(Register::EDI), src);
        }
        Size::Byte => {
            let src = mem.get_pod::<u8>(cpu.regs.get32(Register::ESI));
            mem.put_pod::<u8>(cpu.regs.get32(Register::EDI), src);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        *cpu.regs.get32_mut(Register::EDI) -= size as u32;
        *cpu.regs.get32_mut(Register::ESI) -= size as u32;
    } else {
        *cpu.regs.get32_mut(Register::EDI) += size as u32;
        *cpu.regs.get32_mut(Register::ESI) += size as u32;
    };
}

fn movs(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if Rep::is_rep(instr) {
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
            let src = mem.get_pod::<u32>(cpu.regs.get32(Register::EDI));
            sub(cpu.regs.get32(Register::EAX), src, &mut cpu.flags);
        }
        Size::Word => {
            let src = mem.get_pod::<u16>(cpu.regs.get32(Register::EDI));
            sub(cpu.regs.get32(Register::EAX) as u16, src, &mut cpu.flags);
        }
        Size::Byte => {
            let src = mem.get_pod::<u8>(cpu.regs.get32(Register::EDI));
            sub(cpu.regs.get32(Register::EAX) as u8, src, &mut cpu.flags);
        }
    }
    if cpu.flags.contains(Flags::DF) {
        *cpu.regs.get32_mut(Register::EDI) -= size as u32;
    } else {
        *cpu.regs.get32_mut(Register::EDI) += size as u32;
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
        Size::Byte => mem.put_pod::<u8>(
            cpu.regs.get32(Register::EDI),
            cpu.regs.get32(Register::EAX) as u8,
        ),
        Size::Word => mem.put_pod::<u16>(
            cpu.regs.get32(Register::EDI),
            cpu.regs.get32(Register::EAX) as u16,
        ),
        Size::Dword => {
            mem.put_pod::<u32>(cpu.regs.get32(Register::EDI), cpu.regs.get32(Register::EAX))
        }
    }
    if cpu.flags.contains(Flags::DF) {
        *cpu.regs.get32_mut(Register::EDI) -= size as u32;
    } else {
        *cpu.regs.get32_mut(Register::EDI) += size as u32;
    };
}

fn stos(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if Rep::is_rep(instr) {
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
            let value = mem.get_pod::<u8>(cpu.regs.get32(Register::ESI));
            cpu.regs.set8(iced_x86::Register::AL, value)
        }
        Size::Word => {
            let value = mem.get_pod::<u16>(cpu.regs.get32(Register::ESI));
            cpu.regs.set16(iced_x86::Register::AX, value)
        }
        Size::Dword => {
            cpu.regs.set32(
                Register::EAX,
                mem.get_pod::<u32>(cpu.regs.get32(Register::ESI)),
            );
        }
    }
    if cpu.flags.contains(Flags::DF) {
        *cpu.regs.get32_mut(Register::ESI) -= size as u32;
    } else {
        *cpu.regs.get32_mut(Register::ESI) += size as u32;
    };
}

fn lods(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if Rep::is_rep(instr) {
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
