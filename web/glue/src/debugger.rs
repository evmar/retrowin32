//! API used specifically for debugging the emulator.

use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// Registers are serialized as a JSON blob.
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub struct Registers {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    eip: u32,
    cs: u16,
    ds: u16,
    es: u16,
    fs: u16,
    gs: u16,
    ss: u16,
    flags: u32,
    flags_str: String,
    st: Box<[f64]>,
}

impl Registers {
    pub fn from_x86(x86: &x86::CPU) -> Registers {
        Registers {
            eax: x86.regs.get32(x86::Register::EAX),
            ebx: x86.regs.get32(x86::Register::EBX),
            ecx: x86.regs.get32(x86::Register::ECX),
            edx: x86.regs.get32(x86::Register::EDX),
            esp: x86.regs.get32(x86::Register::ESP),
            ebp: x86.regs.get32(x86::Register::EBP),
            esi: x86.regs.get32(x86::Register::ESI),
            edi: x86.regs.get32(x86::Register::EDI),
            eip: x86.regs.eip,
            cs: x86.regs.get16(x86::Register::CS),
            ds: x86.regs.get16(x86::Register::DS),
            es: x86.regs.get16(x86::Register::ES),
            fs: x86.regs.get16(x86::Register::FS),
            gs: x86.regs.get16(x86::Register::GS),
            ss: x86.regs.get16(x86::Register::SS),
            flags: x86.flags.bits(),
            flags_str: format!("{:?}", x86.flags),
            st: x86.fpu.st[x86.fpu.st_top..].into(),
        }
    }
}
