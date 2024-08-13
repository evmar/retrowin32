use crate::shims::{BoxFuture, Shim};
use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims_unicorn::Shims,
    winapi, StopReason,
};
use memory::Mem;
use std::collections::{hash_map, HashMap};
use std::path::Path;
use std::pin::Pin;
use unicorn_engine::unicorn_const::{uc_error, Arch, Mode, Permission};
use unicorn_engine::{RegisterX86, Unicorn, X86Mmr};

pub struct MemImpl(Pin<Box<[u8]>>);

impl MemImpl {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Self(Pin::from(v.into_boxed_slice()))
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
    pub fn mem(&self) -> Mem {
        Mem::from_slice(&self.0)
    }

    pub fn ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

pub struct Emulator {
    pub unicorn: Unicorn<'static, ()>,
    pub shims: Shims,
    pub memory: MemImpl,
    breakpoints: HashMap<u32, *mut core::ffi::c_void>,
    exit_code: Option<u32>,
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::new(32 << 20);
        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);

        let mut unicorn = Unicorn::new(Arch::X86, Mode::MODE_32).unwrap();
        unsafe {
            let offset = 0x1000usize; // Leave the first 4k of memory unmapped
            unicorn
                .mem_map_ptr(
                    offset as u64,
                    (memory.len() as usize) - offset,
                    Permission::ALL,
                    memory.ptr().add(offset) as *mut _,
                )
                .unwrap();
        };

        let shims = {
            let mapping = kernel32
                .mappings
                .alloc(0x1000, "syscall hooks".into(), &mut memory);
            Shims::new(&mut unicorn, mapping.addr)
        };

        let state = winapi::State::new(&mut memory, kernel32);

        Machine {
            emu: Emulator {
                unicorn,
                shims,
                memory,
                breakpoints: Default::default(),
                exit_code: None,
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
    fn setup_stack(&mut self, stack_size: u32) -> u32 {
        let stack =
            self.state
                .kernel32
                .mappings
                .alloc(stack_size, "stack".into(), &mut self.emu.memory);
        let stack_pointer = stack.addr + stack.size - 4;

        // TODO: put this init somewhere better.
        self.emu
            .unicorn
            .reg_write(RegisterX86::ESP, stack_pointer as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EBP, stack_pointer as u64)
            .unwrap();

        stack_pointer
    }

    /// Initialize segment registers.  In particular, we need FS to point at the kernel32 TEB.
    fn setup_segments(&mut self) {
        // To be able to set FS, we need to create a GDT, which then requires entries
        // for the code and data segments too.
        // https://scoding.de/setting-global-descriptor-table-unicorn
        // https://github.com/unicorn-engine/unicorn/blob/master/samples/sample_x86_32_gdt_and_seg_regs.c
        let gdt = self.state.kernel32.create_gdt(self.emu.memory.mem());

        let gdtr = X86Mmr {
            selector: 0, // unused
            base: gdt.addr as u64,
            limit: 5 * 8,
            flags: 0, // unused
        };
        // Gross: need gdtr as a slice to pass to reg_write_long.
        let gdtr_slice = unsafe {
            std::slice::from_raw_parts(&gdtr as *const _ as *const u8, size_of::<X86Mmr>())
        };

        self.emu
            .unicorn
            .reg_write_long(RegisterX86::GDTR, gdtr_slice)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::CS, gdt.cs as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::DS, gdt.ds as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::SS, gdt.ss as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::FS, gdt.fs as u64)
            .unwrap();
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        path: &Path,
        relocate: Option<Option<u32>>,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, path, relocate)?;

        let stack_pointer = self.setup_stack(exe.stack_size);
        self.setup_segments();

        self.emu
            .unicorn
            .reg_write(RegisterX86::EAX, 0xdeadbeea)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EBX, 0xdeadbeeb)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EIP, exe.entry_point as u64)
            .unwrap();
        // To make CPU traces match more closely, set up some registers to what their
        // initial values appear to be from looking in a debugger.
        self.emu
            .unicorn
            .reg_write(RegisterX86::ECX, exe.entry_point as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EDX, exe.entry_point as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::ESI, exe.entry_point as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(RegisterX86::EDI, exe.entry_point as u64)
            .unwrap();

        self.exe_path = path.to_path_buf();
        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        crate::shims_unicorn::call_x86(self, func, args).await
    }

    /// Call a shim function. If it returns a future, it will be scheduled to run.
    pub fn call_shim(&mut self, shim: &'static Shim) -> Option<BoxFuture<u32>> {
        crate::shims_unicorn::handle_shim_call(self, shim)
    }

    /// Finish a shim call. This will set the return value and pop the stack.
    pub fn finish_shim_call(&mut self, shim: &'static Shim, ret: u32) {
        crate::shims_unicorn::finish_shim_call(self, shim, ret)
    }

    pub fn run(&mut self, mut instruction_count: usize) -> StopReason {
        if let Some(code) = self.emu.exit_code {
            return StopReason::Exit { code };
        }
        let eip = self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap();
        if eip == crate::shims_unicorn::MAGIC_ADDR as u64 {
            return StopReason::Blocked;
        }
        if instruction_count == 0 {
            // Insert some reasonable number of instructions per loop.
            instruction_count = 10_000;
        }
        match self.emu.unicorn.emu_start(eip, 0, 0, instruction_count) {
            Ok(()) => {}
            Err(e) => {
                let eip = self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap() as u32;
                return StopReason::Error {
                    message: format!("{:?}", e),
                    signal: match e {
                        uc_error::NOMEM => 6,        // SIGABRT
                        uc_error::INSN_INVALID => 4, // SIGILL
                        _ => 11,                     // SIGSEGV
                    },
                    eip,
                };
            }
        }
        let eip = self.emu.unicorn.reg_read(RegisterX86::EIP).unwrap() as u32;
        if self.emu.breakpoints.contains_key(&eip) {
            StopReason::Breakpoint { eip }
        } else if let Some(result) = self.emu.shims.get(eip) {
            match result {
                Ok(shim) => StopReason::ShimCall(shim),
                Err(name) => StopReason::Error {
                    message: format!("unimplemented shim {name}"),
                    signal: 31, // SIGSYS
                    eip,
                },
            }
        } else {
            StopReason::None
        }
    }

    pub fn add_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.entry(addr) {
            hash_map::Entry::Occupied(_) => {
                log::warn!("machine_unicorn: breakpoint already set at {:#x}", addr);
                false
            }
            hash_map::Entry::Vacant(entry) => {
                let hook = self
                    .emu
                    .unicorn
                    .add_code_hook(addr as u64, (addr + 1) as u64, |u, addr, _size| {
                        log::debug!("machine_unicorn: breakpoint hit at {:#x}", addr);
                        u.emu_stop().unwrap()
                    })
                    .unwrap();
                entry.insert(hook);
                true
            }
        }
    }

    pub fn clear_breakpoint(&mut self, addr: u32) -> bool {
        if let Some(hook) = self.emu.breakpoints.remove(&addr) {
            self.emu.unicorn.remove_hook(hook).unwrap();
            true
        } else {
            log::warn!("machine_unicorn: no breakpoint at {:#x}", addr);
            false
        }
    }

    pub fn exit(&mut self, code: u32) {
        self.emu.exit_code = Some(code);
    }
}
