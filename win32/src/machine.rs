use std::collections::HashMap;
use x86::X86;

use crate::{
    host, pe,
    shims::{Shims, SHIM_BASE},
    winapi,
};

/// Integrates the X86 CPU emulator with the Windows OS support.
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

    pub fn load_exe(&mut self, buf: &[u8], cmdline: String, relocate: bool) -> anyhow::Result<()> {
        pe::load_exe(self, buf, cmdline, relocate)
    }

    /// If eip points at a shim address, call the handler and update eip.
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.x86.cpu.regs.eip & 0xFFFF_0000 != SHIM_BASE {
            return Ok(false);
        }
        let handler = self
            .shims
            .get(self.x86.cpu.regs.eip)
            .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
        handler(self);
        // Handler will have set eip to the return address from the stack.
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }
        self.x86.execute_block().map_err(|err| anyhow::anyhow!(err))
    }

    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }
        self.x86.single_step().map_err(|err| anyhow::anyhow!(err))
    }
}
