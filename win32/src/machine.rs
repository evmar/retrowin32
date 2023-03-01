use std::collections::HashMap;

use anyhow::bail;
use x86::X86;

use crate::{host, pe::ImageSectionFlags, winapi, windows::load_exe};

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims(Vec<Shim>);
impl Shims {
    fn new() -> Self {
        Shims(Vec::new())
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        let id = SHIM_BASE | self.0.len() as u32;
        self.0.push(Shim { name, handler });
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut Machine)> {
        let index = (addr & 0x0000_FFFF) as usize;
        match self.0.get(index) {
            Some(shim) => {
                if let Some(handler) = &shim.handler {
                    return Some(handler);
                }
                log::error!("unimplemented: {}", shim.name);
            }
            None => log::error!("unknown import reference at {:x}", addr),
        };
        None
    }

    pub fn lookup(&self, name: &str) -> Option<u32> {
        if let Some(idx) = self.0.iter().position(|shim| shim.name == name) {
            return Some(SHIM_BASE | idx as u32);
        }
        None
    }
}

pub struct Machine {
    pub x86: X86,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
}

impl Machine {
    pub fn new(host: Box<dyn host::Host>) -> Self {
        Machine {
            x86: X86::new(),
            host,
            state: winapi::State::new(),
            shims: Shims::new(),
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

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
    ) -> anyhow::Result<HashMap<u32, String>> {
        let labels = load_exe(&mut self.machine, buf, cmdline)?;

        // Disassemble the 'code' section.
        let mapping = self
            .machine
            .state
            .kernel32
            .mappings
            .vec()
            .iter()
            .find(|mapping| mapping.flags.contains(ImageSectionFlags::CODE))
            .ok_or_else(|| anyhow::anyhow!("no code section"))?;
        let section =
            &self.machine.x86.mem[mapping.addr as usize..(mapping.addr + mapping.size) as usize];
        self.icache.disassemble(section, mapping.addr);
        self.icache
            .jmp(&self.machine.x86.mem, self.machine.x86.regs.eip)?;

        Ok(labels)
    }

    pub fn add_breakpoint(&mut self, addr: u32) {
        self.icache.add_breakpoint(addr)
    }

    pub fn clear_breakpoint(&mut self, addr: u32) {
        self.icache.clear_breakpoint(addr)
    }

    /// If eip points at a shim address, call the handler and update eip.
    fn check_shim_call(&mut self) -> anyhow::Result<()> {
        if self.machine.x86.regs.eip & 0xFFFF_0000 != SHIM_BASE {
            return Ok(());
        }
        let ret = x86::ops::pop(&mut self.machine.x86);
        let handler = self
            .machine
            .shims
            .get(self.machine.x86.regs.eip)
            .ok_or_else(|| anyhow::anyhow!("missing shim"))?;
        handler(&mut self.machine);
        x86::ops::x86_jmp(&mut self.machine.x86, ret).map_err(|err| anyhow::anyhow!(err))
    }

    // Single-step execution.  Returns Ok(false) if we stopped.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        self.instr_count += 1;
        match self.icache.step(&mut self.machine.x86) {
            Err(x86::StepError::Interrupt) => Ok(false),
            Err(x86::StepError::Error(err)) => bail!(err),
            Ok(false) => {
                self.check_shim_call()?;
                // Instruction changed eip.  Update icache to match.
                self.icache
                    .jmp(&self.machine.x86.mem, self.machine.x86.regs.eip)?;
                Ok(true)
            }
            Ok(true) => Ok(true),
        }
    }

    // Multi-step execution.  Returns Ok(false) on breakpoint.
    pub fn step_many(&mut self, count: usize) -> anyhow::Result<usize> {
        for i in 0..count {
            if !self.step()? {
                return Ok(i);
            }
        }
        Ok(count)
    }

    pub fn load_snapshot(&mut self, snap: x86::X86) {
        self.machine.x86 = snap;
        self.icache
            .jmp(&self.machine.x86.mem, self.machine.x86.regs.eip)
            .unwrap();
    }
}
