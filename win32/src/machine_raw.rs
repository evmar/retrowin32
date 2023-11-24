use crate::{
    host,
    machine::{LoadedAddrs, MachineX},
    pe,
    shims_raw::Shims,
    winapi,
};
use memory::Mem;
use std::collections::HashMap;

#[derive(Default)]
pub struct RawMem {}

impl RawMem {
    pub fn mem(&self) -> Mem {
        let s = unsafe { std::slice::from_raw_parts(0 as *const u8, 1 << 30) };
        Mem::from_slice(s)
    }
    pub fn len(&self) -> u32 {
        0xFFFF_FFFF
    }
    pub fn resize(&mut self, _size: u32, _value: u8) {
        unreachable!()
    }
}

pub struct Emulator {
    pub shims: Shims,
}

impl crate::machine::Emulator for Emulator {
    fn register(&mut self, shim: Result<&'static crate::shims::Shim, String>) -> u32 {
        self.shims.add(shim)
    }
}

pub type MemImpl = RawMem;
pub type Machine = MachineX<Emulator>;

impl MachineX<Emulator> {
    pub fn new(host: Box<dyn host::Host>, cmdline: String) -> Self {
        let mut memory = MemImpl::default();
        let mut kernel32 = winapi::kernel32::State::new(&mut memory, cmdline);
        let mapping = kernel32
            .mappings
            .alloc(0x4000, "shims x64 trampoline".into(), &mut memory);
        let shims = Shims::new(
            &mut kernel32.ldt,
            mapping.addr as u64 as *mut u8,
            mapping.size,
        );
        let state = winapi::State::new(kernel32);

        Machine {
            emu: Emulator { shims },
            memory,
            host,
            state,
            labels: HashMap::new(),
        }
    }

    #[allow(non_snake_case)]
    pub fn load_exe(
        &mut self,
        buf: &[u8],
        cmdline: String,
        relocate: bool,
    ) -> anyhow::Result<LoadedAddrs> {
        let exe = pe::load_exe(self, buf, cmdline, relocate)?;

        let stack =
            self.state
                .kernel32
                .mappings
                .alloc(exe.stack_size, "stack".into(), &mut self.memory);
        let stack_pointer = stack.addr + stack.size - 4;

        Ok(LoadedAddrs {
            entry_point: exe.entry_point,
            stack_pointer,
        })
    }
}
