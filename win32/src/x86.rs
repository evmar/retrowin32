use std::collections::HashMap;

use anyhow::bail;
use bitflags::bitflags;
use tsify::Tsify;

use crate::{host, memory::Memory, pe::ImageSectionFlags, winapi, windows::load_exe};

/// If the x86 code does something wrong (like OOB read), we want to crash.
/// We cannot recover from a panic in wasm, so instead we use this janky flag.
// TODO: there are lots of array bounds in this codebase, we need to somehow
// track all them down(?).
static mut CRASHED: Option<String> = None;

pub fn crash(msg: String) {
    unsafe {
        CRASHED = Some(msg);
    }
}

/// Addresses from 0 up to this point cause panics if we access them.
/// This helps catch implementation bugs earlier.
pub const NULL_POINTER_REGION_SIZE: u32 = 0x1000;

/// Check whether reading a T from mem[addr] would cause OOB, and crash() if so.
fn check_oob<T>(mem: &[u8], addr: u32) -> bool {
    if addr < NULL_POINTER_REGION_SIZE {
        crash(format!("crash: null pointer at {addr:#x}"));
        return true;
    }
    if addr as usize + std::mem::size_of::<T>() > mem.len() {
        crash(format!("crash: oob pointer at {addr:#x}"));
        return true;
    }
    false
}

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

bitflags! {
    pub struct Flags: u32 {
        /// carry
        const CF = 1 << 0;
        /// zero
        const ZF = 1 << 6;
        /// sign
        const SF = 1 << 7;
        /// overflow
        const OF = 1 << 11;
    }
}

/// Offset from top of FP stack for a given ST0, ST1 etc reg.
fn st_offset(reg: iced_x86::Register) -> usize {
    match reg {
        iced_x86::Register::ST0 => 0,
        iced_x86::Register::ST1 => 1,
        _ => unreachable!("{reg:?}"),
    }
}

#[derive(Tsify)]
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,

    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,

    pub eip: u32,

    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,

    pub flags: Flags,

    /// FPU registers.
    pub st: [f64; 8],
    /// Top of FPU stack; 8 when stack empty.
    pub st_top: usize,
}
impl Registers {
    fn new() -> Self {
        Registers {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            eip: 0,
            cs: 0,
            ds: 0,
            es: 0,
            fs: 0,
            gs: 0,
            ss: 0,
            flags: Flags::empty(),

            st: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            st_top: 8,
        }
    }

    fn get32(&self, reg: iced_x86::Register) -> u32 {
        match reg {
            iced_x86::Register::EAX => self.eax,
            iced_x86::Register::EBX => self.ebx,
            iced_x86::Register::ECX => self.ecx,
            iced_x86::Register::EDX => self.edx,
            iced_x86::Register::ESP => self.esp,
            iced_x86::Register::EBP => self.ebp,
            iced_x86::Register::ESI => self.esi,
            iced_x86::Register::EDI => self.edi,
            _ => unreachable!("{reg:?}"),
        }
    }
    fn get16(&self, reg: iced_x86::Register) -> u16 {
        match reg {
            iced_x86::Register::AX => self.eax as u16,
            iced_x86::Register::CX => self.ecx as u16,
            iced_x86::Register::DX => self.edx as u16,
            iced_x86::Register::BX => self.ebx as u16,
            iced_x86::Register::SP => self.esp as u16,
            iced_x86::Register::BP => self.ebp as u16,
            iced_x86::Register::SI => self.esi as u16,
            iced_x86::Register::DI => self.edi as u16,
            iced_x86::Register::ES => self.es,
            iced_x86::Register::CS => self.cs,
            iced_x86::Register::SS => self.ss,
            iced_x86::Register::DS => self.ds,
            iced_x86::Register::FS => self.fs,
            iced_x86::Register::GS => self.gs,
            _ => unreachable!(),
        }
    }
    fn get8(&self, reg: iced_x86::Register) -> u8 {
        match reg {
            iced_x86::Register::AL => self.eax as u8,
            iced_x86::Register::CL => self.ecx as u8,
            iced_x86::Register::DL => self.edx as u8,
            iced_x86::Register::BL => self.ebx as u8,
            iced_x86::Register::AH => (self.eax >> 8) as u8,
            iced_x86::Register::CH => (self.ecx >> 8) as u8,
            iced_x86::Register::DH => (self.edx >> 8) as u8,
            iced_x86::Register::BH => (self.ebx >> 8) as u8,
            _ => unreachable!("{reg:?}"),
        }
    }

    fn set32(&mut self, reg: iced_x86::Register, value: u32) {
        match reg {
            iced_x86::Register::EAX => self.eax = value,
            iced_x86::Register::EBX => self.ebx = value,
            iced_x86::Register::ECX => self.ecx = value,
            iced_x86::Register::EDX => self.edx = value,
            iced_x86::Register::ESP => self.esp = value,
            iced_x86::Register::EBP => self.ebp = value,
            iced_x86::Register::ESI => self.esi = value,
            iced_x86::Register::EDI => self.edi = value,
            _ => unreachable!(),
        }
    }
    fn set16(&mut self, reg: iced_x86::Register, value: u16) {
        match reg {
            iced_x86::Register::AX => self.eax = (self.eax as u16 | value) as u32,
            iced_x86::Register::CX => self.ecx = (self.ecx as u16 | value) as u32,
            iced_x86::Register::DX => self.edx = (self.edx as u16 | value) as u32,
            iced_x86::Register::BX => self.ebx = (self.ebx as u16 | value) as u32,
            iced_x86::Register::SI => self.esi = (self.esi as u16 | value) as u32,
            iced_x86::Register::DI => self.edi = (self.edi as u16 | value) as u32,
            iced_x86::Register::ES => self.es = value,
            iced_x86::Register::CS => self.cs = value,
            iced_x86::Register::SS => self.ss = value,
            iced_x86::Register::DS => self.ds = value,
            iced_x86::Register::FS => self.fs = value,
            iced_x86::Register::GS => self.gs = value,
            _ => unreachable!("{reg:?}"),
        }
    }
    fn set8(&mut self, reg: iced_x86::Register, value: u8) {
        match reg {
            iced_x86::Register::AL => self.eax = (self.eax & 0xFFFF_FF00) | value as u32,
            iced_x86::Register::CL => self.ecx = (self.ecx & 0xFFFF_FF00) | value as u32,
            iced_x86::Register::DL => self.edx = (self.edx & 0xFFFF_FF00) | value as u32,
            iced_x86::Register::BL => self.ebx = (self.ebx & 0xFFFF_FF00) | value as u32,

            iced_x86::Register::AH => self.eax = (self.eax & 0xFFFF_00FF) | ((value as u32) << 8),
            iced_x86::Register::CH => self.ecx = (self.ecx & 0xFFFF_00FF) | ((value as u32) << 8),
            iced_x86::Register::DH => self.edx = (self.edx & 0xFFFF_00FF) | ((value as u32) << 8),
            iced_x86::Register::BH => self.ebx = (self.ebx & 0xFFFF_00FF) | ((value as u32) << 8),
            _ => unreachable!("{reg:?}"),
        }
    }

