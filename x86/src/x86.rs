//! The central x86 machine object.

use iced_x86::FlowControl;

use crate::{
    memory::Memory,
    ops,
    registers::{Flags, Registers},
    StepError, StepResult,
};
use std::collections::{BTreeMap, HashMap};

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
impl Default for BasicBlock {
    fn default() -> Self {
        Self {
            len: Default::default(),
            instrs: Default::default(),
        }
    }
}
impl BasicBlock {
    fn disassemble(buf: &[u8], ip: u32, single_step: bool) -> Self {
        let mut instrs = Vec::new();
        let mut decoder =
            iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
        while decoder.can_decode() {
            let instr = decoder.decode();
            instrs.push(instr);
            if instr.flow_control() != FlowControl::Next || single_step {
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
pub struct InstrCache {
    pub blocks: BTreeMap<u32, BasicBlock>,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the bytes from before the breakpoint.
    breakpoints: HashMap<u32, u8>,
}

impl InstrCache {
    pub fn new() -> Self {
        InstrCache {
            blocks: BTreeMap::new(),
            breakpoints: HashMap::new(),
        }
    }

    /// Remove any BasicBlock that covers ip.
    /// Returns the ip after the end of the block, if any.
    fn kill_block(&mut self, ip: u32) -> Option<u32> {
        if let Some((&prev_ip, block)) = self.blocks.range(0..ip + 1).last() {
            let end = prev_ip + block.len;
            if end > ip {
                // log::info!("killed block {:x}..{:x}", prev_ip, end);
                self.blocks.remove(&prev_ip);
                return Some(end);
            }
        }
        None
    }

    fn update_block(&mut self, mem: &[u8], ip: u32, single_step: bool) {
        // If there's a block after this location, ensure we don't disassemble over it.
        let end = if let Some((&later_ip, _)) = self.blocks.range(ip + 1..).next() {
            later_ip as usize
        } else {
            mem.len()
        };

        // Ensure we don't overlap any previous block.
        self.kill_block(ip);

        let block = BasicBlock::disassemble(&mem[ip as usize..end], ip, single_step);
        // log::info!("added block {:x}..{:x}", ip, ip + block.len);
        self.blocks.insert(ip, block);
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, mem: &mut [u8], addr: u32) {
        self.kill_block(addr); // Allow recreating lazily.
        self.breakpoints.insert(addr, mem[addr as usize]);
        mem[addr as usize] = 0xcc; // int3
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, mem: &mut [u8], addr: u32) {
        self.kill_block(addr); // Allow recreating lazily.

        // Allow a subsequent block that might have been split due to the int3
        // to be recreated.
        self.kill_block(addr + 1);

        let prev = self.breakpoints.remove(&addr).unwrap();
        mem[addr as usize] = prev;
    }

    /// Executes the current basic block, updating eip.
    /// Returns the number of instructions executed.
    pub fn execute_block(&mut self, x86: &mut X86) -> StepResult<usize> {
        let mut ip = x86.regs.eip;
        let block = match self.blocks.get(&ip) {
            Some(b) => b,
            None => {
                self.update_block(&x86.mem, ip, false);
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

    /// Change cache such that there's a single basic block at ip.
    /// This means executing the next execute_block() will execute a single instruction.
    pub fn make_single_step(&mut self, x86: &mut X86, ip: u32) {
        match self.blocks.get(&ip) {
            Some(b) if b.instrs.len() == 1 => {}
            _ => {
                self.update_block(&x86.mem, ip, true);
            }
        };
    }

    /// Undo the effects of make_single_step().
    pub fn clear_single_step(&mut self, ip: u32) {
        if let Some(end) = self.kill_block(ip) {
            self.kill_block(end);
        }
    }
}
