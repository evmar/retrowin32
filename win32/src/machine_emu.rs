use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims_emu::Shims,
    winapi,
};
use memory::Mem;
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct VecMem(#[serde(with = "serde_bytes")] Vec<u8>);

impl VecMem {
    pub fn resize(&mut self, size: u32, value: u8) {
        self.0.resize(size as usize, value);
    }
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
    pub fn mem(&self) -> Mem {
        Mem::from_slice(&self.0)
    }
}

pub struct Emulator {
    pub x86: x86::X86,
    pub shims: Shims,
}

impl crate::machine::Emulator for Emulator {
    fn register(&mut self, shim: Result<&'static crate::shims::Shim, String>) -> u32 {
        self.shims.add(shim)
    }
}

pub type MemImpl = VecMem;
pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::default();
        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);
        let shims = {
            kernel32 = kernel32;
            Shims::new()
        };
        let state = winapi::State::new(kernel32);

        Machine {
            emu: Emulator {
                x86: x86::X86::new(),
                shims,
            },
            memory,
            host,
            state,
            labels: HashMap::new(),
        }
    }

    /// Initialize a memory mapping for the stack and return the initial stack pointer.
    fn setup_stack(&mut self, stack_size: u32) -> u32 {
        let stack =
            self.state
                .kernel32
                .mappings
                .alloc(stack_size, "stack".into(), &mut self.memory);
        let stack_pointer = stack.addr + stack.size - 4;
        self.emu.x86.cpu.regs.esp = stack_pointer;
        self.emu.x86.cpu.regs.ebp = stack_pointer;

        stack_pointer
    }

    #[allow(non_snake_case)]
    async fn invoke_dllmains(&mut self, dllmains: Vec<u32>) {
        for dll_main in dllmains {
            log::info!("invoking dllmain {:x}", dll_main);
            let hInstance = 0u32; // TODO
            let fdwReason = 1u32; // DLL_PROCESS_ATTACH
            let lpvReserved = 0u32;
            crate::shims::call_x86(self, dll_main, vec![hInstance, fdwReason, lpvReserved]).await;
        }
    }

    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: bool,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, cmdline, relocate)?;

        let stack_pointer = self.setup_stack(exe.stack_size);
        self.emu.x86.cpu.regs.fs_addr = self.state.kernel32.teb;

        // To make CPU traces match more closely, set up some registers to what their
        // initial values appear to be from looking in a debugger.
        self.emu.x86.cpu.regs.ecx = exe.entry_point;
        self.emu.x86.cpu.regs.edx = exe.entry_point;
        self.emu.x86.cpu.regs.esi = exe.entry_point;
        self.emu.x86.cpu.regs.edi = exe.entry_point;

        let mut dllmains = Vec::new();
        for dll in &self.state.kernel32.dlls {
            if dll.dll.entry_point != 0 {
                dllmains.push(dll.dll.entry_point);
            }
        }

        if dllmains.is_empty() {
            self.emu.x86.cpu.regs.eip = exe.entry_point;
        } else {
            // Invoke any DllMains then jump to the entry point.
            let m = self as *mut Machine;
            crate::shims::become_async(
                self,
                Box::pin(async move {
                    let machine = unsafe { &mut *m };
                    machine.invoke_dllmains(dllmains).await;
                    machine.emu.x86.cpu.regs.eip = exe.entry_point;
                }),
            );
        };

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    /// If eip points at a shim address, call the handler and update eip.
    fn check_shim_call(&mut self) -> anyhow::Result<bool> {
        if self.emu.x86.cpu.regs.eip & 0xFFFF_0000 != crate::shims_emu::SHIM_BASE {
            return Ok(false);
        }
        let shim = match self.emu.shims.get(self.emu.x86.cpu.regs.eip) {
            Ok(shim) => shim,
            Err(name) => unimplemented!("{}", name),
        };
        let crate::shims::Shim {
            func,
            stack_consumed,
            is_async,
            ..
        } = *shim;
        let ret = unsafe { func(self, self.emu.x86.cpu.regs.esp) };
        if !is_async {
            self.emu.x86.cpu.regs.eip = self.mem().get::<u32>(self.emu.x86.cpu.regs.esp);
            self.emu.x86.cpu.regs.esp += stack_consumed;
            self.emu.x86.cpu.regs.eax = ret;
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
        self.emu
            .x86
            .execute_block(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }

    pub fn single_step(&mut self) -> anyhow::Result<()> {
        if self.check_shim_call()? {
            // Treat any shim call as a single block.
            return Ok(());
        }
        self.emu
            .x86
            .single_step(self.memory.mem())
            .map_err(|err| anyhow::anyhow!(err))
    }
}
