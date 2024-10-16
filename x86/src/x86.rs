//! The central x86 machine object.

use crate::{
    fpu::FPU,
    icache::{BasicBlock, InstrCache},
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
    SysCall,
    Error(String),
    Free,
}

impl CPUState {
    pub fn is_running(&self) -> bool {
        matches!(self, CPUState::Running)
    }
}

/// When eip==MAGIC_ADDR, the CPU executes futures (async tasks) rather than x86 code.
const MAGIC_ADDR: u32 = 0xFFFF_FFF0;

// Similar to futures::future::BoxFuture, but 'static + !Send.
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

#[derive(Default)]
pub struct CPU {
    pub regs: Registers,
    // Flags are in principle a register but we moved it outside of regs for lifetime reasons,
    // because there are operations we want to do over mut regs and flags at the same time.
    // TODO: this may no longer be necessary (?)
    pub flags: Flags,
    pub fpu: FPU,

    pub state: CPUState,

    /// If eip==MAGIC_ADDR, then the next step is to poll a future rather than
    /// executing a basic block.
    futures: Vec<BoxFuture<()>>,
}

impl CPU {
    pub fn err(&mut self, msg: String) {
        self.state = CPUState::Error(msg);
    }

    /// Jump to an address, verifying it's within valid bounds.
    pub fn jmp(&mut self, mem: Mem, addr: u32) {
        if addr < 0x1000 {
            self.err(format!("jmp to null page addr={addr:x}"));
            return;
        }
        if mem.is_oob::<u8>(addr) && addr != MAGIC_ADDR {
            self.err(format!("jmp to oob addr={addr:x}"));
            return;
        }
        self.regs.eip = addr;
    }

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
        self.jmp(mem, func);

        // Clear registers to make traces clean.
        // Other registers are callee-saved per ABI.
        self.regs.set32(Register::EAX, 0);
        self.regs.set32(Register::ECX, 0);
        self.regs.set32(Register::EDX, 0);

        X86Future { cpu: self, esp }
    }

    /// Set up the CPU such that we are making an x86->async call, enqueuing a Future
    /// that is polled the next time the CPU executes.
    pub fn call_async(&mut self, future: BoxFuture<u32>, return_address: u32) {
        self.regs.eip = MAGIC_ADDR;
        let cpu = self as *mut CPU;
        self.futures.push(Box::pin(async move {
            let cpu = unsafe { &mut *cpu };
            let ret = future.await;
            cpu.regs.set32(Register::EAX, ret);
            cpu.regs.eip = return_address;
        }));
    }

    fn async_executor(&mut self) {
        let future = self.futures.last_mut().unwrap();
        // TODO: we don't use the waker at all.  Rust doesn't like us passing a random null pointer
        // here but it seems like nothing accesses it(?).
        //let c = unsafe { std::task::Context::from_waker(&Waker::from_raw(std::task::RawWaker::)) };
        #[allow(deref_nullptr)]
        let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
        let poll = future.as_mut().poll(context);
        match poll {
            Poll::Ready(()) => {
                self.futures.pop();
            }
            Poll::Pending => {}
        }
    }

    pub fn block(&mut self, wait: Option<u32>) -> BlockFuture {
        self.state = CPUState::Blocked(wait);
        BlockFuture { cpu: self }
    }

    // Useful to disassemble this function (see misc/dump-fn.sh):
    // #[inline(never)]
    pub fn execute_block(&mut self, mem: Mem, block: &BasicBlock) -> usize {
        // Performance note: this function is the central hottest loop in the emulator.
        // Some things I've tried:
        // - changing eip to be a usize: worth a few percent when usize!=u32
        // - unrolling the loop: worth a few percent
        //   To try it:
        //   1) make loop iterative: loop { let Some(op) = iter.next() else { break }; ... }
        //   2) macro paste the block: macro_rules! unroll { ($code:tt) => { $code $code $code $code } }

        let mut count = 0;
        for op in block.ops.iter() {
            let prev_ip = self.regs.eip;
            self.regs.eip = op.instr.next_ip() as u32;
            count += 1;
            (op.op)(self, mem, &op.instr);
            match self.state {
                CPUState::Running => continue,
                CPUState::Error(_) => {
                    // Point the debugger at the failed instruction.
                    self.regs.eip = prev_ip;
                    break;
                }
                _ => break,
            }
        }
        count
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
    pub instr_count: usize,

    pub icache: InstrCache,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            cpus: Default::default(),
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
        self.cpus.push(Box::pin(CPU::default()));
        self.cpus.last_mut().unwrap()
    }

    pub fn single_step_next_block(&mut self, mem: Mem) {
        let ip = self.cpu().regs.eip;
        if ip == MAGIC_ADDR {
            return;
        }
        self.icache.make_single_step(mem, ip);
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
            let i = (self.cur_cpu + i + 1) % self.cpus.len();
            match self.cpus[i].state {
                CPUState::Running | CPUState::SysCall | CPUState::Error(_) => {
                    self.cur_cpu = i;
                    return;
                }
                _ => {}
            }
        }

        // Otherwise, find the CPU that will unblock soonest.
        let mut soonest = None;
        for (i, cpu) in self.cpus.iter().enumerate() {
            match cpu.state {
                CPUState::Running | CPUState::Free => {}
                CPUState::DebugBreak | CPUState::Error(_) | CPUState::SysCall => {
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
    pub fn execute_block(&mut self, mem: Mem) {
        let cpu = &mut *self.cpus[self.cur_cpu];
        debug_assert!(cpu.state.is_running());
        if cpu.regs.eip == MAGIC_ADDR {
            cpu.async_executor();
            return;
        }
        let block = self.icache.get_block(mem, cpu.regs.eip);
        let count = cpu.execute_block(mem, block);
        self.instr_count = self.instr_count.wrapping_add(count);
    }
}
