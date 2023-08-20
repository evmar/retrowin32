use crate::{host, pe, shims::Shims, winapi};
use memory::{Mem, MemImpl};
use std::collections::HashMap;

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct Machine {
    #[cfg(feature = "cpuemu")]
    pub x86: x86::X86,
    pub memory: MemImpl,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let mut memory = MemImpl::default();
        let mut state = winapi::State::new(&mut memory);

        #[cfg(feature = "cpuemu")]
        let shims = {
            state = state;
            Shims::new()
        };
        #[cfg(not(feature = "cpuemu"))]
        let shims = {
            let mapping =
                state
                    .kernel32
                    .mappings
                    .alloc(0x1000, "shims x64 trampoline".into(), &mut memory);
            Shims::new(mapping.addr as u64 as *mut u8, mapping.size)
        };

        Machine {
            #[cfg(feature = "cpuemu")]
            x86: x86::X86::new(),
            memory,
            host,
            state,
            shims,
            labels: HashMap::new(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: bool,
    ) -> anyhow::Result<pe::LoadedAddrs> {
        pe::load_exe(self, buf, cmdline, relocate)
    }

    /// If eip points at a shim address, call the handler and update eip.
    #[cfg(feature = "cpuemu")]
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.x86.cpu.regs.eip & 0xFFFF_0000 != crate::shims_cpuemu::SHIM_BASE {
            return Ok(false);
        }
        let crate::shims::Shim {
            func,
            stack_consumed,
            is_async,
            ..
        } = *self.shims.get(self.x86.cpu.regs.eip);
        let ret = unsafe { func(self, self.x86.cpu.regs.esp) };
        if !is_async {
            self.x86.cpu.regs.eip = self.mem().get::<u32>(self.x86.cpu.regs.esp);
            self.x86.cpu.regs.esp += stack_consumed;
            self.x86.cpu.regs.eax = ret;
        } else {
            // Async handler will manage the return address etc.
        }
        Ok(true)
    }

    // Execute one basic block.  Returns Ok(false) if we stopped early.
    #[cfg(feature = "cpuemu")]
    pub fn execute_block(&mut self) -> anyhow::Result<bool> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(true);
        }
        self.x86
            .execute_block(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }

    #[cfg(feature = "cpuemu")]
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
