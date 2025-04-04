//! Ops that tend to loop with 'rep' prefix, e.g. movs, stos.

use super::math::sub;
use crate::{registers::Flags, x86::CPU};
use iced_x86::{Instruction, Register};
use memory::{Extensions, ExtensionsMut, Mem};

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

/// cmps: Compare String Operands
fn cmps(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(r) = Rep::from_instr(instr) {
        rep(cpu, mem, r, size, cmps_single);
    } else {
        cmps_single(cpu, mem, size);
    }
}

/// cmpsd: Compare String Operands
pub fn cmpsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cmps(cpu, mem, instr, Size::Dword);
}

/// cmpsw: Compare String Operands
pub fn cmpsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cmps(cpu, mem, instr, Size::Word);
}

/// cmpsb: Compare String Operands
pub fn cmpsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    cmps(cpu, mem, instr, Size::Byte);
}

/// movs: Move Data From String to String
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

/// movs: Move Data From String to String
fn movs(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if Rep::is_rep(instr) {
        rep(cpu, mem, Rep::REP, size, movs_single);
    } else {
        movs_single(cpu, mem, size);
    }
}

/// movsd: Move Data From String to String
pub fn movsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, Size::Dword)
}

/// movsw: Move Data From String to String
pub fn movsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, Size::Word)
}

/// movsb: Move Data From String to String
pub fn movsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    movs(cpu, mem, instr, Size::Byte)
}

/// scas: Scan String
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

/// scas: Scan String
fn scas(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if let Some(r) = Rep::from_instr(instr) {
        rep(cpu, mem, r, size, scas_single);
    } else {
        scas_single(cpu, mem, size);
    }
}

/// scasd: Scan String
pub fn scasd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, Size::Dword)
}

/// scasw: Scan String
pub fn scasw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, Size::Word)
}

/// scasb: Scan String
pub fn scasb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    scas(cpu, mem, instr, Size::Byte)
}

/// stos: Store String
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

/// stos: Store String
fn stos(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if Rep::is_rep(instr) {
        rep(cpu, mem, Rep::REP, size, stos_single);
    } else {
        stos_single(cpu, mem, size);
    }
}

/// stosd: Store String
pub fn stosd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Size::Dword)
}

/// stosw: Store String
pub fn stosw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Size::Word)
}

/// stosb: Store String
pub fn stosb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    stos(cpu, mem, instr, Size::Byte)
}

/// lods: Load String
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

/// lods: Load String
fn lods(cpu: &mut CPU, mem: Mem, instr: &Instruction, size: Size) {
    if Rep::is_rep(instr) {
        rep(cpu, mem, Rep::REP, size, lods_single);
    } else {
        lods_single(cpu, mem, size);
    }
}

/// lodsd: Load String
pub fn lodsd(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, Size::Dword)
}

/// lodsw: Load String
pub fn lodsw(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, Size::Word)
}

/// lodsb: Load String
pub fn lodsb(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    lods(cpu, mem, instr, Size::Byte)
}
