//! The central x86 machine object.

use crate::{
    fpu::FPU,
    icache::InstrCache,
    ops,
    registers::{Flags, Registers},
    Register,
};
use memory::Mem;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum CPUState {
    #[default]
    Running,
    Blocked(Option<u32>),
    DebugBreak,
    Error(String),
    Exit(u32),
}

impl CPUState {
    pub fn is_running(&self) -> bool {
        matches!(*self, CPUState::Running)
    }
}

/// When eip==MAGIC_ADDR, the CPU executes futures (async tasks) rather than x86 code.
pub const MAGIC_ADDR: u32 = 0xFFFF_FFF0;

pub struct CPU {
    pub regs: Registers,
    // Flags are in principle a register but we moved it outside of regs for lifetime reasons,
    // because there are operations we want to do over mut regs and flags at the same time.
    // TODO: this may no longer be necessary (?)
    pub flags: Flags,
    pub fpu: FPU,

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

    /// Set up the CPU such that we are making a Rust->x86 call, returning a Future
    /// that completes when the x86 call returns.
    pub fn call_x86(&mut self, mem: Mem, func: u32, args: Vec<u32>) -> X86Future {
        // Save original esp, as that's the marker that we use to know when the call is done.
        let esp = self.regs.get32(Register::ESP);
        // Push the args in reverse order.
        for &arg in args.iter().rev() {
            ops::push(self, mem, arg);
        }
        ops::push(self, mem, MAGIC_ADDR); // return address
        self.regs.eip = func;

        // Clear registers to make traces clean.
        // Other registers are callee-saved per ABI.
        self.regs.set32(Register::EAX, 0);
        self.regs.set32(Register::ECX, 0);
        self.regs.set32(Register::EDX, 0);

        X86Future { cpu: self, esp }
    }

    pub fn block(&mut self, wait: Option<u32>) -> BlockFuture {
        self.state = CPUState::Blocked(wait);
        BlockFuture { cpu: self }
    }
}

pub struct X86Future {
    // We assume the CPU is around for the duration of the future execution.
    // https://github.com/rust-lang/futures-rs/issues/316
    cpu: *mut CPU,
    esp: u32,
}
impl Future for X86Future {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let cpu = self.cpu;
        let cpu = unsafe { &mut *cpu };
        if cpu.regs.get32(Register::ESP) == self.esp {
            Poll::Ready(cpu.regs.get32(Register::EAX))
        } else {
            Poll::Pending
        }
    }
}

/// A future which polls until the given CPU is marked as no longer blocked.
pub struct BlockFuture {
    // We assume the CPU is around for the duration of the future execution.
    // https://github.com/rust-lang/futures-rs/issues/316
    cpu: *mut CPU,
}

impl Future for BlockFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let cpu = self.cpu;
        let cpu = unsafe { &mut *cpu };
        if matches!(cpu.state, CPUState::Blocked(_)) {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

pub struct X86 {
    /// CPUs are boxed because their futures take pointers to self.
    pub cpus: Vec<Pin<Box<CPU>>>,
    pub cur_cpu: usize,

    /// Total number of instructions executed.
    pub instr_count: u64,

    pub icache: InstrCache,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            cpus: vec![Box::pin(CPU::new())],
            cur_cpu: 0,
            instr_count: 0,
            icache: InstrCache::default(),
        }
    }

    pub fn cpu(&self) -> &CPU {
        &*self.cpus[self.cur_cpu]
    }

    pub fn cpu_mut(&mut self) -> &mut CPU {
        &mut *self.cpus[self.cur_cpu]
    }

    pub fn new_cpu(&mut self) -> &mut CPU {
        self.cpus.push(Box::pin(CPU::new()));
        self.cpus.last_mut().unwrap()
    }

    /// Schedule the next runnable thread to run.
    pub fn schedule(&mut self) {
        // log::info!(
        //     "cpustate {:?}",
        //     self.cpus
        //         .iter()
        //         .map(|cpu| format!("{:?}", cpu.state))
        //         .collect::<Vec<_>>()
        // );
        // let prev = self.cur_cpu;

        // Common perf-sensitive case: find next runnable CPU.
        for i in 0..self.cpus.len() {
            let i = (self.cur_cpu + i) % self.cpus.len();
            if self.cpus[i].state.is_running() {
                self.cur_cpu = i;
                return;
            }
        }

        // Otherwise, find the CPU that will unblock soonest.
        let mut soonest = None;
        for (i, cpu) in self.cpus.iter().enumerate() {
            match cpu.state {
                CPUState::Running => {}
                CPUState::DebugBreak | CPUState::Error(_) | CPUState::Exit(_) => {
                    self.cur_cpu = i;
                    return;
                }
                CPUState::Blocked(wait) => match soonest {
                    None => soonest = Some((i, wait)),
                    Some((_, soonest_wait)) => {
                        if wait < soonest_wait {
                            soonest = Some((i, wait));
                        }
                    }
                },
            }
        }
        self.cur_cpu = soonest.unwrap().0;

        // if self.cur_cpu != prev || !self.cpu().state.is_running() {
        //     log::info!("cpu {prev}=>{} {:?}", self.cur_cpu, self.cpu().state);
        // }
    }

    /// Execute one basic block starting at current ip.
    pub fn execute_block(&mut self, mem: Mem, instruction_count: usize) -> usize {
        let cpu = &mut *self.cpus[self.cur_cpu];
        debug_assert!(cpu.state.is_running());
        let mut prev_ip = cpu.regs.eip;
        let block = self.icache.get_block(mem, prev_ip, instruction_count);
        for op in block.ops.iter() {
            prev_ip = cpu.regs.eip;
            cpu.regs.eip = op.instr.next_ip() as u32;
            self.instr_count = self.instr_count.wrapping_add(1);
            (op.op)(cpu, mem, &op.instr);
            if !cpu.state.is_running() {
                break;
            }
        }
        match cpu.state {
            CPUState::Error(_) => {
                // Point the debugger at the failed instruction.
                cpu.regs.eip = prev_ip;
            }
            _ => {}
        }
        block.len as usize
    }
}
