//! For performance, we cache decoded CPU instructions in basic blocks,
//! so when executing we always know we'll execute a full basic block in
//! linear order before making any jumps.
//!
//! Because of this, we cannot represent a CPU state that is partway through
//! a basic block.  To implement single-stepping and breakpoints, we break
//! any affected basic block into smaller pieces to maintain the invariant of
//! always executing through a basic block's end.

use std::collections::{BTreeMap, HashMap};

use crate::{Mem, Memory, StepResult, CPU};

struct BasicBlock {
    /// Number of x86 instruction bytes covered by this block.
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
    fn disassemble(buf: &Mem, ip: u32, single_step: bool) -> Self {
        let mut instrs = Vec::new();
        let mut decoder = iced_x86::Decoder::with_ip(
            32,
            buf.as_slice_todo(),
            ip as u64,
            iced_x86::DecoderOptions::NONE,
        );
        while decoder.can_decode() {
            let instr = decoder.decode();
            instrs.push(instr);
            if instr.flow_control() != iced_x86::FlowControl::Next || single_step {
                break;
            }
        }
        BasicBlock {
            instrs,
            len: decoder.ip() as u32 - ip,
        }
    }
}

/// Cache of decoded instructions as basic blocks.
pub struct InstrCache {
    /// Initial IP => block of decoded instructions.
    blocks: BTreeMap<u32, BasicBlock>,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the bytes from before the breakpoint.
    breakpoints: HashMap<u32, u8>,
}

impl Default for InstrCache {
    fn default() -> Self {
        InstrCache {
            blocks: BTreeMap::new(),
            breakpoints: HashMap::new(),
        }
    }
}

impl InstrCache {
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

    fn update_block(&mut self, mem: &Mem, ip: u32, single_step: bool) {
        // If there's a block after this location, ensure we don't disassemble over it.
        let end = if let Some((&later_ip, _)) = self.blocks.range(ip + 1..).next() {
            later_ip
        } else {
            mem.len() as u32
        };

        // Ensure we don't overlap any previous block.
        self.kill_block(ip);

        let block = BasicBlock::disassemble(&mem.slice(ip..end), ip, single_step);
        // log::info!("added block {:x}..{:x}", ip, ip + block.len);
        // if block.len == 1 {
        //     log::info!(
        //         "{}",
        //         block
        //             .instrs
        //             .iter()
        //             .map(|i| i.to_string())
        //             .collect::<Vec<_>>()
        //             .join("; ")
        //     );
        // }
        self.blocks.insert(ip, block);
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, mem: &mut Mem, addr: u32) {
        self.kill_block(addr); // Allow recreating lazily.
        self.breakpoints.insert(addr, *mem.view::<u8>(addr));
        *mem.view_mut::<u8>(addr) = 0xcc; // int3
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, mem: &mut Mem, addr: u32) {
        self.kill_block(addr); // Allow recreating lazily.

        // Allow a subsequent block that might have been split due to the int3
        // to be recreated.
        self.kill_block(addr + 1);

        let prev = self.breakpoints.remove(&addr).unwrap();
        *mem.view_mut::<u8>(addr) = prev;
    }

    /// Executes the current basic block, updating eip.
    /// Returns the number of instructions executed.
    pub fn execute_block(&mut self, cpu: &mut CPU, mem: &mut Mem) -> StepResult<usize> {
        let mut ip = cpu.regs.eip;
        let block = match self.blocks.get(&ip) {
            Some(b) => b,
            None => {
                self.update_block(mem, ip, false);
                self.blocks.get(&ip).unwrap()
            }
        };
        for instr in block.instrs.iter() {
            cpu.regs.eip = instr.next_ip() as u32;
            match cpu.run(mem, instr) {
                Err(err) => {
                    // Point the debugger at the failed instruction.
                    cpu.regs.eip = ip;
                    return Err(err);
                }
                Ok(_) => {}
            }
            ip = cpu.regs.eip;
        }
        Ok(block.instrs.len())
    }

    /// Change cache such that there's a single basic block at ip.
    /// This means executing the next execute_block() will execute a single instruction.
    pub fn make_single_step(&mut self, mem: &Mem, ip: u32) {
        match self.blocks.get(&ip) {
            Some(b) if b.instrs.len() == 1 => {}
            _ => {
                self.update_block(mem, ip, true);
            }
        };
    }
}
