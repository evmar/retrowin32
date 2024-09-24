use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims::{Handler, Shims},
    winapi,
};
use memory::{Extensions, ExtensionsMut, Mem};
use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use unicorn_engine::unicorn_const::{Arch, Mode, Permission};
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
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::new(32 << 20);
        let retrowin32_syscall = b"\x0f\x34\xc3".as_slice(); // sysenter; ret
        let kernel32 = winapi::kernel32::State::new(&mut memory, cmdline, retrowin32_syscall);

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

        unicorn
            .add_insn_sys_hook(
                unicorn_engine::InsnSysX86::SYSENTER,
                0,
                0xFFFF_FFFF,
                |unicorn| {
                    unicorn.emu_stop().unwrap();
                },
            )
            .unwrap();

        // Uncomment to trace every executed instruction:
        unicorn
            .add_code_hook(0, 0xFFFF_FFFF, |_unicorn, addr, size| {
                println!("u {addr:x}+{size:x}");
            })
            .unwrap();

        let state = winapi::State::new(&mut memory, kernel32);

        Machine {
            emu: Emulator {
                unicorn,
                shims: Shims::default(),
                memory,
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
            std::slice::from_raw_parts(
                &gdtr as *const _ as *const u8,
                std::mem::size_of::<X86Mmr>(),
            )
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

    fn handle_shim_call(&mut self) {
        // See doc/shims.md for the state of the stack when we get here.

        // address of shim = return address - length of call instruction
        let esp = self
            .emu
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::ESP)
            .unwrap() as u32;
        let addr = self.emu.memory.mem().get_pod::<u32>(esp) - 6;
        let shim = match self.emu.shims.get(addr) {
            Ok(shim) => shim,
            Err(name) => unimplemented!("shim call to {name}"),
        };
        let name = shim.name;

        let func = match shim.func {
            Handler::Sync(func) => func,
            Handler::Async(_) => unimplemented!("async shim {name}"),
        };
        let stack_args = esp + 8;
        let ret = unsafe { func(self, stack_args) };

        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EAX, ret as u64)
            .unwrap();
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        let mem = self.emu.memory.mem();

        let ret_addr = self
            .emu
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EIP)
            .unwrap() as u32;

        let mut esp = self
            .emu
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::ESP)
            .unwrap() as u32;
        for &arg in args.iter().rev() {
            esp -= 4;
            mem.put_pod::<u32>(esp, arg);
        }
        esp -= 4;
        mem.put_pod::<u32>(esp, ret_addr);

        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::ESP, esp as u64)
            .unwrap();

        unicorn_loop(self, func, ret_addr);

        self.emu
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EAX)
            .unwrap() as u32
    }

    pub fn add_breakpoint(&mut self, addr: u32) -> bool {
        match self.emu.breakpoints.entry(addr) {
            std::collections::hash_map::Entry::Occupied(_) => {
                log::warn!("machine_unicorn: breakpoint already set at {:#x}", addr);
                false
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
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
}

/// Run emulation via machine.emu starting from eip=begin until eip==until is hit.
/// This is like unicorn.emu_start() but handles shim calls as well.
pub fn unicorn_loop(machine: &mut Machine, begin: u32, until: u32) {
    let mut eip = begin as u64;
    let until = until as u64;
    loop {
        machine.emu.unicorn.emu_start(eip, until, 0, 0).unwrap();
        eip = machine
            .emu
            .unicorn
            .reg_read(unicorn_engine::RegisterX86::EIP)
            .unwrap();
        if eip == until {
            return;
        }
        machine.handle_shim_call();
    }
}
