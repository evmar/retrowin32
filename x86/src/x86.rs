//! The central x86 machine object.

use crate::{
    icache::InstrCache,
    ops,
    registers::{Flags, Registers},
    Mem, VecMem,
};

/// Addresses from 0 up to this point cause panics if we access them.
/// This helps catch implementation bugs earlier.
pub const NULL_POINTER_REGION_SIZE: u32 = 0x1000;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CPU {
    pub regs: Registers,
    // Flags are in principle a register but we hold it outside of regs for lifetime reasons,
    // because there are operations we want to do over mut regs and flags at the same time.
    pub flags: Flags,

    /// True when running, false when stopped, Err when in error state.
    /// Used both for error states and for process exit.
    // TODO: this is gross because we must check it after every instruction.
    // It would be nice if there was some more clever way to thread process exit...
    pub state: Result<bool, String>,
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
    pub fn run(&mut self, mem: &mut Mem, instr: &iced_x86::Instruction) -> &Result<bool, String> {
        ops::execute(self, mem, instr);
        &self.state
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct X86 {
    pub cpu: CPU,
    pub mem: VecMem,

    /// Total number of instructions executed.
    pub instr_count: usize,

    #[serde(skip)]
    icache: InstrCache,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            cpu: CPU::new(),
            mem: VecMem::default(),
            instr_count: 0,
            icache: InstrCache::default(),
        }
    }

    pub fn add_breakpoint(&mut self, addr: u32) {
        self.icache.add_breakpoint(&mut self.mem, addr)
    }

    pub fn clear_breakpoint(&mut self, addr: u32) {
        self.icache.clear_breakpoint(&mut self.mem, addr)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    pub fn execute_block(&mut self) -> Result<bool, String> {
        let count = self.icache.execute_block(&mut self.cpu, &mut self.mem);
        self.instr_count += count;
        self.cpu.state.clone()
    }

    pub fn single_step(&mut self) -> Result<(), String> {
        let ip = self.cpu.regs.eip;
        self.icache.make_single_step(&mut self.mem, ip);
        self.execute_block()?;
        // TODO: clear_single_step doesn't help here, because ip now points into the middle
        // of some block and the next time we execute we'll just recreate the partial block anyway.
        // self.icache.clear_single_step(ip);
        Ok(())
    }

    pub fn load_snapshot(&mut self, snap: X86) {
        *self = snap;
    }
}
