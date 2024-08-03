use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims_unicorn::Shims,
    winapi,
};
use memory::Mem;
use std::collections::HashMap;

pub struct MemImpl(Box<[u8]>);

impl MemImpl {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Self(v.into_boxed_slice())
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
    pub unicorn: unicorn_engine::Unicorn<'static, ()>,
    pub shims: Shims,
    pub memory: MemImpl,
}

pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::new(32 << 20);
        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);

        let mut unicorn = unicorn_engine::Unicorn::new(
            unicorn_engine::unicorn_const::Arch::X86,
            unicorn_engine::unicorn_const::Mode::MODE_32,
        )
        .unwrap();
        unsafe {
            unicorn
                .mem_map_ptr(
                    0,
                    memory.len() as usize,
                    unicorn_engine::unicorn_const::Permission::ALL,
                    memory.ptr() as *mut _,
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
            },
            host,
            state,
            labels: HashMap::new(),
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
            .reg_write(unicorn_engine::RegisterX86::ESP, stack_pointer as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EBP, stack_pointer as u64)
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

        let gdtr = unicorn_engine::X86Mmr {
            selector: 0, // unused
            base: gdt.addr as u64,
            limit: 5 * 8,
            flags: 0, // unused
        };
        // Gross: need gdtr as a slice to pass to reg_write_long.
        let gdtr_slice = unsafe {
            std::slice::from_raw_parts(
                &gdtr as *const _ as *const u8,
                std::mem::size_of::<unicorn_engine::X86Mmr>(),
            )
        };

        self.emu
            .unicorn
            .reg_write_long(unicorn_engine::RegisterX86::GDTR, gdtr_slice)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::CS, gdt.cs as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::DS, gdt.ds as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::SS, gdt.ss as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::FS, gdt.fs as u64)
            .unwrap();
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        filename: &str,
        relocate: Option<Option<u32>>,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, filename, relocate)?;

        let stack_pointer = self.setup_stack(exe.stack_size);
        self.setup_segments();

        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EAX, 0xdeadbeea)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EBX, 0xdeadbeeb)
            .unwrap();
        // To make CPU traces match more closely, set up some registers to what their
        // initial values appear to be from looking in a debugger.
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::ECX, exe.entry_point as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EDX, exe.entry_point as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::ESI, exe.entry_point as u64)
            .unwrap();
        self.emu
            .unicorn
            .reg_write(unicorn_engine::RegisterX86::EDI, exe.entry_point as u64)
            .unwrap();

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        crate::shims_unicorn::call_x86(self, func, args).await
    }
}
