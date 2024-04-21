//! The central x86 machine object.

use crate::{
    fpu::FPU,
    icache::InstrCache,
    ops,
    registers::{Flags, Registers},
};
use memory::Mem;

#[derive(Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CPUState {
    #[default]
    Running,
    Blocked,
    Error(String),
    Exit(u32),
}

impl CPUState {
    pub fn is_running(&self) -> bool {
        matches!(*self, CPUState::Running)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CPU {
    pub regs: Registers,
    // Flags are in principle a register but we moved it outside of regs for lifetime reasons,
    // because there are operations we want to do over mut regs and flags at the same time.
    // TODO: this may no longer be necessary (?)
    pub flags: Flags,
    pub fpu: FPU,

    /// Total number of instructions executed.
    pub instr_count: usize,

    pub state: CPUState,
}

impl CPU {
    pub fn new() -> Self {
        unsafe {
            ops::init_op_tab();
        }
        CPU {
            regs: Registers::default(),
            flags: Flags::empty(),
            fpu: FPU::default(),
            instr_count: 0,
            state: Default::default(),
        }
    }

    pub fn err(&mut self, msg: String) {
        self.state = CPUState::Error(msg);
    }

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

    /// Executes an instruction, leaving eip alone.  Returns false on CPU stop.
    pub fn run(&mut self, mem: Mem, instr: &iced_x86::Instruction) -> &CPUState {
        self.state = CPUState::Running;
        ops::execute(self, mem, instr);
        self.instr_count += 1;
        &self.state
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct X86 {
    pub cpu: CPU,

    #[serde(skip)]
    pub icache: InstrCache,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            cpu: CPU::new(),
            icache: InstrCache::default(),
        }
    }

    pub fn add_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.icache.add_breakpoint(mem, addr)
    }

    pub fn clear_breakpoint(&mut self, mem: Mem, addr: u32) {
        self.icache.clear_breakpoint(mem, addr)
    }

    pub fn single_step_next_block(&mut self, mem: Mem) {
        let ip = self.cpu.regs.eip;
        self.icache.make_single_step(mem, ip);
    }

    // Execute one basic block.  Returns false if we stopped early.
    pub fn execute_block(&mut self, mem: Mem) -> &CPUState {
        self.icache.execute_block(&mut self.cpu, mem)
        // NOTE: clear_single_step doesn't help here, because ip now points into the middle
        // of some block and the next time we execute we'll just recreate the partial block anyway.
        // self.icache.clear_single_step(ip);
    }
}
