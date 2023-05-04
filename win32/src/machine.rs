use std::collections::HashMap;

use anyhow::bail;
use x86::X86;

use crate::{
    host, pe,
    shims::{Shims, SHIM_BASE},
    winapi,
};

pub struct Machine {
    pub x86: X86,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Machine {
            x86: X86::new(),
            host,
            state: winapi::State::new(),
            shims: Shims::new(),
            labels: HashMap::new(),
        }
    }
}

/// Manages decoding and running instructions in an owned Machine.
pub struct Runner {
    pub machine: Machine,
    /// Total number of instructions executed.
    pub instr_count: usize,

    icache: x86::InstrCache,
}
impl Runner {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Runner {
            machine: Machine::new(host),
            instr_count: 0,
            icache: x86::InstrCache::new(),
        }
    }

    pub fn load_exe(&mut self, buf: &[u8], cmdline: String, relocate: bool) -> anyhow::Result<()> {
        pe::load_exe(&mut self.machine, buf, cmdline, relocate)
    }

    pub fn add_breakpoint(&mut self, addr: u32) {
        self.icache.add_breakpoint(&mut self.machine.x86.mem, addr)
    }

    pub fn clear_breakpoint(&mut self, addr: u32) {
        self.icache
            .clear_breakpoint(&mut self.machine.x86.mem, addr)
    }

    /// If eip points at a shim address, call the handler and update eip.
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.machine.x86.regs.eip & 0xFFFF_0000 != SHIM_BASE {
            return Ok(false);
        }
        let handler = self
            .machine
            .shims
            .get(self.machine.x86.regs.eip)
            .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
        handler(&mut self.machine);
        // Handler will have set eip to the return address from the stack.
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }

        match self.icache.execute_block(&mut self.machine.x86) {
            Err(x86::StepError::Interrupt) => Ok(false),
            Err(x86::StepError::Error(err)) => bail!(err),
            Ok(count) => {
                self.instr_count += count;
                Ok(true)
            }
        }
    }

    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }

        let ip = self.machine.x86.regs.eip;
        self.icache.make_single_step(&mut self.machine.x86, ip);
        self.execute_block()?;
        // TODO: clear_single_step doesn't help here, because ip now points into the middle
        // of some block and the next time we execute we'll just recreate the partial block anyway.
        // self.icache.clear_single_step(ip);
        Ok(())
    }

    pub fn load_snapshot(&mut self, snap: x86::X86) {
        self.machine.x86 = snap;
    }
}
