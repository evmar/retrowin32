//! For performance, we cache decoded CPU instructions in basic blocks,
//! so when executing we always know we'll execute a full basic block in
//! linear order before making any jumps.
//!
//! Because of this, we cannot represent a CPU state that is partway through
//! a basic block.  To implement single-stepping and breakpoints, we break
//! any affected basic block into smaller pieces to maintain the invariant of
//! always executing through a basic block's end.

use crate::{x86::CPUState, CPU};
use memory::Mem;
use std::collections::{BTreeMap, HashMap};

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
    fn decode(buf: Mem, ip: u32, single_step: bool) -> Option<Self> {
        let mut instrs = Vec::new();
        let mut decoder = iced_x86::Decoder::with_ip(
            32,
            buf.as_slice_todo(),
            ip as u64,
            iced_x86::DecoderOptions::NONE,
        );
        let mut len = 0;
        while decoder.can_decode() {
            let instr = decoder.decode();
            if instr.code() == iced_x86::Code::INVALID {
                // We can hit invalid instruction when decoding confusing control flows.
                // For example, this UPX code
                // https://github.com/upx/upx/blob/d615985b8a1b68bbdc0f31e0e6e648f93c434095/src/stub/src/i386-win32.pe.S#L136-L142
                // encodes as byte sequence b9 57 48 f2 ae, which is
                //   mov ecx,0xaef24857
                // but if you jmp into one byte within it, it's instead
                //   push edi
                //   dec eax
                //   repne scasb
                if len > 0 {
                    // By truncating this BasicBlock at the invalid instruction, we give the surrounding
                    // logic a chance to generate smaller basic blocks that will be able to toggle between
                    // the two interpretations.
                    break;
                } else {
                    // Otherwise the caller must repair this.
                    return None;
                }
            }
            instrs.push(instr);
            len += instr.len() as u32;
            if instr.flow_control() != iced_x86::FlowControl::Next || single_step {
                break;
            }
        }
        Some(BasicBlock { instrs, len })
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

    fn find_next_block_after(&self, ip: u32) -> Option<u32> {
        if let Some((&later_ip, _)) = self.blocks.range(ip + 1..).next() {
            Some(later_ip)
        } else {
            None
        }
    }

    /// Decode the instructions within ip..end and save in self.blocks.
    /// Returns false on decode error.
    fn decode_block(&mut self, mem: Mem, ip: u32, end: u32, single_step: bool) -> bool {
        let block = match BasicBlock::decode(mem.slice(ip..end), ip, single_step) {
            Some(block) => block,
            None => return false,
        };
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
        true
    }

    /// Update the basic block found starting at ip, clearing any previous blocks.
    fn update_block(&mut self, mem: Mem, ip: u32, single_step: bool) {
        // If there's a block after this location, ensure we don't decode over it.
        let next_block = self.find_next_block_after(ip);

        // Ensure we don't overlap any previous block.
        self.kill_block(ip);

        if !self.decode_block(mem, ip, next_block.unwrap_or(mem.len()), single_step) {
            // On a decode error, it's possible we needed some of the bytes of the next block to successfully decode.
            // Try that if possible.
            if let Some(next) = next_block {
                self.kill_block(next);
                let next_block = self.find_next_block_after(ip);
                if self.decode_block(mem, ip, next_block.unwrap_or(mem.len()), single_step) {
                    return;
                }
            }
            panic!("failed to decode instruction at {ip:x}");
        }
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.kill_block(addr); // Allow recreating lazily.
        self.breakpoints.insert(addr, mem.get::<u8>(addr));
        mem.put::<u8>(addr, 0xcc); // int3
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.kill_block(addr); // Allow recreating lazily.

        // Allow a subsequent block that might have been split due to the int3
        // to be recreated.
        self.kill_block(addr + 1);

        let prev = self.breakpoints.remove(&addr).unwrap();
        mem.put::<u8>(addr, prev);
    }

    /// Executes the current basic block, updating eip.
    /// Returns false if the CPU stopped.
    pub fn execute_block<'a>(&mut self, cpu: &'a mut CPU, mem: Mem) -> &'a CPUState {
        let ip = cpu.regs.eip;
        let block = match self.blocks.get(&ip) {
            Some(b) => b,
            None => {
                self.update_block(mem, ip, false);
                self.blocks.get(&ip).unwrap()
            }
        };
        for instr in block.instrs.iter() {
            let ip = cpu.regs.eip;
            cpu.regs.eip = instr.next_ip() as u32;
            match cpu.run(mem, instr) {
                CPUState::Running => {}
                CPUState::Error(_) => {
                    // Point the debugger at the failed instruction.
                    cpu.regs.eip = ip;
                    break;
                }
                CPUState::Exit(_) => break,
                CPUState::Blocked => break,
            }
        }
        &cpu.state
    }

    /// Change cache such that there's a single basic block at ip.
    /// This means executing the next execute_block() will execute a single instruction.
    pub fn make_single_step(&mut self, mem: Mem, ip: u32) {
        match self.blocks.get(&ip) {
            Some(b) if b.instrs.len() == 1 => {}
            _ => {
                self.update_block(mem, ip, true);
            }
        };
    }
}
