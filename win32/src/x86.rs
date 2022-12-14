use std::collections::HashMap;

use anyhow::bail;
use serde::ser::SerializeStruct;

use crate::{
    host, memory::Memory, ops, pe::ImageSectionFlags, registers::Registers, winapi,
    windows::load_exe,
};

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
    /// Toggled on by breakpoints/process exit.
    pub stopped: bool,
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
            stopped: false,
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

    pub fn push(&mut self, value: u32) {
        self.regs.esp -= 4;
        self.write_u32(self.regs.esp, value);
    }

    pub fn push16(&mut self, value: u16) {
        self.regs.esp -= 2;
        self.write_u16(self.regs.esp, value);
    }

    pub fn pop(&mut self) -> u32 {
        let value = self.read_u32(self.regs.esp);
        self.regs.esp += 4;
        value
    }

    pub fn pop16(&mut self) -> u16 {
        let value = self.read_u16(self.regs.esp);
        self.regs.esp += 2;
        value
    }

    /// Compute the address found in instructions that reference memory, e.g.
    ///   mov [eax+03h],...
    pub fn addr(&self, instr: &iced_x86::Instruction) -> u32 {
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
            self.regs
                .get32(instr.memory_index())
                .wrapping_mul(instr.memory_index_scale())
        } else {
            0
        };
        // In general these operations aren't written to wrap, but in some cases
        // the components are negative which is implemented in two's complement by
        // a wrapping add.
        seg.wrapping_add(base)
            .wrapping_add(index)
            .wrapping_add(instr.memory_displacement32())
    }

    pub fn op0_rm32(&self, instr: &iced_x86::Instruction) -> u32 {
        match instr.op0_kind() {
            iced_x86::OpKind::Register => self.regs.get32(instr.op0_register()),
            iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    pub fn op0_rm16(&self, instr: &iced_x86::Instruction) -> u16 {
        match instr.op0_kind() {
            iced_x86::OpKind::Register => self.regs.get16(instr.op0_register()),
            iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    pub fn op0_rm8(&self, instr: &iced_x86::Instruction) -> u8 {
        match instr.op0_kind() {
            iced_x86::OpKind::Register => self.regs.get8(instr.op0_register()),
            iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    pub fn op1_rm32(&self, instr: &iced_x86::Instruction) -> u32 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get32(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u32(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    pub fn op1_rm16(&self, instr: &iced_x86::Instruction) -> u16 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get16(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u16(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    pub fn op1_rm8(&self, instr: &iced_x86::Instruction) -> u8 {
        match instr.op1_kind() {
            iced_x86::OpKind::Register => self.regs.get8(instr.op1_register()),
            iced_x86::OpKind::Memory => self.read_u8(self.addr(instr)),
            _ => unreachable!(),
        }
    }

    pub fn rm32_x(&mut self, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u32) -> u32) {
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

    pub fn rm16_x(&mut self, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u16) -> u16) {
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

    pub fn rm8_x(&mut self, instr: &iced_x86::Instruction, op: impl FnOnce(&mut X86, u8) -> u8) {
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

    pub fn jmp(&mut self, addr: u32) -> anyhow::Result<()> {
        if addr < 0x1000 {
            bail!("jmp to null page");
        }

        if addr & 0xFFFF_0000 == SHIM_BASE {
            let ret = self.pop();
            let eip = self.regs.eip;
            let handler = self
                .shims
                .get(addr)
                .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
            handler(self);
            if self.regs.eip != eip {
                return Ok(()); // handler set eip.
            }
            return self.jmp(ret);
        }

        self.regs.eip = addr;
        Ok(())
    }

    fn run(&mut self, instr: &iced_x86::Instruction) -> anyhow::Result<()> {
        self.regs.eip = instr.next_ip() as u32;
        match instr.code() {
            iced_x86::Code::Enterd_imm16_imm8 => ops::enterd_imm16_imm8(self, instr),
            iced_x86::Code::Leaved => ops::leaved(self, instr),

            iced_x86::Code::Call_rel32_32 => ops::call(self, instr)?,
            iced_x86::Code::Call_rm32 => ops::call_rm32(self, instr)?,
            iced_x86::Code::Retnd => ops::retnd(self, instr)?,
            iced_x86::Code::Retnd_imm16 => ops::retnd_imm16(self, instr)?,
            iced_x86::Code::Jmp_rel32_32 | iced_x86::Code::Jmp_rel8_32 => ops::jmp(self, instr)?,
            iced_x86::Code::Jmp_rm32 => ops::jmp_rm32(self, instr)?,
            iced_x86::Code::Ja_rel32_32 | iced_x86::Code::Ja_rel8_32 => ops::ja(self, instr)?,
            iced_x86::Code::Jae_rel32_32 | iced_x86::Code::Jae_rel8_32 => ops::jae(self, instr)?,
            iced_x86::Code::Jb_rel32_32 | iced_x86::Code::Jb_rel8_32 => ops::jb(self, instr)?,
            iced_x86::Code::Jbe_rel32_32 | iced_x86::Code::Jbe_rel8_32 => ops::jbe(self, instr)?,
            iced_x86::Code::Je_rel32_32 | iced_x86::Code::Je_rel8_32 => ops::je(self, instr)?,
            iced_x86::Code::Jecxz_rel8_32 => ops::jecxz(self, instr)?,
            iced_x86::Code::Jne_rel32_32 | iced_x86::Code::Jne_rel8_32 => ops::jne(self, instr)?,
            iced_x86::Code::Jns_rel32_32 | iced_x86::Code::Jns_rel8_32 => ops::jns(self, instr)?,
            iced_x86::Code::Jg_rel32_32 | iced_x86::Code::Jg_rel8_32 => ops::jg(self, instr)?,
            iced_x86::Code::Jge_rel32_32 | iced_x86::Code::Jge_rel8_32 => ops::jge(self, instr)?,
            iced_x86::Code::Jle_rel32_32 | iced_x86::Code::Jle_rel8_32 => ops::jle(self, instr)?,
            iced_x86::Code::Jl_rel32_32 | iced_x86::Code::Jl_rel8_32 => ops::jl(self, instr)?,
            iced_x86::Code::Js_rel32_32 | iced_x86::Code::Js_rel8_32 => ops::js(self, instr)?,

            iced_x86::Code::Pushd_imm8 => ops::pushd_imm8(self, instr),
            iced_x86::Code::Pushd_imm32 => ops::pushd_imm32(self, instr),
            iced_x86::Code::Push_r32 => ops::push_r32(self, instr),
            iced_x86::Code::Push_rm32 => ops::push_rm32(self, instr),
            iced_x86::Code::Push_rm16 => ops::push_rm16(self, instr),

            iced_x86::Code::Pop_r32 | iced_x86::Code::Pop_rm32 => ops::pop_rm32(self, instr),
            iced_x86::Code::Pop_r16 | iced_x86::Code::Pop_rm16 => ops::pop_rm16(self, instr),

            iced_x86::Code::Mov_rm32_imm32 => ops::mov_rm32_imm32(self, instr),
            iced_x86::Code::Mov_r32_imm32 => ops::mov_r32_imm32(self, instr),
            iced_x86::Code::Mov_moffs32_EAX => ops::mov_moffs32_eax(self, instr),
            iced_x86::Code::Mov_EAX_moffs32 => ops::mov_eax_moffs32(self, instr),
            iced_x86::Code::Mov_rm32_r32 => ops::mov_rm32_r32(self, instr),
            iced_x86::Code::Mov_r32_rm32 => ops::mov_r32_rm32(self, instr),
            iced_x86::Code::Mov_r16_rm16 => ops::mov_r16_rm16(self, instr),
            iced_x86::Code::Mov_rm16_r16 => ops::mov_rm16_r16(self, instr),
            iced_x86::Code::Mov_r8_rm8 => ops::mov_r8_rm8(self, instr),
            iced_x86::Code::Mov_rm8_r8 => ops::mov_rm8_r8(self, instr),
            iced_x86::Code::Mov_r8_imm8 | iced_x86::Code::Mov_rm8_imm8 => {
                ops::mov_rm8_imm8(self, instr)
            }

            iced_x86::Code::Movsx_r32_rm16 => ops::movsx_r32_rm16(self, instr),
            iced_x86::Code::Movsx_r32_rm8 => ops::movsx_r32_rm8(self, instr),
            iced_x86::Code::Movsx_r16_rm8 => ops::movsx_r16_rm8(self, instr),

            iced_x86::Code::Movzx_r32_rm16 => ops::movzx_r32_rm16(self, instr),
            iced_x86::Code::Movzx_r32_rm8 => ops::movzx_r32_rm8(self, instr),
            iced_x86::Code::Movzx_r16_rm8 => ops::movzx_r16_rm8(self, instr),

            iced_x86::Code::Xchg_rm32_r32 | iced_x86::Code::Xchg_r32_EAX => {
                ops::xchg_rm32_r32(self, instr)
            }
            iced_x86::Code::Cmpxchg_rm32_r32 => ops::cmpxchg_rm32_r32(self, instr),

            iced_x86::Code::Cmpsb_m8_m8 => ops::cmps(self, instr)?,
            iced_x86::Code::Movsd_m32_m32 => ops::movs(self, instr, 4)?,
            iced_x86::Code::Movsb_m8_m8 => ops::movs(self, instr, 1)?,
            iced_x86::Code::Scasb_AL_m8 => ops::scas(self, instr)?,
            iced_x86::Code::Stosd_m32_EAX => ops::stosd(self, instr)?,
            iced_x86::Code::Stosb_m8_AL => ops::stosb(self, instr)?,
            iced_x86::Code::Lodsd_EAX_m32 => ops::lods(self, instr)?,

            iced_x86::Code::And_rm32_imm32 => ops::and_rm32_imm32(self, instr),
            iced_x86::Code::And_EAX_imm32 => ops::and_rm32_imm32(self, instr),
            iced_x86::Code::And_rm32_imm8 => ops::and_rm32_imm8(self, instr),
            iced_x86::Code::And_rm32_r32 => ops::and_rm32_r32(self, instr),
            iced_x86::Code::And_r32_rm32 => ops::and_r32_rm32(self, instr),
            iced_x86::Code::And_rm16_imm16 => ops::and_rm16_imm16(self, instr),
            iced_x86::Code::And_rm8_imm8 => ops::and_rm8_imm8(self, instr),
            iced_x86::Code::And_AL_imm8 => ops::and_rm8_imm8(self, instr),
            iced_x86::Code::Or_rm32_r32 | iced_x86::Code::Or_r32_rm32 => {
                ops::or_rm32_rm32(self, instr)
            }
            iced_x86::Code::Or_rm32_imm32 | iced_x86::Code::Or_EAX_imm32 => {
                ops::or_rm32_imm32(self, instr)
            }
            iced_x86::Code::Or_rm32_imm8 => ops::or_rm32_imm8(self, instr),
            iced_x86::Code::Or_rm16_imm16 => ops::or_rm16_imm16(self, instr),
            iced_x86::Code::Or_rm8_imm8 => ops::or_rm8_imm8(self, instr),
            iced_x86::Code::Shl_rm32_imm8 => ops::shl_rm32_imm8(self, instr),
            iced_x86::Code::Shl_rm32_1 => ops::shl_rm32_imm8(self, instr),
            iced_x86::Code::Shl_rm32_CL => ops::shl_rm32_cl(self, instr),
            iced_x86::Code::Shl_rm8_CL => ops::shl_rm8_cl(self, instr),
            iced_x86::Code::Shl_rm8_imm8 => ops::shl_rm8_imm8(self, instr),
            iced_x86::Code::Shr_rm32_CL => ops::shr_rm32_cl(self, instr),
            iced_x86::Code::Shr_rm32_1 => ops::shr_rm32_1(self, instr),
            iced_x86::Code::Shr_rm32_imm8 => ops::shr_rm32_imm8(self, instr),
            iced_x86::Code::Sar_rm32_imm8 => ops::sar_rm32_imm8(self, instr),
            iced_x86::Code::Sar_rm32_1 => ops::sar_rm32_imm8(self, instr),
            iced_x86::Code::Sar_rm32_CL => ops::sar_rm32_cl(self, instr),
            iced_x86::Code::Ror_rm32_CL => ops::ror_rm32_cl(self, instr),
            iced_x86::Code::Xor_rm32_r32 => ops::xor_rm32_rm32(self, instr),
            iced_x86::Code::Xor_r32_rm32 => ops::xor_rm32_rm32(self, instr),
            iced_x86::Code::Xor_rm32_imm32 | iced_x86::Code::Xor_EAX_imm32 => {
                ops::xor_rm32_imm32(self, instr)
            }
            iced_x86::Code::Xor_rm32_imm8 => ops::xor_rm32_imm8(self, instr),
            iced_x86::Code::Xor_rm8_imm8 => ops::xor_rm8_imm8(self, instr),
            iced_x86::Code::Xor_r8_rm8 => ops::xor_r8_rm8(self, instr),
            iced_x86::Code::Add_r32_rm32 => ops::add_r32_rm32(self, instr),
            iced_x86::Code::Add_rm32_r32 => ops::add_rm32_r32(self, instr),
            iced_x86::Code::Add_rm32_imm32 => ops::add_rm32_imm32(self, instr),
            iced_x86::Code::Add_EAX_imm32 => ops::add_rm32_imm32(self, instr),
            iced_x86::Code::Add_rm32_imm8 => ops::add_rm32_imm8(self, instr),
            iced_x86::Code::Add_rm16_imm8 => ops::add_rm16_imm8(self, instr),
            iced_x86::Code::Add_rm8_imm8 => ops::add_rm8_imm8(self, instr),
            iced_x86::Code::Add_r8_rm8 => ops::add_r8_rm8(self, instr),
            iced_x86::Code::Sub_rm32_imm8 => ops::sub_rm32_imm8(self, instr),
            iced_x86::Code::Sub_EAX_imm32 => ops::sub_rm32_imm32(self, instr),
            iced_x86::Code::Sub_rm32_imm32 => ops::sub_rm32_imm32(self, instr),
            iced_x86::Code::Sub_rm32_r32 => ops::sub_rm32_r32(self, instr),
            iced_x86::Code::Sub_r32_rm32 => ops::sub_r32_rm32(self, instr),
            iced_x86::Code::Sub_rm8_imm8 => ops::sub_rm8_imm8(self, instr),
            iced_x86::Code::Sbb_r32_rm32 => ops::sbb_r32_rm32(self, instr),
            iced_x86::Code::Sbb_r8_rm8 => ops::sbb_r8_rm8(self, instr),
            iced_x86::Code::Imul_r32_rm32 => ops::imul_r32_rm32(self, instr),
            iced_x86::Code::Imul_r32_rm32_imm32 => ops::imul_r32_rm32_imm32(self, instr),
            iced_x86::Code::Imul_r32_rm32_imm8 => ops::imul_r32_rm32_imm8(self, instr),
            iced_x86::Code::Idiv_rm32 => ops::idiv_rm32(self, instr),
            iced_x86::Code::Div_rm32 => ops::div_rm32(self, instr),
            iced_x86::Code::Dec_r32 | iced_x86::Code::Dec_rm32 => ops::dec_rm32(self, instr),
            iced_x86::Code::Inc_r32 | iced_x86::Code::Inc_rm32 => ops::inc_rm32(self, instr),
            iced_x86::Code::Inc_rm8 => ops::inc_rm8(self, instr),
            iced_x86::Code::Neg_rm32 => ops::neg_rm32(self, instr),
            iced_x86::Code::Not_rm32 => ops::not_rm32(self, instr),

            iced_x86::Code::Lea_r32_m => ops::lea_r32_m(self, instr),

            iced_x86::Code::Cmp_rm32_r32 => ops::cmp_rm32_r32(self, instr),
            iced_x86::Code::Cmp_r32_rm32 => ops::cmp_r32_rm32(self, instr),
            iced_x86::Code::Cmp_EAX_imm32 | iced_x86::Code::Cmp_rm32_imm32 => {
                ops::cmp_rm32_imm32(self, instr)
            }
            iced_x86::Code::Cmp_rm32_imm8 => ops::cmp_rm32_imm8(self, instr),
            iced_x86::Code::Cmp_rm16_r16 | iced_x86::Code::Cmp_r16_rm16 => {
                ops::cmp_rm16_rm16(self, instr)
            }
            iced_x86::Code::Cmp_rm16_imm16 => ops::cmp_rm16_imm16(self, instr),
            iced_x86::Code::Cmp_rm16_imm8 => ops::cmp_rm16_imm8(self, instr),
            iced_x86::Code::Cmp_rm8_imm8 | iced_x86::Code::Cmp_AL_imm8 => {
                ops::cmp_rm8_imm8(self, instr)
            }
            iced_x86::Code::Cmp_rm8_r8 => ops::cmp_rm8_r8(self, instr),
            iced_x86::Code::Cmp_r8_rm8 => ops::cmp_r8_rm8(self, instr),
            iced_x86::Code::Test_rm32_r32 => ops::test_rm32_r32(self, instr),
            iced_x86::Code::Test_rm32_imm32 | iced_x86::Code::Test_EAX_imm32 => {
                ops::test_rm32_imm32(self, instr)
            }
            iced_x86::Code::Test_rm16_r16 => ops::test_rm16_r16(self, instr),
            iced_x86::Code::Test_rm8_r8 => ops::test_rm8_r8(self, instr),
            iced_x86::Code::Test_rm8_imm8 | iced_x86::Code::Test_AL_imm8 => {
                ops::test_rm8_imm8(self, instr)
            }
            iced_x86::Code::Bt_rm32_imm8 => ops::bt_rm32_imm8(self, instr),

            iced_x86::Code::Sete_rm8 => ops::sete_rm8(self, instr),
            iced_x86::Code::Setne_rm8 => ops::setne_rm8(self, instr),
            iced_x86::Code::Setge_rm8 => ops::setge_rm8(self, instr),

            iced_x86::Code::Fld1 => ops::fld1(self, instr),
            iced_x86::Code::Fldz => ops::fldz(self, instr),
            iced_x86::Code::Fld_m64fp => ops::fld_m64fp(self, instr),
            iced_x86::Code::Fld_m32fp => ops::fld_m32fp(self, instr),
            iced_x86::Code::Fild_m32int => ops::fild_m32int(self, instr),
            iced_x86::Code::Fild_m16int => ops::fild_m16int(self, instr),
            iced_x86::Code::Fst_m64fp => ops::fst_m64fp(self, instr),
            iced_x86::Code::Fstp_m64fp => ops::fstp_m64fp(self, instr),
            iced_x86::Code::Fstp_m32fp => ops::fstp_m32fp(self, instr),
            iced_x86::Code::Fistp_m64int => ops::fistp_m64int(self, instr),
            iced_x86::Code::Fistp_m32int => ops::fistp_m32int(self, instr),
            iced_x86::Code::Fchs => ops::fchs(self, instr),
            iced_x86::Code::Fcos => ops::fcos(self, instr),
            iced_x86::Code::Fsin => ops::fsin(self, instr),
            iced_x86::Code::Fpatan => ops::fpatan(self, instr),
            iced_x86::Code::Fsqrt => ops::fsqrt(self, instr),
            iced_x86::Code::Fadd_m64fp => ops::fadd_m64fp(self, instr),
            iced_x86::Code::Fadd_m32fp => ops::fadd_m32fp(self, instr),
            iced_x86::Code::Faddp_sti_st0 => ops::faddp_sti_st0(self, instr),
            iced_x86::Code::Fsub_m32fp => ops::fsub_m32fp(self, instr),
            iced_x86::Code::Fsubr_m64fp => ops::fsubr_m64fp(self, instr),
            iced_x86::Code::Fsubr_m32fp => ops::fsubr_m32fp(self, instr),
            iced_x86::Code::Fmul_m64fp => ops::fmul_m64fp(self, instr),
            iced_x86::Code::Fmul_m32fp => ops::fmul_m32fp(self, instr),
            iced_x86::Code::Fmul_st0_sti => ops::fmul_sti_sti(self, instr),
            iced_x86::Code::Fmul_sti_st0 => ops::fmul_sti_sti(self, instr),
            iced_x86::Code::Fmulp_sti_st0 => ops::fmulp_sti_st0(self, instr),
            iced_x86::Code::Fdivrp_sti_st0 => ops::fdivrp_sti_st0(self, instr),
            iced_x86::Code::Fdiv_m64fp => ops::fdiv_m64fp(self, instr),
            iced_x86::Code::Fxch_st0_sti => ops::fxch_st0_sti(self, instr),
            iced_x86::Code::Fcomp_m32fp => ops::fcomp_m32fp(self, instr),
            iced_x86::Code::Fcomp_m64fp => ops::fcomp_m64fp(self, instr),
            iced_x86::Code::Fnstsw_AX => ops::fnstsw_ax(self, instr),
            iced_x86::Code::Fnstcw_m2byte => ops::fnstcw_m2byte(self, instr),
            iced_x86::Code::Fldcw_m2byte => ops::fldcw_m2byte(self, instr),
            iced_x86::Code::Fclex | iced_x86::Code::Fnclex | iced_x86::Code::Wait => {
                // ignore
            }

            iced_x86::Code::Pushad => ops::pushad(self, instr),
            iced_x86::Code::Popad => ops::popad(self, instr),
            iced_x86::Code::Pushfd => ops::pushfd(self, instr),
            iced_x86::Code::Pushfw => ops::pushfw(self, instr),
            iced_x86::Code::Popfd => ops::popfd(self, instr),
            iced_x86::Code::Popfw => ops::popfw(self, instr),
            iced_x86::Code::Sahf => ops::sahf(self, instr),

            iced_x86::Code::Std => ops::std(self, instr),
            iced_x86::Code::Cld => ops::cld(self, instr),
            iced_x86::Code::Cwde => ops::cwde(self, instr),
            iced_x86::Code::Cdq => ops::cdq(self, instr),

            iced_x86::Code::Emms
            | iced_x86::Code::Movd_mm_rm32
            | iced_x86::Code::Movd_rm32_mm
            | iced_x86::Code::Packuswb_mm_mmm64
            | iced_x86::Code::Paddusb_mm_mmm64
            | iced_x86::Code::Pmullw_mm_mmm64
            | iced_x86::Code::Psrlw_mm_imm8
            | iced_x86::Code::Punpcklbw_mm_mmm32
            | iced_x86::Code::Psubusb_mm_mmm64
            | iced_x86::Code::Pxor_mm_mmm64 => {
                // TODO: mmx
            }

            iced_x86::Code::Nopd => {}

            iced_x86::Code::Int3 => {
                self.stopped = true;
            }

            code => {
                bail!("unhandled instruction {:?}", code);
            }
        }
        Ok(())
    }
}

impl serde::Serialize for X86 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("X86", 2)?;
        // TODO: serialize remaining state.
        state.serialize_field("mem", serde_bytes::Bytes::new(&self.mem))?;
        state.serialize_field("regs", &self.regs)?;
        state.end()
    }
}

pub struct Snapshot {
    mem: Vec<u8>,
    regs: Registers,
}

impl<'de> serde::Deserialize<'de> for Snapshot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct X86")
            }
            fn visit_seq<V: serde::de::SeqAccess<'de>>(
                self,
                mut seq: V,
            ) -> Result<Snapshot, V::Error> {
                let mem: serde_bytes::ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let regs = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                Ok(Snapshot {
                    mem: mem.into_vec(),
                    regs,
                })
            }
        }
        deserializer.deserialize_struct("X86", &["mem", "regs"], Visitor)
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

    /// Given an IP that wasn't found in the decoded instructions, re-decode starting at that
    /// address and patch in the new instructions in place of the previous ones.
    /// start is the index of where in the instruction table the patch will begin.
    fn repatch(&mut self, addr: u32, start: usize) {
        let mut decoder = iced_x86::Decoder::with_ip(
            32,
            &self.x86.mem[addr as usize..],
            addr as u64,
            iced_x86::DecoderOptions::NONE,
        );
        let mut end = start;
        let mut patch = Vec::new();
        while decoder.can_decode() {
            let ip = decoder.ip() as u32;
            // Overwrite any instructions with conflicting ips.
            while ip > self.instrs[end].0 {
                end += 1;
            }
            if ip == self.instrs[end].0 {
                break;
            }
            patch.push((decoder.ip() as u32, decoder.decode()));
            if end - start > 100 {
                // We haven't hit this, just a defense against this going wrong.
                panic!("resync: oversized patch?");
            }
        }
        log::info!(
            "replacing [{:x}..{:x}] with {} instrs starting at {:x}",
            self.instrs[start].0,
            self.instrs[end].0,
            patch.len(),
            patch[0].0,
        );
        self.instrs.splice(start..end, patch);
    }

    fn ip_to_instr_index(&mut self, target_ip: u32) -> Option<usize> {
        match self.instrs.binary_search_by_key(&target_ip, |&(ip, _)| ip) {
            Ok(pos) => Some(pos),
            Err(pos) => {
                // We may hit this case if the disassembler gets desynchronized from the instruction
                // stream.  We need to re-disassemble or something in that case.
                let nearby = match self.instrs.get(pos) {
                    Some(&(ip, _)) => format!("nearby address {:x}", ip),
                    None => format!("address beyond decoded range"),
                };
                log::error!("invalid ip {:x}, desync? {}", target_ip, nearby);
                self.repatch(target_ip, pos);
                self.ip_to_instr_index(target_ip)
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

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
    ) -> anyhow::Result<HashMap<u32, String>> {
        let labels = load_exe(&mut self.x86, buf, cmdline)?;

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
            .ok_or_else(|| anyhow::anyhow!("couldn't map ip"))?;
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
            .ok_or_else(|| anyhow::anyhow!("couldn't map ip"))?;
        let prev = self.breakpoints.remove(&addr).unwrap();
        self.instrs[index].1 = prev;
        Ok(())
    }

    // Single-step execution.  Returns Ok(false) if we stopped.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        let (ip, ref instr) = self.instrs[self.instr_index];
        match self.x86.run(instr) {
            Err(err) => {
                // Point the debugger at the failed instruction.
                self.x86.regs.eip = ip;
                Err(err)
            }
            Ok(_) => {
                unsafe {
                    if let Some(ref msg) = CRASHED {
                        self.x86.regs.eip = ip;
                        bail!(msg);
                    }
                }
                self.instr_count += 1;
                if self.x86.stopped {
                    self.x86.regs.eip = ip;
                    self.x86.stopped = false;
                    return Ok(false);
                }
                self.instr_index += 1;
                self.jmp(self.x86.regs.eip);
                Ok(true)
            }
        }
    }

    // Multi-step execution.  Returns Ok(false) on breakpoint.
    pub fn step_many(&mut self, count: usize) -> anyhow::Result<usize> {
        for i in 0..count {
            if !self.step()? {
                return Ok(i);
            }
        }
        Ok(count)
    }

    pub fn load_snapshot(&mut self, snap: Snapshot) {
        self.x86.mem = snap.mem;
        self.x86.regs = snap.regs;
        self.jmp(self.x86.regs.eip);
    }
}
