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
            eax: x86.regs.eax,
            ebx: x86.regs.ebx,
            ecx: x86.regs.ecx,
            edx: x86.regs.edx,
            esp: x86.regs.esp,
            ebp: x86.regs.ebp,
            esi: x86.regs.esi,
            edi: x86.regs.edi,
            eip: x86.regs.eip,
            cs: x86.regs.cs,
            ds: x86.regs.ds,
            es: x86.regs.es,
            fs: x86.regs.fs,
            gs: x86.regs.gs,
            ss: x86.regs.ss,
            flags: x86.flags.bits(),
            flags_str: format!("{:?}", x86.flags),
            st: x86.fpu.st[x86.fpu.st_top..].into(),
        }
    }
}
