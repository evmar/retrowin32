//! The central x86 machine object.

use crate::{
    fpu::FPU,
    icache::InstrCache,
    ops,
    registers::{Flags, Registers},
    Register,
};
use memory::Mem;

#[derive(Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
const MAGIC_ADDR: u32 = 0xFFFF_FFF0;

#[derive(serde::Serialize, serde::Deserialize)]
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
    #[serde(skip)]
    futures: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>>,
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
            futures: Default::default(),
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

    /// Set up the CPU such that we are making an x86->async call, enqueuing a Future
    /// that is polled the next time the CPU executes.
    pub fn call_async(&mut self, future: std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>) {
        self.regs.eip = MAGIC_ADDR;
        self.futures.push(future);
    }

    #[allow(deref_nullptr)]
    fn async_executor(&mut self) {
        let mut future = self.futures.pop().unwrap();
        // TODO: we don't use the waker at all.  Rust doesn't like us passing a random null pointer
        // here but it seems like nothing accesses it(?).
        //let c = unsafe { std::task::Context::from_waker(&Waker::from_raw(std::task::RawWaker::)) };
        let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
        let poll = future.as_mut().poll(context);
        match poll {
            std::task::Poll::Ready(()) => {}
            std::task::Poll::Pending => {
                self.futures.push(future);
            }
        }
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
impl std::future::Future for X86Future {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let cpu = self.cpu;
        let cpu = unsafe { &mut *cpu };
        if cpu.regs.get32(Register::ESP) == self.esp {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}

/// A future which polls until the given CPU is marked as no longer blocked.
pub struct BlockFuture {
    // We assume the CPU is around for the duration of the future execution.
    // https://github.com/rust-lang/futures-rs/issues/316
    cpu: *mut CPU,
}

impl std::future::Future for BlockFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let cpu = self.cpu;
        let cpu = unsafe { &mut *cpu };
        if matches!(cpu.state, CPUState::Blocked(_)) {
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct X86 {
    /// CPUs are boxed because their futures take pointers to self.
    /// In theory we could use Pin here but maybe we just need to revisit how this works in general.
    pub cpus: Vec<Box<CPU>>,
    pub cur_cpu: usize,

    /// Total number of instructions executed.
    pub instr_count: usize,

    #[serde(skip)]
    pub icache: InstrCache,
}

impl X86 {
    pub fn new() -> Self {
        X86 {
            cpus: vec![Box::new(CPU::new())],
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
        self.cpus.push(Box::new(CPU::new()));
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
    pub fn schedule(&mut self) -> &CPUState {
        // log::info!(
        //     "cpustate {:?}",
        //     self.cpus
        //         .iter()
        //         .map(|cpu| format!("{:?}", cpu.state))
        //         .collect::<Vec<_>>()
        // );
        // let prev = self.cur_cpu;
        for _ in 0..self.cpus.len() {
            self.cur_cpu = (self.cur_cpu + 1) % self.cpus.len();
            if self.cpu().state == CPUState::Running {
                break;
            }
        }
        // if self.cur_cpu != prev || !self.cpu().state.is_running() {
        //     log::info!("cpu {prev}=>{} {:?}", self.cur_cpu, self.cpu().state);
        // }

        // TODO: this should pick the soonest-ready CPU because we use it to mark
        // how long of a timer to set.
        &self.cpu().state
    }

    /// Execute one basic block starting at current ip.
    pub fn execute_block(&mut self, mem: Mem) {
        let cpu = &mut *self.cpus[self.cur_cpu];
        debug_assert!(cpu.state.is_running());
        if cpu.regs.eip == MAGIC_ADDR {
            cpu.async_executor();
            return;
        }
        let mut prev_ip = cpu.regs.eip;
        let block = self.icache.get_block(mem, prev_ip);
        for op in block.ops.iter() {
            prev_ip = cpu.regs.eip;
            cpu.regs.eip = op.instr.next_ip() as u32;
            self.instr_count += 1;
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
    }
}
