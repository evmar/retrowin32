//! For performance, we cache decoded CPU instructions in basic blocks,
//! so when executing we always know we'll execute a full basic block in
//! linear order before making any jumps.
//!
//! Because of this, we cannot represent a CPU state that is partway through
//! a basic block.  To implement single-stepping and breakpoints, we break
//! any affected basic block into smaller pieces to maintain the invariant of
//! always executing through a basic block's end.
//!
//! Some good notes on how to make this kind of thing perform well:
//! http://www.emulators.com/docs/nx25_nostradamus.htm

use memory::{Extensions, Mem};
use std::collections::HashMap;

const CACHE_LINES: usize = 2 << 10;

pub struct Op {
    pub instr: iced_x86::Instruction,
    /// The function that implements instr.  Cached here to avoid looking it up;
    /// this was worth about 10% performance in a quick test.
    pub op: crate::ops::Op,
}

#[derive(Default)]
pub struct BasicBlock {
    /// Number of x86 instruction bytes covered by this block.
    pub len: u32,
    pub ops: Vec<Op>,
}

impl BasicBlock {
    fn decode(buf: Mem, ip: u32, single_step: bool) -> Option<Self> {
        let mut ops = Vec::new();
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
            let op =
                crate::ops::decode(&instr).unwrap_or_else(|| todo!("{instr} ({:?})", instr.code()));
            ops.push(Op { op, instr });
            len += instr.len() as u32;
            if instr.flow_control() != iced_x86::FlowControl::Next || single_step {
                break;
            }
        }
        Some(BasicBlock { ops, len })
    }
}

#[derive(Default)]
struct CacheLine {
    ip: u32,
    block: BasicBlock,
}

/// Cache of decoded instructions as basic blocks.
pub struct InstrCache {
    lines: Box<[CacheLine; CACHE_LINES]>,
    hit: usize,
    miss: usize,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the bytes from before the breakpoint.
    breakpoints: HashMap<u32, u8>,
}

impl Default for InstrCache {
    fn default() -> Self {
        let mut lines = Vec::with_capacity(CACHE_LINES);
        lines.resize_with(CACHE_LINES, || Default::default());
        InstrCache {
            lines: lines.try_into().unwrap_or_else(|_| panic!()),
            hit: 0,
            miss: 0,
            breakpoints: HashMap::new(),
        }
    }
}

impl InstrCache {
    pub fn stats(&self) -> String {
        let total = self.hit + self.miss;
        let percent = if total > 0 { self.hit * 100 / total } else { 0 };
        format!(
            "{} hit, {} miss, {}% hit rate",
            self.hit, self.miss, percent
        )
    }

    /// Remove any cache line that covers ip.
    fn clear_cache(&mut self, ip: u32) {
        for line in self.lines.iter_mut() {
            if line.ip <= ip && line.ip + line.block.len > ip {
                line.ip = 0;
                return;
            }
        }
    }

    /// Decode the instructions starting at ip and save in self.lines.
    fn decode_block(&mut self, mem: Mem, ip: u32, single_step: bool) -> &BasicBlock {
        let block = match BasicBlock::decode(mem.slice(ip..), ip, single_step) {
            Some(block) => block,
            None => unreachable!(),
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
        let index = ip as usize % self.lines.len();
        self.lines[index] = CacheLine { ip, block };
        &self.lines[index].block
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.clear_cache(addr); // Allow recreating lazily.
        self.breakpoints.insert(addr, mem.get_pod::<u8>(addr));
        mem.put::<u8>(addr, 0xcc); // int3
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.clear_cache(addr); // Allow recreating lazily.
        let prev = self.breakpoints.remove(&addr).unwrap();
        mem.put::<u8>(addr, prev);
    }

    /// Gets basic block starting at a given ip.
    pub fn get_block<'a>(&'a mut self, mem: Mem, ip: u32) -> &'a BasicBlock {
        let index = ip as usize % self.lines.len();
        if self.lines[index].ip == ip {
            self.hit += 1;
            return &self.lines[index].block;
        } else {
            self.miss += 1;
            self.decode_block(mem, ip, false)
        }
    }

    /// Change cache such that there's a single basic block at ip.
    /// This means the next get_block() will get a block with a single instruction.
    pub fn make_single_step(&mut self, mem: Mem, ip: u32) {
        self.decode_block(mem, ip, true);
    }
}
