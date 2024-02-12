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

    /// Total number of instructions executed.
    pub instr_count: usize,

    /// Set when the CPU wants to stop, due to error, interrupt, or process exit.
    /// TODO: model process exit differently somehow?
    pub error: Option<String>,
}
impl CPU {
    pub fn new() -> Self {
        unsafe {
            ops::init_op_tab();
        }
        CPU {
            regs: Registers::new(),
            flags: Flags::empty(),
            instr_count: 0,
            error: None,
        }
    }

    pub fn err(&mut self, msg: String) {
        self.error = Some(msg);
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
    pub fn run(&mut self, mem: Mem, instr: &iced_x86::Instruction) -> bool {
        self.error = None;
        ops::execute(self, mem, instr);
        self.instr_count += 1;
        self.error.is_none()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct X86 {
    pub cpu: CPU,

    #[serde(skip)]
    icache: InstrCache,
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

    // Execute one basic block.  Returns false if we stopped early.
    pub fn execute_block(&mut self, mem: Mem, single_step: bool) -> bool {
        if single_step {
            let ip = self.cpu.regs.eip;
            self.icache.make_single_step(mem, ip);
        }
        self.icache.execute_block(&mut self.cpu, mem)
        // NOTE: clear_single_step doesn't help here, because ip now points into the middle
        // of some block and the next time we execute we'll just recreate the partial block anyway.
        // self.icache.clear_single_step(ip);
    }

    pub fn load_snapshot(&mut self, snap: X86) {
        *self = snap;
    }
}