    /// Get st(0), the current top of the FPU stack.
    fn st_top(&mut self) -> &mut f64 {
        &mut self.st[self.st_top]
    }
    fn st_swap(&mut self, r1: iced_x86::Register, r2: iced_x86::Register) {
        self.st.swap(st_offset(r1), st_offset(r2));
    }
    fn getst(&mut self, reg: iced_x86::Register) -> &mut f64 {
        &mut self.st[self.st_top + st_offset(reg)]
    }
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims(Vec<Result<fn(&mut X86), String>>);
impl Shims {
    fn new() -> Self {
        Shims(Vec::new())
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, entry: Result<fn(&mut X86), String>) -> u32 {
        let id = SHIM_BASE | self.0.len() as u32;
        self.0.push(entry);
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut X86)> {
        let index = (addr & 0x0000_FFFF) as usize;
        let handler = self.0.get(index);
        match handler {
            Some(handler) => match handler {
                Ok(handler) => return Some(handler),
                Err(msg) => log::error!("{}", msg),
            },
            None => log::error!("unknown import reference at {:x}", addr),
        };
        None
    }
}

pub struct X86 {
    pub host: Box<dyn host::Host>,
    pub mem: Vec<u8>,
    pub regs: Registers,
    pub shims: Shims,
    pub state: winapi::State,
}
impl X86 {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let mut regs = Registers::new();
        regs.eax = 0xdeadbeea;
        regs.ebx = 0xdeadbeeb;
        regs.ecx = 0xdeadbeec;
        regs.edx = 0xdeadbeed;
        regs.esi = 0xdeadbe51;
        regs.edi = 0xdeadbed1;
        X86 {
            host,
            mem: Vec::new(),
            regs,
            shims: Shims::new(),
            state: winapi::State::new(),
        }
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) {
        if check_oob::<u32>(&self.mem, addr) {
            return;
        }
        self.mem.write_u32(addr, value);
    }
    pub fn write_u16(&mut self, addr: u32, value: u16) {
        if check_oob::<u16>(&self.mem, addr) {
            return;
        }
        let addr = addr as usize;
        // Safety: check_oob checked bounds.
        unsafe {
            *self.mem.get_unchecked_mut(addr) = value as u8;
            *self.mem.get_unchecked_mut(addr + 1) = (value >> 8) as u8;
        }
    }
    pub fn write_u8(&mut self, addr: u32, value: u8) {
        if check_oob::<u8>(&self.mem, addr) {
            return;
        }
        // Safety: check_oob checked bounds.
        unsafe { *self.mem.get_unchecked_mut(addr as usize) = value }
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        if check_oob::<u32>(&self.mem, addr) {
            return 0;
        }
        self.mem.read_u32(addr)
    }
    pub fn read_u16(&self, addr: u32) -> u16 {
        if check_oob::<u16>(&self.mem, addr) {
            return 0;
        }
        let offset = addr as usize;
        // Safety: check_oob checked bounds.
        unsafe {
            (*self.mem.get_unchecked(offset) as u16) << 0
                | (*self.mem.get_unchecked(offset + 1) as u16) << 8
        }
    }
    pub fn read_u8(&self, addr: u32) -> u8 {
        if check_oob::<u8>(&self.mem, addr) {
            return 0;
        }
        // Safety: check_oob checked bounds.
        unsafe { *self.mem.get_unchecked(addr as usize) }
    }

    pub fn read_f64(&self, addr: u32) -> f64 {
        if addr < NULL_POINTER_REGION_SIZE {
            panic!("null pointer read at {addr:#x}");
        }
        let addr = addr as usize;
        let n = u64::from_le_bytes(self.mem[addr..addr + 8].try_into().unwrap());
        f64::from_bits(n)
    }

    pub fn write_f64(&mut self, addr: u32, value: f64) {
        if addr < NULL_POINTER_REGION_SIZE {
            panic!("null pointer read at {addr:#x}");
        }
        let addr = addr as usize;
        self.mem[addr..addr + 8].copy_from_slice(&f64::to_le_bytes(value));
    }

    pub fn push(&mut self, value: u32) {
        self.regs.esp -= 4;
        self.write_u32(self.regs.esp, value);
    }

    pub fn pop(&mut self) -> u32 {
        let value = self.read_u32(self.regs.esp);
        self.regs.esp += 4;
        value
    }

    /// Compute the address found in instructions that reference memory, e.g.
    ///   mov [eax+03h],...
    fn addr(&self, instr: &iced_x86::Instruction) -> u32 {
        let seg = if instr.segment_prefix() == iced_x86::Register::FS {
            self.state.kernel32.teb
        } else {
            0
        };
        let base = if instr.memory_base() != iced_x86::Register::None {
            self.regs.get32(instr.memory_base())
        } else {
            0
        };
        let index = if instr.memory_index() != iced_x86::Register::None {
            self.regs.get32(instr.memory_index()) * instr.memory_index_scale()
        } else {
            0
        };
        (seg + base + index).wrapping_add(instr.memory_displacement32())
    }

