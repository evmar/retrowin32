//! The central x86 machine object.

use iced_x86::FlowControl;

use crate::{
    memory::Memory,
    ops,
    registers::{Flags, Registers},
    StepError, StepResult,
};
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

/// Addresses from 0 up to this point cause panics if we access them.
/// This helps catch implementation bugs earlier.
pub const NULL_POINTER_REGION_SIZE: u32 = 0x1000;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct X86 {
    #[serde(with = "serde_bytes")]
    pub mem: Vec<u8>,
    pub regs: Registers,
    // Flags are in principle a register but we hold it outside of regs for lifetime reasons,
    // because there are operations we want to do over mut regs and flags at the same time.
    pub flags: Flags,
    /// Toggled on by breakpoints/process exit.
    // TODO: this is gross, because we must check it after every instruction.
    // It would be nice if there was some more clever way to thread process exit...
    stopped: bool,
    crashed: Option<String>,
}
impl X86 {
    pub fn new() -> Self {
        unsafe {
            ops::init_op_tab();
        }
        let mut regs = Registers::new();
        regs.eax = 0xdeadbeea;
        regs.ebx = 0xdeadbeeb;
        regs.ecx = 0xdeadbeec;
        regs.edx = 0xdeadbeed;
        regs.esi = 0xdeadbe51;
        regs.edi = 0xdeadbed1;
        X86 {
            mem: Vec::new(),
            regs,
            flags: Flags::empty(),
            stopped: false,
            crashed: None,
        }
    }

    fn crash(&mut self, msg: String) {
        self.crashed = Some(msg);
        self.stopped = true;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    /// Check whether reading a T from mem[addr] would cause OOB, and crash() if so.
    fn check_oob<T>(&mut self, addr: u32) -> bool {
        if addr < NULL_POINTER_REGION_SIZE {
            self.crash(format!("crash: null pointer at {addr:#x}"));
            return true;
        }
        if addr as usize + std::mem::size_of::<T>() > self.mem.len() {
            self.crash(format!("crash: oob pointer at {addr:#x}"));
            return true;
        }
        false
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) {
        if self.check_oob::<u32>(addr) {
            return;
        }
        self.mem.write_u32(addr, value);
    }
    pub fn write_u16(&mut self, addr: u32, value: u16) {
        if self.check_oob::<u16>(addr) {
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
        if self.check_oob::<u8>(addr) {
            return;
        }
        // Safety: check_oob checked bounds.
        unsafe { *self.mem.get_unchecked_mut(addr as usize) = value }
    }

    pub fn read_u32(&mut self, addr: u32) -> u32 {
        if self.check_oob::<u32>(addr) {
            return 0;
        }
        self.mem.read_u32(addr)
    }
    pub fn read_u16(&mut self, addr: u32) -> u16 {
        if self.check_oob::<u16>(addr) {
            return 0;
        }
        let offset = addr as usize;
        // Safety: check_oob checked bounds.
        unsafe {
            (*self.mem.get_unchecked(offset) as u16) << 0
                | (*self.mem.get_unchecked(offset + 1) as u16) << 8
        }
    }
    pub fn read_u8(&mut self, addr: u32) -> u8 {
        if self.check_oob::<u8>(addr) {
            return 0;
        }
        // Safety: check_oob checked bounds.
        unsafe { *self.mem.get_unchecked(addr as usize) }
    }

    /// Executes an instruction, leaving eip alone.
    pub fn run(&mut self, instr: &iced_x86::Instruction) -> StepResult<()> {
        ops::execute(self, instr)?; // Note: may ::Error or ::Interrupt here...
        if self.stopped {
            // ...but we also might set self.stopped instead in some scenarios.
            self.stopped = false;
            if let Some(crash) = self.crashed.take() {
                return Err(StepError::Error(crash));
            }
            return Err(StepError::Interrupt);
        }
        Ok(())
    }
}

pub struct BasicBlock {
    pub len: u32,
    pub instrs: Vec<iced_x86::Instruction>,
}

impl BasicBlock {
    fn disassemble(buf: &[u8], ip: u32) -> Self {
        let mut instrs = Vec::new();
        let mut decoder =
            iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
        while decoder.can_decode() {
            let instr = decoder.decode();
            instrs.push(instr);
            if instr.flow_control() != FlowControl::Next {
                break;
            }
        }
        BasicBlock {
            instrs,
            len: decoder.ip() as u32 - ip,
        }
    }
}

/// Cache of decoded instructions.
/// This also caches the current instruction index, so that we don't need to map
/// x86 eip addresses to the instruction cache entry.  Instead, whenever we step
/// we update index as appropriate.
pub struct InstrCache {
    pub blocks: BTreeMap<u32, Rc<BasicBlock>>,

    /// Span of addresses that refer to code, for error checking
    pub span: std::ops::Range<u32>,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the instruction from before the breakpoint.
    breakpoints: HashMap<u32, iced_x86::Instruction>,
}

impl InstrCache {
    pub fn new() -> Self {
        InstrCache {
            blocks: BTreeMap::new(),
            span: 0..0,
            breakpoints: HashMap::new(),
        }
    }

    fn update_block(&mut self, mem: &[u8], ip: u32) -> Rc<BasicBlock> {
        // If there's a block after this location, ensure we don't disassemble over it.
        let end = if let Some((&later_ip, _)) = self.blocks.range(ip + 1..).next() {
            later_ip as usize
        } else {
            mem.len()
        };

        // If there's a block before this location, ensure we don't overlap it.
        if let Some((&prev_ip, block)) = self.blocks.range(0..ip).last() {
            if prev_ip + block.len > ip {
                self.blocks.remove(&prev_ip); // We'll recreate it when we next need it.
            }
        }

        let block = Rc::new(BasicBlock::disassemble(&mem[ip as usize..end], ip));
        self.blocks.insert(ip, block.clone());
        block
    }

    /// Replace the instruction found at a given ip, returning the previous instruction.
    fn patch(&mut self, _addr: u32, _instr: iced_x86::Instruction) -> iced_x86::Instruction {
        todo!();
        // let index = self.ip_to_instr_index(&[], addr).expect("couldn't map ip");
        // std::mem::replace(&mut self.instrs[index].1, instr)
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) {
        let mut int3 = iced_x86::Instruction::with(iced_x86::Code::Int3);
        // The instruction needs a length/next_ip so the execution machinery doesn't lose its location.
        int3.set_len(1);
        int3.set_next_ip(addr as u64 + 1);
        let prev = self.patch(addr, int3);
        self.breakpoints.insert(addr, prev);
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) {
        let prev = self.breakpoints.remove(&addr).unwrap();
        self.patch(addr, prev);
    }

    /// Executes the current basic block, updating eip.
    /// Returns the number of instructions executed.
    pub fn step(&mut self, x86: &mut X86) -> StepResult<usize> {
        let mut ip = x86.regs.eip;
        let block = match self.blocks.get(&ip) {
            Some(b) => b,
            None => {
                self.update_block(&x86.mem, ip);
                self.blocks.get(&ip).unwrap()
            }
        };
        for instr in block.instrs.iter() {
            x86.regs.eip = instr.next_ip() as u32;
            match x86.run(instr) {
                Err(err) => {
                    // Point the debugger at the failed instruction.
                    x86.regs.eip = ip;
                    return Err(err);
                }
                Ok(_) => {}
            }
            ip = x86.regs.eip;
        }
        Ok(block.instrs.len())
    }
}
