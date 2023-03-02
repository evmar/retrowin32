//! The central x86 machine object.

use crate::{
    memory::Memory,
    ops,
    registers::{Flags, Registers},
    StepError, StepResult,
};
use std::collections::HashMap;

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

/// Cache of decoded instructions.
/// This also caches the current instruction index, so that we don't need to map
/// x86 eip addresses to the instruction cache entry.  Instead, whenever we step
/// we update index as appropriate.
pub struct InstrCache {
    /// (ip, instruction) pairs of cached decoded instructions.
    pub instrs: Vec<(u32, iced_x86::Instruction)>,
    /// Current position within instrs.
    pub index: usize,
    /// Span of addresses that refer to code, for error checking
    pub span: std::ops::Range<u32>,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the instruction from before the breakpoint.
    breakpoints: HashMap<u32, iced_x86::Instruction>,
}

impl InstrCache {
    pub fn new() -> Self {
        InstrCache {
            instrs: Vec::new(),
            index: 0,
            span: 0..0,
            breakpoints: HashMap::new(),
        }
    }

    pub fn disassemble(&mut self, buf: &[u8], ip: u32) {
        self.instrs.clear();
        let mut decoder =
            iced_x86::Decoder::with_ip(32, buf, ip as u64, iced_x86::DecoderOptions::NONE);
        while decoder.can_decode() {
            self.instrs.push((decoder.ip() as u32, decoder.decode()));
        }
        self.span = ip..decoder.ip() as u32;
    }

    /// Given an IP that wasn't found in the decoded instructions, re-decode starting at that
    /// address and patch in the new instructions in place of the previous ones.
    /// start is the index of where in the instruction table the patch will begin.
    fn repatch(&mut self, mem: &[u8], addr: u32, start: usize) {
        let mut decoder = iced_x86::Decoder::with_ip(
            32,
            &mem[addr as usize..],
            addr as u64,
            iced_x86::DecoderOptions::NONE,
        );
        let mut end = start;
        let mut patch = Vec::new();
        while decoder.can_decode() {
            let new_entry = (decoder.ip() as u32, decoder.decode());
            // Overwrite any instructions with conflicting ips.
            while new_entry.0 > self.instrs[end].0 {
                end += 1;
            }
            if new_entry == self.instrs[end] {
                // Resynchronized.
                // Note: this compares both ip and the decoded instruction, to handle the case
                // where the IPs line up but the underlying instructions still don't match due
                // to a change in memory.  This is an incorrect fix to the more general problem
                // of keeping this cache up to date with writes to memory.
                break;
            }
            patch.push(new_entry);
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

    fn ip_to_instr_index(&mut self, mem: &[u8], target_ip: u32) -> Option<usize> {
        match self.instrs.binary_search_by_key(&target_ip, |&(ip, _)| ip) {
            Ok(pos) => Some(pos),
            Err(pos) => {
                // We may hit this case if the disassembler gets desynchronized from the instruction
                // stream.  We need to re-disassemble or something in that case.
                let nearby = match self.instrs.get(pos) {
                    Some((ip, instr)) => format!("nearby address {:x} {}", ip, instr),
                    None => format!("address beyond decoded range"),
                };
                log::error!("invalid ip {:x}, desync? {}", target_ip, nearby);
                self.repatch(mem, target_ip, pos);
                self.ip_to_instr_index(mem, target_ip)
            }
        }
    }

    pub fn jmp(&mut self, mem: &[u8], target_ip: u32) -> StepResult<()> {
        if !self.span.contains(&target_ip) {
            return Err(StepError::Error(format!(
                "jmp to {target_ip:x} outside of code segment {:x?}",
                self.span
            )));
        }
        self.index = self.ip_to_instr_index(mem, target_ip).unwrap();
        Ok(())
    }

    /// Replace the instruction found at a given ip, returning the previous instruction.
    fn patch(&mut self, addr: u32, instr: iced_x86::Instruction) -> iced_x86::Instruction {
        let index = self.ip_to_instr_index(&[], addr).expect("couldn't map ip");
        std::mem::replace(&mut self.instrs[index].1, instr)
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

    /// Executes the current instruction, updating eip.
    /// Returns Ok(false) if we jumped, Ok(true) if we single-stepped.
    /// Caller must call self.jmp() in the jump case.
    pub fn step(&mut self, x86: &mut X86) -> StepResult<bool> {
        let (prev_ip, ref instr) = self.instrs[self.index];
        let next_ip = instr.next_ip() as u32;
        // Need to update eip before executing because instructions like 'call' will push eip onto the stack.
        x86.regs.eip = next_ip;
        match x86.run(instr) {
            Err(err) => {
                // Point the debugger at the failed instruction.
                x86.regs.eip = prev_ip;
                Err(err)
            }
            Ok(_) => {
                if x86.regs.eip == next_ip {
                    self.index += 1;
                    Ok(true)
                } else {
                    // if !(0x401000..0x40a600).contains(&x86.regs.eip) && x86.regs.eip < 0xf1a7_000 {
                    //     x86.regs.eip = prev_ip;
                    //     return Err(StepError::Error(format!("bad jmp at {:x}", prev_ip)));
                    // }
                    Ok(false)
                }
            }
        }
    }
}
