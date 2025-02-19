//! Implements Machine using retrowin32's x86 emulator as found in the x86/ directory.

use crate::{
    host, loader,
    machine::{LoadedAddrs, MachineX, Status},
    shims::{Handler, Shims},
    winapi::{
        self,
        kernel32::{create_thread, CommandLine, NewThread},
    },
};
use memory::{Extensions, ExtensionsMut, Mem};
use std::collections::HashMap;

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
    pub fn new(host: Box<dyn host::Host>) -> Self {
        let mut memory = BoxMem::new(256 << 20);
        let retrowin32_syscall = b"\x0f\x34\xc3".as_slice(); // sysenter; ret
        let kernel32 = winapi::kernel32::State::new(&mut memory, retrowin32_syscall);
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
            external_dlls: Default::default(),
            status: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.emu.memory.mem()
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: Option<Option<u32>>,
    ) -> anyhow::Result<LoadedAddrs> {
        self.state
            .kernel32
            .init_process(self.emu.memory.mem(), CommandLine::new(cmdline));
        let exe = loader::load_exe(self, buf, &self.state.kernel32.cmdline.exe_name(), relocate)?;

        // Initialize process heap after exe has loaded, to ensure it doesn't occupy any addresses
        // that the exe wants.
        self.state.kernel32.init_process_heap(&mut self.emu.memory);

        let retrowin32_main = winapi::kernel32::get_kernel32_builtin(self, "retrowin32_main");

        let NewThread {
            thread,
            stack_pointer,
        } = create_thread(self, exe.stack_size);

        let cpu = self.emu.x86.new_cpu();
        cpu.regs.set32(x86::Register::ESP, stack_pointer);
        cpu.regs.set32(x86::Register::EBP, stack_pointer);
        cpu.regs.fs_addr = thread.teb;

        let mem = self.emu.memory.mem();
        x86::ops::push(cpu, mem, exe.entry_point);
        x86::ops::push(cpu, mem, 0); // return address
        cpu.regs.eip = retrowin32_main;

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    pub fn single_step(&mut self) {
        self.emu.x86.single_step_next_block(self.emu.memory.mem());
        self.run();
    }

    pub fn unblock_all(&mut self) {
        for cpu in self.emu.x86.cpus.iter_mut() {
            if matches!(
                cpu.state,
                x86::CPUState::Blocked(_) | x86::CPUState::DebugBreak
            ) {
                cpu.state = x86::CPUState::Running;
                self.status = Status::Running;
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
            self.status = Status::Running;
        }
    }

    pub fn run(&mut self) -> bool {
        self.emu.x86.schedule();
        match &self.emu.x86.cpu().state {
            x86::CPUState::Running => self.execute_block(),
            x86::CPUState::SysCall => self.syscall(),
            x86::CPUState::Blocked(wait) => {
                let wait = *wait;
                if self.host.block(wait) {
                    self.unblock();
                } else {
                    self.status = Status::Blocked;
                }
            }
            x86::CPUState::Error(message) => {
                self.status = Status::Error {
                    message: message.clone(),
                };
            }
            x86::CPUState::DebugBreak => {
                self.status = Status::DebugBreak;
            }
            state => unimplemented!("{state:?}"),
        }
        self.status.is_running()
    }

    fn execute_block(&mut self) {
        self.emu.x86.execute_block(self.emu.memory.mem())
    }

    fn syscall(&mut self) {
        self.emu.x86.cpu_mut().state = x86::CPUState::Running;

        // See doc/shims.md for the state of the stack when we get here.

        let regs = &mut self.emu.x86.cpu_mut().regs;

        // stack[0] is the return address within the shim DLL, after the call instruction.
        // The 'call retrowin32_syscall' instruction is ff15+addr, for 6 bytes.
        // Subtract to find the address of the shim function.
        let esp = regs.get32(x86::Register::ESP);
        let call_len = 6;
        let shim_addr = self.emu.memory.mem().get_pod::<u32>(esp) - call_len;

        let shim = match self.emu.shims.get(shim_addr) {
            Ok(shim) => shim,
            Err(name) => unimplemented!("{}", name),
        };

        let stack_args = esp + 8;
        match shim.func {
            Handler::Sync(func) => {
                let ret = unsafe { func(self, stack_args) };
                let regs = &mut self.emu.x86.cpu_mut().regs;
                regs.set32(x86::Register::EAX, ret as u32);
                regs.set32(x86::Register::EDX, (ret >> 32) as u32);

                // After call, attempt to clear registers to make execution traces easier to match.
                // eax: holds return value
                regs.set32(x86::Register::ECX, 0);
                // edx: sometimes used for 64-bit returns
                // ebx: callee-saved
            }

            Handler::Async(func) => {
                let eip = regs.eip; // return address
                let future = unsafe { func(self, stack_args) };
                self.emu.x86.cpu_mut().call_async(future, eip);
            }
        }
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        self.emu
            .x86
            .cpu_mut()
            .call_x86(self.emu.memory.mem(), func, args)
            .await
    }

    pub fn dump_stack(&self) {
        let esp = self.emu.x86.cpu().regs.get32(x86::Register::ESP);
        for addr in ((esp - 0x10)..(esp + 0x10)).step_by(4) {
            let extra = if addr == esp { " <- esp" } else { "" };
            println!(
                "{:08x} {:08x}{extra}",
                addr,
                self.mem().get_pod::<u32>(addr)
            );
        }
    }

    pub fn dump_state(&self, eip_offset: usize) {
        let cpu = self.emu.x86.cpu();
        x86::debug::dump_state(cpu, self.mem(), &self.labels, eip_offset);
        println!("stack:");
        self.dump_stack();
        x86::debug::dump_fpu_state(cpu);
    }

    /// Patch in an int3 over the instruction at that addr, backing up the current one.
    pub fn add_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.entry(addr) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mem = self.emu.memory.mem();
                entry.insert(mem.get_pod::<u8>(addr));
                mem.put_pod::<u8>(addr, 0xcc); // int3
                self.emu.x86.icache.clear_cache(addr);
                true
            }
        }
    }

    /// Undo an add_breakpoint().
    pub fn clear_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.remove(&addr) {
            Some(prev) => {
                self.emu.x86.icache.clear_cache(addr);
                self.mem().put_pod::<u8>(addr, prev);
                true
            }
            None => false,
        }
    }

    pub fn teb_addr(&self) -> u32 {
        self.emu.x86.cpu().regs.fs_addr
    }

    pub fn exit_thread(&mut self) {
        self.emu.x86.cpu_mut().state = x86::CPUState::Free;
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.status = Status::Exit(exit_code);
    }
}
