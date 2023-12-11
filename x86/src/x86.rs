//! The central x86 machine object.

use crate::{
    icache::InstrCache,
    ops,
    registers::{Flags, Registers},
};
use memory::Mem;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CPU {
    pub regs: Registers,
    // Flags are in principle a register but we moved it outside of regs for lifetime reasons,
    // because there are operations we want to do over mut regs and flags at the same time.
    // TODO: this may no longer be necessary (?)
    pub flags: Flags,

    /// True when running, false when stopped, Err when in error state.
    /// Used both for error states and for process exit.
    // TODO: this is gross because we must check it after every instruction.
    // It would be nice if there was some more clever way to thread process exit...
    state: Result<bool, String>,
}
impl CPU {
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
        CPU {
            regs,
            flags: Flags::empty(),
            state: Ok(true),
        }
    }

    pub fn stop(&mut self) {
        self.state = Ok(false);
    }

    pub fn err(&mut self, msg: String) {
        self.state = Err(msg);
    }
    // fn crash(&mut self, msg: String) {
    //     self.crashed = Some(msg);
    //     self.stopped = true;
    // }

    // /// Check whether reading a T from mem[addr] would cause OOB, and crash() if so.
    // fn check_oob<T>(&mut self, addr: u32) -> bool {
    //     if addr < NULL_POINTER_REGION_SIZE {
    //         self.crash(format!("crash: null pointer at {addr:#x}"));
    //         return true;
    //     }
    //     let end = addr.wrapping_add(std::mem::size_of::<T>() as u32);
    //     if end > self.mem.len() as u32 || end < addr {
    //         self.crash(format!("crash: oob pointer at {addr:#x}"));
    //         return true;
    //     }
    //     false
    // }

    /// Executes an instruction, leaving eip alone.
    pub fn run(&mut self, mem: Mem, instr: &iced_x86::Instruction) -> &Result<bool, String> {
        self.state = Ok(true);
        ops::execute(self, mem, instr);
        &self.state
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct X86 {
    pub cpu: CPU,

    /// Total number of instructions executed.
    pub instr_count: usize,

    #[serde(skip)]
    icache: InstrCache,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            cpu: CPU::new(),
            instr_count: 0,
            icache: InstrCache::default(),
        }
    }

    pub fn add_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.icache.add_breakpoint(mem, addr)
    }

    pub fn clear_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.icache.clear_breakpoint(mem, addr)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    pub fn execute_block(&mut self, mem: Mem) -> Result<bool, String> {
        let count = self.icache.execute_block(&mut self.cpu, mem);
        self.instr_count += count;
        self.cpu.state.clone()
    }

    pub fn single_step(&mut self, mem: Mem) -> Result<(), String> {
        let ip = self.cpu.regs.eip;
        self.icache.make_single_step(mem, ip);
        self.execute_block(mem)?;
        // TODO: clear_single_step doesn't help here, because ip now points into the middle
        // of some block and the next time we execute we'll just recreate the partial block anyway.
        // self.icache.clear_single_step(ip);
        Ok(())
    }

    pub fn load_snapshot(&mut self, snap: X86) {
        *self = snap;
    }
}
