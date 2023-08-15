use crate::{
    host, pe,
    shims::{Shims, SHIM_BASE},
    winapi,
};
use memory::{Mem, MemImpl};
use std::collections::HashMap;
use x86::X86;

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct Machine {
    pub x86: X86,
    pub memory: MemImpl,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let mut memory = MemImpl::default();
        let state = winapi::State::new(&mut memory);
        Machine {
            x86: X86::new(),
            memory,
            host,
            state,
            shims: Shims::new(),
            labels: HashMap::new(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn load_exe(&mut self, buf: &[u8], cmdline: String, relocate: bool) -> anyhow::Result<()> {
        pe::load_exe(self, buf, cmdline, relocate)
    }

    /// If eip points at a shim address, call the handler and update eip.
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.x86.cpu.regs.eip & 0xFFFF_0000 != SHIM_BASE {
            return Ok(false);
        }
        let crate::shims::Shim {
            func,
            stack_consumed,
            ..
        } = *self.shims.get(self.x86.cpu.regs.eip);
        let ret = unsafe { func(self, self.x86.cpu.regs.esp) };
        if let Some(stack_consumed) = stack_consumed {
            self.x86.cpu.regs.eip = self.mem().get::<u32>(self.x86.cpu.regs.esp);
            self.x86.cpu.regs.esp += stack_consumed;
            self.x86.cpu.regs.eax = ret;
        } else {
            // Async handler will manage the return address etc.
        }
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }
        self.x86
            .execute_block(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }

    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }
        self.x86
            .single_step(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }
}