    fn op1_rm32(&self, instr: &iced_x86::Instruction) -> u32 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get32(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
            _ => unreachable!(),
        }
    }
    fn op1_rm16(&self, instr: &iced_x86::Instruction) -> u16 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get16(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
            _ => unreachable!(),
        }
    }
    fn op1_rm8(&self, instr: &iced_x86::Instruction) -> u8 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get8(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    fn add32(&mut self, x: u32, y: u32) -> u32 {
        // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        let (result, carry) = x.overflowing_add(y);
        self.regs.flags.set(Flags::CF, carry);
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  0  1
        //   1  1  0
        let of = ((x ^ !y) & (x ^ result)) >> 31 != 0;
        self.regs.flags.set(Flags::OF, of);
        result
    }
    fn add8(&mut self, x: u8, y: u8) -> u8 {
        // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        let (result, carry) = x.overflowing_add(y);
        self.regs.flags.set(Flags::CF, carry);
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x80 != 0);
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  0  1
        //   1  1  0
        let of = ((x ^ !y) & (x ^ result)) >> 7 != 0;
        self.regs.flags.set(Flags::OF, of);
        result
    }

    fn sub32(&mut self, x: u32, y: u32) -> u32 {
        let (result, carry) = x.overflowing_sub(y);
        // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::CF, carry);
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
        // Overflow is true exactly when the high (sign) bits are like:
        //   x  y  result
        //   0  1  1
        //   1  0  0
        let of = ((x ^ y) & (x ^ result)) >> 31 != 0;
        self.regs.flags.set(Flags::OF, of);
        result
    }
    fn sub16(&mut self, x: u16, y: u16) -> u16 {
        let (result, carry) = x.overflowing_sub(y);
        // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::CF, carry);
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x80 != 0);
        // See comment in sub32.
        let of = ((x ^ y) & (x ^ result)) >> 7 != 0;
        self.regs.flags.set(Flags::OF, of);
        result
    }
    fn sub8(&mut self, x: u8, y: u8) -> u8 {
        let (result, carry) = x.overflowing_sub(y);
        // TODO "The CF, OF, SF, ZF, AF, and PF flags are set according to the result."
        self.regs.flags.set(Flags::CF, carry);
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x80 != 0);
        // See comment in sub32.
        let of = ((x ^ y) & (x ^ result)) >> 7 != 0;
        self.regs.flags.set(Flags::OF, of);
        result
    }

    fn and32(&mut self, x: u32, y: u32) -> u32 {
        let result = x & y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x8000_0000 != 0);
        self.regs.flags.set(Flags::OF, false);
        result
    }
    fn and16(&mut self, x: u16, y: u16) -> u16 {
        let result = x & y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x8000 != 0);
        self.regs.flags.set(Flags::OF, false);
        result
    }
    fn and8(&mut self, x: u8, y: u8) -> u8 {
        let result = x & y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        self.regs.flags.set(Flags::SF, result & 0x80 != 0);
        self.regs.flags.set(Flags::OF, false);
        result
    }
    fn or32(&mut self, x: u32, y: u32) -> u32 {
        let result = x | y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }
    fn or16(&mut self, x: u16, y: u16) -> u16 {
        let result = x | y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }
    fn or8(&mut self, x: u8, y: u8) -> u8 {
        let result = x | y;
        // XXX More flags.
        self.regs.flags.set(Flags::ZF, result == 0);
        result
    }

    fn shl32(&mut self, x: u32, y: u8) -> u32 {
        // Note: overflowing_shl is not what we want.
        let val = (x as u64).wrapping_shl(y as u32);
        self.regs.flags.set(Flags::CF, val & (1 << 32) != 0);
        val as u32
    }
    fn shl8(&mut self, x: u8, y: u8) -> u8 {
        // Note: overflowing_shl is not what we want.
        let val = (x as u16).wrapping_shl(y as u32);
        self.regs.flags.set(Flags::CF, val & (1 << 8) != 0);
        val as u8
    }

    fn rm32_x(&mut self, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u32) -> u32) {
        match instr.op0_kind() {
            iced_x86::OpKind::Register => {
                let reg = instr.op0_register();
                let x = self.regs.get32(reg);
                let value = op(self, x);
                self.regs.set32(reg, value);
            }
            iced_x86::OpKind::Memory => {
                let addr = self.addr(instr);
                let x = self.read_u32(addr);
                let value = op(self, x);
                self.write_u32(addr, value);
            }
            _ => unimplemented!(),
        }
    }
    fn rm16_x(&mut self, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u16) -> u16) {
        match instr.op0_kind() {
            iced_x86::OpKind::Register => {
                let reg = instr.op0_register();
                let x = self.regs.get16(reg);
                let value = op(self, x);
                self.regs.set16(reg, value);
            }
            iced_x86::OpKind::Memory => {
                let addr = self.addr(instr);
                let x = self.read_u16(addr);
                let value = op(self, x);
                self.write_u16(addr, value);
            }
            _ => unimplemented!(),
        }
    }
    fn rm8_x(&mut self, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u8) -> u8) {
        match instr.op0_kind() {
            iced_x86::OpKind::Register => {
                let reg = instr.op0_register();
                let x = self.regs.get8(reg);
                let value = op(self, x);
                self.regs.set8(reg, value);
            }
            iced_x86::OpKind::Memory => {
                let addr = self.addr(instr);
                let x = self.read_u8(addr);
                let value = op(self, x);
                self.write_u8(addr, value);
            }
            _ => unimplemented!(),
        }
    }

    fn jmp(&mut self, addr: u32) -> anyhow::Result<()> {
        if addr < 0x1000 {
            bail!("jmp to null page");
        }

        if addr & 0xFFFF_0000 == SHIM_BASE {
            let ret = self.pop();
            let eip = self.regs.eip;
            let handler = self.shims.get(addr).unwrap();
            handler(self);
            if self.regs.eip != eip {
                return Ok(()); // handler set eip.
            }
            return self.jmp(ret);
        }

        self.regs.eip = addr;
        Ok(())
    }

    /// Returns Ok(false) if we hit a breakpoint.
    fn run(&mut self, instr: &iced_x86::Instruction) -> anyhow::Result<bool> {
        self.regs.eip = instr.next_ip() as u32;
        match instr.code() {
            iced_x86::Code::Enterd_imm16_imm8 => {
                self.push(self.regs.ebp);
                self.regs.ebp = self.regs.esp;
                self.regs.esp -= instr.immediate16() as u32;
            }
            iced_x86::Code::Leaved => {
                self.regs.esp = self.regs.ebp;
                self.regs.ebp = self.pop();
            }

            iced_x86::Code::Call_rel32_32 => {
                let target = instr.near_branch32();
                if target == 0x00408d65 || target == 0x0040a281 {
                    log::warn!("HACK: manually nop'd call at {target:x}");
                } else {
                    self.push(self.regs.eip);
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Call_rm32 => {
                // call dword ptr [addr]
                let target = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                self.push(self.regs.eip);
                self.jmp(target)?;
            }
            iced_x86::Code::Retnd => {
                let addr = self.pop();
                self.jmp(addr)?;
            }
            iced_x86::Code::Retnd_imm16 => {
                let addr = self.pop();
                self.jmp(addr)?;
                self.regs.esp += instr.immediate16() as u32;
            }

            iced_x86::Code::Jmp_rel8_32 => {
                self.jmp(instr.near_branch32())?;
            }
            iced_x86::Code::Jmp_rel32_32 => {
                self.jmp(instr.near_branch32())?;
            }
            iced_x86::Code::Jmp_rm32 => {
                let target = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                self.jmp(target)?;
            }

            iced_x86::Code::Ja_rel8_32 => {
                if !self.regs.flags.contains(Flags::CF) && !self.regs.flags.contains(Flags::ZF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jae_rel32_32 | iced_x86::Code::Jae_rel8_32 => {
                if !self.regs.flags.contains(Flags::CF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jb_rel8_32 | iced_x86::Code::Jb_rel32_32 => {
                if self.regs.flags.contains(Flags::CF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jbe_rel32_32 | iced_x86::Code::Jbe_rel8_32 => {
                if self.regs.flags.contains(Flags::CF) || self.regs.flags.contains(Flags::ZF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Je_rel8_32 => {
                if self.regs.flags.contains(Flags::ZF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Je_rel32_32 => {
                if self.regs.flags.contains(Flags::ZF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jecxz_rel8_32 => {
                if self.regs.ecx == 0 {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jne_rel32_32 | iced_x86::Code::Jne_rel8_32 => {
                if !self.regs.flags.contains(Flags::ZF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jg_rel8_32 => {
                if !self.regs.flags.contains(Flags::ZF)
                    && (self.regs.flags.contains(Flags::SF) == self.regs.flags.contains(Flags::OF))
                {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jge_rel32_32 | iced_x86::Code::Jge_rel8_32 => {
                if self.regs.flags.contains(Flags::SF) == self.regs.flags.contains(Flags::OF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jl_rel32_32 => {
                if self.regs.flags.contains(Flags::SF) != self.regs.flags.contains(Flags::OF) {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jle_rel32_32 | iced_x86::Code::Jle_rel8_32 => {
                if self.regs.flags.contains(Flags::ZF)
                    || (self.regs.flags.contains(Flags::SF) != self.regs.flags.contains(Flags::OF))
                {
                    self.jmp(instr.near_branch32())?;
                }
            }
            iced_x86::Code::Jl_rel8_32 => {
                if self.regs.flags.contains(Flags::SF) != self.regs.flags.contains(Flags::OF) {
                    self.jmp(instr.near_branch32())?;
                }
            }

            iced_x86::Code::Pushd_imm8 => self.push(instr.immediate8to32() as u32),
            iced_x86::Code::Pushd_imm32 => self.push(instr.immediate32()),
            iced_x86::Code::Push_r32 => self.push(self.regs.get32(instr.op0_register())),
            iced_x86::Code::Push_rm32 => {
                // push [eax+10h]
                let value = self.read_u32(self.addr(instr));
                self.push(value);
            }

            iced_x86::Code::Pop_r32 | iced_x86::Code::Pop_rm32 => {
                let value = self.pop();
                self.rm32_x(instr, |_x86, _x| value);
            }

            iced_x86::Code::Mov_rm32_imm32 => {
                // mov dword ptr [x], y
                // TODO: why is this 'rm32' when there is an r32 variant just below?
                self.rm32_x(instr, |_x86, _x| instr.immediate32());
            }
            iced_x86::Code::Mov_r32_imm32 => {
                self.regs.set32(instr.op0_register(), instr.immediate32());
            }
            iced_x86::Code::Mov_moffs32_EAX => {
                // mov [x],eax
                self.write_u32(self.addr(instr), self.regs.eax);
            }
            iced_x86::Code::Mov_EAX_moffs32 => {
                // mov eax,[x]
                self.regs.eax = self.read_u32(self.addr(instr));
            }
            iced_x86::Code::Mov_rm32_r32 => {
                let value = self.regs.get32(instr.op1_register());
                self.rm32_x(instr, |_x86, _x| value);
            }
            iced_x86::Code::Mov_r32_rm32 => {
                self.regs.set32(instr.op0_register(), self.op1_rm32(instr));
            }
            iced_x86::Code::Mov_r16_rm16 => {
                self.regs.set16(instr.op0_register(), self.op1_rm16(instr));
            }
            iced_x86::Code::Mov_rm16_r16 | iced_x86::Code::Mov_rm16_Sreg => {
                let y = instr.immediate16();
                self.rm16_x(instr, |_x86, _x| y);
            }
            iced_x86::Code::Mov_r8_rm8 => {
                self.regs.set8(instr.op0_register(), self.op1_rm8(instr));
            }
            iced_x86::Code::Mov_rm8_r8 => {
                let y = self.regs.get8(instr.op1_register());
                self.rm8_x(instr, |_x86, _x| y);
            }
            iced_x86::Code::Mov_r8_imm8 | iced_x86::Code::Mov_rm8_imm8 => {
                let y = instr.immediate8();
                self.rm8_x(instr, |_x86, _x| y);
            }

            iced_x86::Code::Movsx_r32_rm16 => {
                let y = self.op1_rm16(instr) as i16 as u32;
                self.rm32_x(instr, |_x86, _x| y);
            }
            iced_x86::Code::Movsx_r32_rm8 => {
                let y = self.op1_rm8(instr) as i8 as u32;
                self.rm32_x(instr, |_x86, _x| y);
            }
            iced_x86::Code::Movzx_r32_rm8 => {
                let y = self.op1_rm8(instr) as u32;
                self.rm32_x(instr, |_x86, _x| y);
            }
            iced_x86::Code::Movzx_r16_rm8 => {
                let y = self.op1_rm8(instr) as u16;
                self.rm16_x(instr, |_x86, _x| y);
            }

            iced_x86::Code::Xchg_rm32_r32 => {
                let r1 = instr.op1_register();
                self.rm32_x(instr, |x86, x| {
                    let tmp = x86.regs.get32(r1);
                    x86.regs.set32(r1, x);
                    tmp
                });
            }
            iced_x86::Code::Cmpxchg_rm32_r32 => {
                let y = self.regs.get32(instr.op1_register());
                match instr.op0_kind() {
                    iced_x86::OpKind::Register => todo!(),
                    iced_x86::OpKind::Memory => {
                        let addr = self.addr(instr);
                        let x = self.mem.read_u32(addr);
                        if self.regs.eax == x {
                            self.mem.write_u32(addr, y);
                        } else {
                            self.regs.eax = y;
                        }
                    }
                    _ => unreachable!(),
                };
            }

            iced_x86::Code::Cmpsb_m8_m8 => {
                let p1 = self.regs.esi as usize;
                let p2 = self.regs.edi as usize;
                let count = self.regs.ecx as usize;
                if instr.has_repe_prefix() {
                    let pos = self.mem[p1..p1 + count]
                        .iter()
                        .zip(self.mem[p2..p2 + count].iter())
                        .position(|(&x, &y)| x == y)
                        .unwrap_or(count);
                    self.regs.esi += pos as u32;
                    self.regs.edi += pos as u32;
                    self.regs.ecx -= pos as u32;
                    self.sub8(self.read_u8(self.regs.esi), self.read_u8(self.regs.edi));
                } else {
                    bail!("unimpl");
                }
            }
            iced_x86::Code::Movsb_m8_m8 => {
                if !instr.has_rep_prefix() {
                    bail!("expected rep movsb");
                }
                let dst = self.regs.edi as usize;
                let src = self.regs.esi as usize;
                let count = self.regs.ecx as usize;
                self.mem.copy_within(src..src + count, dst);
                self.regs.edi += count as u32;
                self.regs.esi += count as u32;
                self.regs.ecx = 0;
            }
            iced_x86::Code::Scasb_AL_m8 => {
                let src = self.regs.edi as usize;
                let value = self.regs.eax as u8;
                let count = self.regs.ecx as usize;
                if instr.has_repne_prefix() {
                    let pos = self.mem[src..src + count]
                        .iter()
                        .position(|&c| c == value)
                        .unwrap_or(count);
                    self.regs.edi += pos as u32;
                    self.regs.ecx -= pos as u32;
                    self.sub8(
                        self.regs.get8(iced_x86::Register::AL),
                        self.regs.get8(iced_x86::Register::DL),
                    );
                } else {
                    bail!("unimpl");
                }
            }
            iced_x86::Code::Stosd_m32_EAX => {
                let dst = self.regs.edi as usize;
                let value = self.regs.eax;
                if value != 0 {
                    bail!("TODO: stosd impl");
                }
                if instr.has_rep_prefix() {
                    let count = self.regs.ecx as usize;
                    self.mem[dst..dst + count * 4].fill(0);
                    self.regs.edi += count as u32 * 4;
                    self.regs.ecx = 0;
                } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
                    bail!("unimpl");
                } else {
                    self.mem[dst..dst + 4].fill(0);
                    self.regs.edi += 4;
                }
            }
            iced_x86::Code::Stosb_m8_AL => {
                let dst = self.regs.edi as usize;
                let value = self.regs.eax as u8;
                if instr.has_rep_prefix() {
                    let count = self.regs.ecx as usize;
                    self.mem[dst..dst + count].fill(value);
                    self.regs.edi += count as u32;
                    self.regs.ecx = 0;
                } else if instr.has_repe_prefix() || instr.has_repne_prefix() {
                    bail!("unimpl");
                } else {
                    self.mem[dst] = value;
                    self.regs.edi += 1;
                }
            }

            iced_x86::Code::Lodsd_EAX_m32 => {
                assert!(
                    !instr.has_rep_prefix()
                        && !instr.has_repe_prefix()
                        && !instr.has_repne_prefix()
                );
                self.regs.eax = self.read_u32(self.regs.esi);
                self.regs.esi += 4;
            }

            iced_x86::Code::And_rm32_imm32 | iced_x86::Code::And_EAX_imm32 => {
                let y = instr.immediate32();
                self.rm32_x(instr, |x86, x| x86.and32(x, y));
            }
            iced_x86::Code::And_rm32_imm8 => {
                let y = instr.immediate8to32() as u32;
                self.rm32_x(instr, |x86, x| x86.and32(x, y));
            }
            iced_x86::Code::And_r32_rm32 => {
                let reg = instr.op0_register();
                let y = self.op1_rm32(instr);
                let value = self.regs.get32(reg) & y;
                self.regs.set32(reg, value);
            }
            iced_x86::Code::And_rm16_imm16 => {
                let y = instr.immediate16();
                self.rm16_x(instr, |x86, x| x86.and16(x, y));
            }
            iced_x86::Code::And_rm8_imm8 => {
                let y = instr.immediate8();
                self.rm8_x(instr, |x86, x| x86.and8(x, y));
            }
            iced_x86::Code::Or_rm32_r32 => {
                let y = self.op1_rm32(instr);
                self.rm32_x(instr, |x86, x| x86.or32(x, y));
            }
            iced_x86::Code::Or_rm32_imm8 => {
                let y = instr.immediate8to32() as u32;
                self.rm32_x(instr, |x86, x| x86.or32(x, y));
            }
            iced_x86::Code::Or_rm16_imm16 => {
                let y = instr.immediate16();
                self.rm16_x(instr, |x86, x| x86.or16(x, y));
            }
            iced_x86::Code::Or_rm8_imm8 => {
                let y = instr.immediate8();
                self.rm8_x(instr, |x86, x| x86.or8(x, y));
            }
            iced_x86::Code::Shl_rm32_imm8 => {
                let y = instr.immediate8();
                self.rm32_x(instr, |x86, x| x86.shl32(x, y));
            }
            iced_x86::Code::Shl_rm32_CL => {
                let y = self.regs.ecx as u8;
                self.rm32_x(instr, |x86, x| x86.shl32(x, y));
            }
            iced_x86::Code::Shl_rm8_CL => {
                let y = self.regs.ecx as u8;
                self.rm8_x(instr, |x86, x| x86.shl8(x, y));
            }
            iced_x86::Code::Shr_rm32_imm8 => {
                let y = instr.immediate8() as u32;
                self.rm32_x(instr, |_x86, x| x >> y);
            }
            iced_x86::Code::Sar_rm32_imm8 | iced_x86::Code::Sar_rm32_1 => {
                let y = instr.immediate8() as u32;
                self.rm32_x(instr, |_x86, x| (x >> y) | (x & 0x8000_0000));
            }
            iced_x86::Code::Xor_rm32_r32 => {
                let y = self.regs.get32(instr.op1_register());
                self.rm32_x(instr, |_x86, x| x ^ y);
            }
            iced_x86::Code::Xor_rm32_imm8 => {
                let y = instr.immediate8to32() as u32;
                self.rm32_x(instr, |_x86, x| x ^ y);
            }
            iced_x86::Code::Xor_r32_rm32 => {
                let reg = instr.op0_register();
                let y = self.op1_rm32(instr);
                let value = self.regs.get32(reg) ^ y;
                self.regs.set32(reg, value);
            }
            iced_x86::Code::Xor_rm8_imm8 => {
                let y = instr.immediate8();
                self.rm8_x(instr, |_x86, x| x ^ y);
            }
            iced_x86::Code::Xor_r8_rm8 => {
                let y = self.op1_rm8(instr);
                self.rm8_x(instr, |_x86, x| x ^ y);
            }

            iced_x86::Code::Add_r32_rm32 => {
                let reg = instr.op0_register();
                let value = self.add32(self.regs.get32(reg), self.op1_rm32(&instr));
                self.regs.set32(reg, value);
            }
            iced_x86::Code::Add_rm32_r32 => {
                let y = self.regs.get32(instr.op1_register());
                self.rm32_x(instr, |x86, x| x86.add32(x, y));
            }
            iced_x86::Code::Add_rm32_imm32 | iced_x86::Code::Add_EAX_imm32 => {
                let y = instr.immediate32();
                self.rm32_x(instr, |x86, x| x86.add32(x, y));
            }
            iced_x86::Code::Add_rm32_imm8 => {
                let y = instr.immediate8to32() as u32;
                self.rm32_x(instr, |x86, x| x86.add32(x, y));
            }
            iced_x86::Code::Add_rm8_imm8 => {
                let y = instr.immediate8();
                self.rm8_x(instr, |x86, x| x86.add8(x, y));
            }

            iced_x86::Code::Sub_rm32_imm8 => {
                let y = instr.immediate8to32() as u32;
                self.rm32_x(instr, |x86, x| x86.sub32(x, y));
            }
            iced_x86::Code::Sub_EAX_imm32 | iced_x86::Code::Sub_rm32_imm32 => {
                let y = instr.immediate32();
                self.rm32_x(instr, |x86, x| x86.sub32(x, y));
            }
            iced_x86::Code::Sub_rm32_r32 => {
                let y = self.regs.get32(instr.op1_register());
                self.rm32_x(instr, |x86, x| x86.sub32(x, y));
            }
            iced_x86::Code::Sub_r32_rm32 => {
                let reg = instr.op0_register();
                let y = self.op1_rm32(instr);
                let value = self.sub32(self.regs.get32(reg), y);
                self.regs.set32(reg, value);
            }
            iced_x86::Code::Sub_rm8_imm8 => {
                let y = instr.immediate8();
                self.rm8_x(instr, |x86, x| x86.sub8(x, y));
            }

            iced_x86::Code::Sbb_r32_rm32 => {
                let reg = instr.op0_register();
                let carry = self.regs.flags.contains(Flags::CF) as u32;
                let y = self.op1_rm32(instr).wrapping_add(carry);
                let value = self.sub32(self.regs.get32(reg), y);
                self.regs.set32(reg, value);
            }
            iced_x86::Code::Sbb_r8_rm8 => {
                let reg = instr.op0_register();
                let carry = self.regs.flags.contains(Flags::CF) as u8;
                let y = self.op1_rm8(instr).wrapping_add(carry);
                let value = self.sub8(self.regs.get8(reg), y);
                self.regs.set8(reg, value);
            }
            iced_x86::Code::Imul_r32_rm32 => {
                let x = self.regs.get32(instr.op0_register());
                let y = self.op1_rm32(instr);
                let value = x.wrapping_mul(y);
                self.regs.set32(instr.op0_register(), value);
            }
            iced_x86::Code::Imul_r32_rm32_imm32 => {
                let x = self.op1_rm32(instr) as i32;
                let y = instr.immediate32() as i32;
                let value = x.wrapping_mul(y);
                self.regs.set32(instr.op0_register(), value as u32);
            }
            iced_x86::Code::Imul_r32_rm32_imm8 => {
                let x = self.op1_rm32(instr) as i32;
                let y = instr.immediate8to32();
                let value = x.wrapping_mul(y);
                self.regs.set32(instr.op0_register(), value as u32);
            }
            iced_x86::Code::Dec_r32 => {
                self.rm32_x(instr, |x86, x| x86.sub32(x, 1));
            }
            iced_x86::Code::Inc_r32 | iced_x86::Code::Inc_rm32 => {
                // TODO: flags.  Note that it's not add32(1) because CF should be preserved.
                self.rm32_x(instr, |_x86, x| x + 1);
            }
            iced_x86::Code::Inc_rm8 => {
                // TODO: flags.  Note that it's not add8(1) because CF should be preserved.
                self.rm8_x(instr, |_x86, x| x.wrapping_add(1));
            }
            iced_x86::Code::Neg_rm32 => {
                self.rm32_x(instr, |x86, x| {
                    x86.regs.flags.set(Flags::CF, x != 0);
                    // TODO: other flags registers.
                    -(x as i32) as u32
                });
            }

            iced_x86::Code::Not_rm32 => {
                self.rm32_x(instr, |_x86, x| !x);
            }

            iced_x86::Code::Lea_r32_m => {
                // lea eax,[esp+10h]
                self.regs.set32(instr.op0_register(), self.addr(instr));
            }

            iced_x86::Code::Cmp_rm32_r32 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get32(instr.op1_register());
                self.sub32(x, y);
            }
            iced_x86::Code::Cmp_r32_rm32 => {
                let x = self.regs.get32(instr.op0_register());
                let y = self.op1_rm32(instr);
                self.sub32(x, y);
            }
            iced_x86::Code::Cmp_EAX_imm32 | iced_x86::Code::Cmp_rm32_imm32 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate32();
                self.sub32(x, y);
            }
            iced_x86::Code::Cmp_rm32_imm8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate8to32() as u32;
                self.sub32(x, y);
            }
            iced_x86::Code::Cmp_rm16_r16 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get16(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.op1_rm16(instr);
                self.sub16(x, y);
            }
            iced_x86::Code::Cmp_rm16_imm16 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get16(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate16();
                self.sub16(x, y);
            }
            iced_x86::Code::Cmp_rm16_imm8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get16(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate8to16() as u16;
                self.sub16(x, y);
            }
            iced_x86::Code::Cmp_rm8_imm8 | iced_x86::Code::Cmp_AL_imm8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get8(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate8();
                self.sub8(x, y);
            }
            iced_x86::Code::Cmp_rm8_r8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get8(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get8(instr.op1_register());
                self.sub8(x, y);
            }
            iced_x86::Code::Cmp_r8_rm8 => {
                let x = self.regs.get8(instr.op0_register());
                let y = match instr.op1_kind() {
                    iced_x86::OpKind::Register => self.regs.get8(instr.op1_register()),
                    iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
                    _ => unreachable!(),
                };
                self.sub8(x, y);
            }

            iced_x86::Code::Test_rm32_r32 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get32(instr.op1_register());
                self.and32(x, y);
            }
            iced_x86::Code::Test_rm32_imm32 | iced_x86::Code::Test_EAX_imm32 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate32();
                self.and32(x, y);
            }
            iced_x86::Code::Test_rm16_r16 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get16(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get16(instr.op1_register());
                self.and16(x, y);
            }
            iced_x86::Code::Test_rm8_r8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get8(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = self.regs.get8(instr.op1_register());
                self.and8(x, y);
            }
            iced_x86::Code::Test_rm8_imm8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get8(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate8();
                self.and8(x, y);
            }

            iced_x86::Code::Bt_rm32_imm8 => {
                let x = match instr.op0_kind() {
                    iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
                    iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
                    _ => unreachable!(),
                };
                let y = instr.immediate8() % 32;
                self.regs.flags.set(Flags::CF, ((x >> y) & 1) != 0);
            }

            iced_x86::Code::Sete_rm8 => {
                let value = self.regs.flags.contains(Flags::ZF) as u8;
                self.rm8_x(instr, |_x86, _x| value);
            }
            iced_x86::Code::Setne_rm8 => {
                let value = !self.regs.flags.contains(Flags::ZF) as u8;
                self.rm8_x(instr, |_x86, _x| value);
            }
            iced_x86::Code::Setge_rm8 => {
                let value = (self.regs.flags.contains(Flags::ZF)
                    == self.regs.flags.contains(Flags::OF)) as u8;
                self.rm8_x(instr, |_x86, _x| value);
            }

            iced_x86::Code::Fld1 => {
                self.regs.st_top -= 1;
                *self.regs.st_top() = 1.0;
            }
            iced_x86::Code::Fldz => {
                self.regs.st_top -= 1;
                *self.regs.st_top() = 0.0;
            }

            iced_x86::Code::Fld_m64fp => {
                self.regs.st_top -= 1;
                *self.regs.st_top() = self.read_f64(self.addr(instr));
            }
            iced_x86::Code::Fld_m32fp => {
                self.regs.st_top -= 1;
                *self.regs.st_top() = f32::from_bits(self.read_u32(self.addr(instr))) as f64;
            }
            iced_x86::Code::Fild_m32int => {
                self.regs.st_top -= 1;
                *self.regs.st_top() = self.read_u32(self.addr(instr)) as i32 as f64;
            }
            iced_x86::Code::Fild_m16int => {
                self.regs.st_top -= 1;
                *self.regs.st_top() = self.read_u16(self.addr(instr)) as i16 as f64;
            }

            iced_x86::Code::Fst_m64fp => {
                let f = *self.regs.st_top();
                self.write_f64(self.addr(instr), f);
            }
            iced_x86::Code::Fstp_m64fp => {
                let f = *self.regs.st_top();
                self.write_f64(self.addr(instr), f);
                self.regs.st_top += 1;
            }
            iced_x86::Code::Fstp_m32fp => {
                let f = *self.regs.st_top();
                self.write_u32(self.addr(instr), (f as f32).to_bits());
                self.regs.st_top += 1;
            }
            iced_x86::Code::Fistp_m32int => {
                let f = *self.regs.st_top();
                self.write_u32(self.addr(instr), f as i32 as u32);
                self.regs.st_top += 1;
            }

            iced_x86::Code::Fchs => {
                *self.regs.st_top() = -*self.regs.st_top();
            }

            iced_x86::Code::Fcos => {
                let reg = self.regs.st_top();
                *reg = reg.cos();
            }

            iced_x86::Code::Fsin => {
                let reg = self.regs.st_top();
                *reg = reg.sin();
            }

            iced_x86::Code::Fsqrt => {
                let reg = self.regs.st_top();
                *reg = reg.sqrt();
            }

            iced_x86::Code::Fadd_m64fp => {
                let y = self.read_f64(self.addr(instr));
                *self.regs.st_top() += y;
            }
            iced_x86::Code::Fadd_m32fp => {
                let y = f32::from_bits(self.read_u32(self.addr(instr))) as f64;
                *self.regs.st_top() += y;
            }
            iced_x86::Code::Faddp_sti_st0 => {
                let y = *self.regs.getst(instr.op1_register());
                let x = self.regs.getst(instr.op0_register());
                *x += y;
                self.regs.st_top += 1;
            }

            iced_x86::Code::Fsubr_m64fp => {
                let x = self.read_f64(self.addr(instr));
                let y = self.regs.st_top();
                *y = x - *y;
            }

            iced_x86::Code::Fmul_m64fp => {
                let y = self.read_f64(self.addr(instr));
                *self.regs.st_top() *= y;
            }
            iced_x86::Code::Fmul_m32fp => {
                let y = f32::from_bits(self.read_u32(self.addr(instr))) as f64;
                *self.regs.st_top() *= y;
            }
            iced_x86::Code::Fmul_st0_sti | iced_x86::Code::Fmul_sti_st0 => {
                let y = *self.regs.getst(instr.op1_register());
                let x = self.regs.getst(instr.op0_register());
                *x *= y;
            }
            iced_x86::Code::Fmulp_sti_st0 => {
                let y = *self.regs.st_top();
                let x = self.regs.getst(instr.op0_register());
                *x *= y;
                self.regs.st_top += 1;
            }

            iced_x86::Code::Fxch_st0_sti => {
                self.regs
                    .st_swap(instr.op0_register(), instr.op1_register());
            }

            iced_x86::Code::Pushad => {
                let esp = self.regs.esp;
                self.push(self.regs.eax);
                self.push(self.regs.ecx);
                self.push(self.regs.edx);
                self.push(self.regs.ebx);
                self.push(esp);
                self.push(self.regs.ebp);
                self.push(self.regs.esi);
                self.push(self.regs.edi);
            }
            iced_x86::Code::Popad => {
                self.regs.edi = self.pop();
                self.regs.esi = self.pop();
                self.regs.ebp = self.pop();
                self.pop(); // ignore esp
                self.regs.ebx = self.pop();
                self.regs.edx = self.pop();
                self.regs.ecx = self.pop();
                self.regs.eax = self.pop();
            }
            iced_x86::Code::Pushfd => {
                self.push(self.regs.flags.bits());
            }
            iced_x86::Code::Popfd => {
                self.regs.flags = Flags::from_bits(self.pop()).unwrap();
            }

            iced_x86::Code::Cwde => {
                self.regs.eax = self.regs.eax as i16 as i32 as u32;
            }
            iced_x86::Code::Cdq => {
                self.regs.edx = if self.regs.eax >> 31 == 0 {
                    0
                } else {
                    0xFFFF_FFFF
                };
            }

            iced_x86::Code::Nopd => {}

            iced_x86::Code::Int3 => {
                return Ok(false);
            }

            code => {
                bail!("unhandled instruction {:?}", code);
            }
        }
        Ok(true)
    }
}

/// Manages decoding and running instructions in an owned X86.
pub struct Runner {
    pub x86: X86,
    /// Total number of instructions executed.
    pub instr_count: usize,

    /// Places to stop execution in a step_many() call.
    /// We unconditionally stop on these; the web frontend manages things like
    /// enabling/disabling breakpoints.  The map values are the instruction
    /// from before the breakpoint.
    breakpoints: HashMap<u32, iced_x86::Instruction>,

    /// Code section of executable we're decoding.
    instrs: Vec<(u32, iced_x86::Instruction)>,
    /// Current position within code.  Updated from eip.
    instr_index: usize,
}
impl Runner {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Runner {
            x86: X86::new(host),
            instr_count: 0,
            breakpoints: HashMap::new(),
            instrs: Vec::new(),
            instr_index: 0,
        }
    }

    fn disassemble(buf: &[u8], ip: u32) -> Vec<(u32, iced_x86::Instruction)> {
        let mut code = Vec::new();
        let mut decoder =
            iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
        while decoder.can_decode() {
            code.push((decoder.ip() as u32, decoder.decode()));
        }
        code
    }

    fn ip_to_instr_index(&self, target_ip: u32) -> Result<usize, &'static str> {
        match self.instrs.binary_search_by_key(&target_ip, |&(ip, _)| ip) {
            Ok(pos) => Ok(pos),
            Err(_) => {
                // I think we may hit this case if the disassembler gets desynchronized from the instruction
                // stream.  We might need to re-disassemble or something in that case.
                Err("invalid ip, possible desync?")
            }
        }
    }

    fn jmp(&mut self, target_ip: u32) {
        let (cur_ip, _) = self.instrs[self.instr_index];
        if cur_ip == target_ip {
            return;
        }
        self.instr_index = self.ip_to_instr_index(target_ip).unwrap();
    }

    pub fn load_exe(&mut self, buf: &[u8]) -> anyhow::Result<HashMap<u32, String>> {
        let labels = load_exe(&mut self.x86, buf)?;

        // Disassemble the 'code' section.
        let mapping = self
            .x86
            .state
            .kernel32
            .mappings
            .iter()
            .find(|mapping| mapping.flags.contains(ImageSectionFlags::CODE))
            .ok_or_else(|| anyhow::anyhow!("no code section"))?;
        let section = &self.x86.mem[mapping.addr as usize..(mapping.addr + mapping.size) as usize];
        self.instrs = Runner::disassemble(section, mapping.addr);
        self.jmp(self.x86.regs.eip);

        Ok(labels)
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) -> anyhow::Result<()> {
        let index = self
            .ip_to_instr_index(addr)
            .map_err(|err| anyhow::anyhow!(err))?;
        let mut int3 = iced_x86::Instruction::with(iced_x86::Code::Int3);
        // The instruction needs a length/next_ip so the execution machinery doesn't lose its location.
        int3.set_len(1);
        int3.set_next_ip(addr as u64 + 1);
        let prev = std::mem::replace(&mut self.instrs[index].1, int3);
        self.breakpoints.insert(addr, prev);
        Ok(())
    }
    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) -> anyhow::Result<()> {
        let index = self
            .ip_to_instr_index(addr)
            .map_err(|err| anyhow::anyhow!(err))?;
        let prev = self.breakpoints.remove(&addr).unwrap();
        self.instrs[index].1 = prev;
        Ok(())
    }

    // Single-step execution.  Returns Ok(false) on breakpoint.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        let instr = &self.instrs[self.instr_index].1;
        match self.x86.run(instr) {
            Err(err) => {
                // On errors, back up one instruction so the debugger points at the failed instruction.
                self.x86.regs.eip -= instr.len() as u32;
                Err(err)
            }
            Ok(false) => {
                // Hit breakpoint
                self.x86.regs.eip -= instr.len() as u32;
                Ok(false)
            }
            Ok(true) => {
                unsafe {
                    if let Some(ref msg) = CRASHED {
                        self.x86.regs.eip -= instr.len() as u32;
                        bail!(msg);
                    }
                }
                self.instr_count += 1;
                self.instr_index += 1;
                self.jmp(self.x86.regs.eip);
                Ok(true)
            }
        }
    }

    // Multi-step execution.  Returns Ok(false) on breakpoint.
    pub fn step_many(&mut self, count: usize) -> anyhow::Result<bool> {
        for _ in 0..count {
            if !self.step()? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
