use crate::{
    host,
    machine::{LoadedAddrs, MachineX, Status},
    pe,
    shims::Shims,
    shims_raw::retrowin32_syscall,
    winapi,
};
use memory::Mem;
use std::collections::HashMap;

#[derive(Default)]
pub struct RawMem {}

impl RawMem {
    pub fn mem(&self) -> Mem {
        Mem::from_ptrs(0 as *mut u8..(1 << 30) as *mut u8)
    }
    pub fn len(&self) -> u32 {
        0xFFFF_FFFF
    }
}

pub struct Emulator {
    pub shims: Shims,
    pub memory: RawMem,
}

pub type MemImpl = RawMem;
pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::default();
        let kernel32 = winapi::kernel32::State::new(&mut memory, cmdline, &retrowin32_syscall());
        let shims = Shims::new(kernel32.teb);
        let state = winapi::State::new(&mut memory, kernel32);

        Machine {
            emu: Emulator { shims, memory },
            host,
            state,
            labels: HashMap::new(),
            exe_path: Default::default(),
            external_dlls: Default::default(),
            status: Default::default(),
        }
    }

    pub fn mem(&self) -> Mem {
        self.emu.memory.mem()
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        path: &std::path::Path,
        relocate: Option<Option<u32>>,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, path, relocate)?;

        let stack = self.state.kernel32.mappings.alloc(
            exe.stack_size,
            "stack".into(),
            &mut self.emu.memory,
        );
        let stack_pointer = stack.addr + stack.size - 4;

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }

    pub async fn call_x86(&mut self, func: u32, args: Vec<u32>) -> u32 {
        crate::shims_raw::call_x86(self, func, args).await
    }

    /// Transfer control to the executable's entry point.
    /// Needs to switch code segments to enter compatibility mode, stacks, etc.
    #[inline(never)] // aid in debugging
    pub fn jump_to_entry_point(&mut self, entry_point: u32) {
        // Assert that our code was loaded in the 3-4gb memory range, which means
        // that calls from/to it can be managed with 32-bit pointers.
        // (This arrangement is set up by the linker flags.)
        let mem_3gb_range = 0xc000_0000u64..0x1_0000_0000u64;
        let fn_addr = &Self::jump_to_entry_point as *const _ as u64;
        assert!(mem_3gb_range.contains(&fn_addr));

        println!("entry point at {:x}, about to jump", entry_point);
        std::io::stdin().read_line(&mut String::new()).unwrap();

        let pin = std::pin::pin!(self.call_x86(entry_point, vec![]));
        crate::shims::call_sync(pin);
    }

    pub fn exit(&mut self, exit_code: u32) {
        self.status = Status::Exit(exit_code);
    }
}
