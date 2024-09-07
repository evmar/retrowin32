use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims_emu::Shims,
    winapi,
};
use memory::{Extensions, Mem};
use std::{collections::HashMap, path::Path};

pub struct BoxMem(Box<[u8]>);

impl BoxMem {
    fn new(size: usize) -> Self {
        let mut buf = Vec::<u8>::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        Self(buf.into_boxed_slice())
    }

    // TODO: we can support growing under wasm by using a custom allocator that
    // ensures this is the last thing in the heap.
    // pub fn grow();

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn mem(&self) -> Mem {
        Mem::from_slice(&self.0)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

pub struct Emulator {
    pub x86: x86::X86,
    pub memory: BoxMem,
    pub shims: Shims,

    /// Places where we've patched out the instruction with an int3.
    /// The map values are the bytes from before the breakpoint.
    breakpoints: HashMap<u32, u8>,
}

pub type MemImpl = BoxMem;
pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = BoxMem::new(256 << 20);
        let kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);
        let shims = Shims::default();
        let state = winapi::State::new(&mut memory, kernel32);

        Machine {
            emu: Emulator {
                x86: x86::X86::new(),
                memory,
                shims,
                breakpoints: Default::default(),
            },
            host,
            state,
            labels: HashMap::new(),
            exe_path: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.emu.memory.mem()
    }

    /// Initialize a memory mapping for the stack and return the initial stack pointer.
    pub fn create_stack(&mut self, desc: String, stack_size: u32) -> u32 {
        let stack = self
            .state
            .kernel32
            .mappings
            .alloc(stack_size, desc, &mut self.emu.memory);
        let stack_pointer = stack.addr + stack.size - 4;
        stack_pointer
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        path: &Path,
        relocate: Option<Option<u32>>,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, path, relocate)?;

        let stack_pointer = self.create_stack("stack".into(), exe.stack_size);
        let regs = &mut self.emu.x86.cpu_mut().regs;
        regs.set32(x86::Register::ESP, stack_pointer);
        regs.set32(x86::Register::EBP, stack_pointer);
        regs.fs_addr = self.state.kernel32.teb;

        // To make CPU traces match more closely, set up some registers to what their
        // initial values appear to be from looking in a debugger.
        regs.set32(x86::Register::ECX, exe.entry_point);
        regs.set32(x86::Register::EDX, exe.entry_point);
        regs.set32(x86::Register::ESI, exe.entry_point);
        regs.set32(x86::Register::EDI, exe.entry_point);

        let retrowin32_main = winapi::kernel32::get_kernel32_builtin(self, "retrowin32_main");
        let cpu = self.emu.x86.cpu_mut();
        x86::ops::push(cpu, self.emu.memory.mem(), exe.entry_point);
        x86::ops::push(cpu, self.emu.memory.mem(), 0); // return address
        cpu.regs.eip = retrowin32_main;

        self.exe_path = path.to_path_buf();
        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    pub fn single_step(&mut self) {
        if !crate::shims_emu::is_ip_at_shim_call(self.emu.x86.cpu().regs.eip) {
            self.emu.x86.single_step_next_block(self.emu.memory.mem());
        }
        self.run();
    }

    pub fn unblock_all(&mut self) {
        for cpu in self.emu.x86.cpus.iter_mut() {
            if matches!(
                cpu.state,
                x86::CPUState::Blocked(_) | x86::CPUState::DebugBreak
            ) {
                cpu.state = x86::CPUState::Running;
            }
        }
    }

    pub fn unblock(&mut self) {
        let cpu = self.emu.x86.cpu_mut();
        if matches!(
            cpu.state,
            x86::CPUState::Blocked(_) | x86::CPUState::DebugBreak
        ) {
            cpu.state = x86::CPUState::Running;
        }
    }

    pub fn run(&mut self) -> bool {
        self.emu.x86.schedule();
        match &self.emu.x86.cpu().state {
            x86::CPUState::Running => self.execute_block(),
            x86::CPUState::Blocked(wait) => {
                let wait = *wait;
                if self.host.block(wait) {
                    self.unblock();
                } else {
                    return false;
                }
            }
            _ => return false,
        }
        true
    }

    // Execute one basic block.  Returns false if we stopped early.
    fn execute_block(&mut self) {
        debug_assert!(self.emu.x86.cpu().state.is_running());
        let eip = self.emu.x86.cpu().regs.eip;
        if crate::shims_emu::is_ip_at_shim_call(eip) {
            if self.emu.breakpoints.contains_key(&eip) {
                self.emu.x86.cpu_mut().state = x86::CPUState::DebugBreak;
                return;
            }

            crate::shims_emu::handle_shim_call(self);
            // Treat any shim call as a single block and return here.
            return;
        }
        self.emu.x86.execute_block(self.emu.memory.mem())
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        self.emu
            .x86
            .cpu_mut()
            .call_x86(self.emu.memory.mem(), func, args)
            .await
    }

    // pub fn dump_stack(&self) {
    //     let esp = self.emu.x86.cpu.regs.esp;
    //     for addr in ((esp - 0x10)..(esp + 0x10)).step_by(4) {
    //         let extra = if addr == esp { " <- esp" } else { "" };
    //         log::info!(
    //             "{:08x} {:08x}{extra}",
    //             addr,
    //             self.mem().get_pod::<u32>(addr)
    //         );
    //     }
    // }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.entry(addr) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(entry) => {
                if crate::shims_emu::is_ip_at_shim_call(addr) {
                    entry.insert(0);
                } else {
                    let mem = self.emu.memory.mem();
                    entry.insert(mem.get_pod::<u8>(addr));
                    mem.put_pod::<u8>(addr, 0xcc); // int3
                    self.emu.x86.icache.clear_cache(addr);
                }
                true
            }
        }
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.remove(&addr) {
            Some(prev) => {
                if !crate::shims_emu::is_ip_at_shim_call(addr) {
                    self.emu.x86.icache.clear_cache(addr);
                    self.mem().put_pod::<u8>(addr, prev);
                }
                true
            }
            None => false,
        }
    }
}
