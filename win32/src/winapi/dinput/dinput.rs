use crate::winapi::{com::vtable, kernel32, stack_args};
use memory::ExtensionsMut;

use crate::Machine;

#[derive(Debug, Copy, Clone)]
pub enum DI {
    OK = 0,
}

impl stack_args::ToX86 for DI {
    fn to_raw(&self) -> u32 {
        *self as u32
    }
}

#[win32_derive::dllexport]
mod IDirectInput {
    use memory::Extensions;

    use super::*;

    pub fn new(machine: &mut Machine) -> u32 {
        let lpDirectInput = machine
            .state
            .kernel32
            .get_process_heap(&mut machine.emu.memory)
            .alloc(machine.emu.memory.mem(), 4);
        let vtable = kernel32::get_symbol(machine, "dinput.dll", "IDirectInput");
        for i in 0..0x20 {
            log::info!(
                "vtable+{i:x}: {vtable:x}",
                i = i * 4,
                vtable = machine.mem().get_pod::<u32>(vtable + i * 4)
            );
        }
        machine.mem().put_pod::<u32>(lpDirectInput, vtable);
        lpDirectInput
    }

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: todo,

        CreateDevice: todo,
        EnumDevices: todo,
        GetDeviceStatus: todo,
        RunControlPanel: todo,
        Initialize: todo,
    ];
}

#[win32_derive::dllexport]
pub fn DirectInputCreateA(
    machine: &mut Machine,
    version: u32,
    ppDI: Option<&mut u32>,
    pUnkOuter: u32,
) -> DI {
    let c = IDirectInput::new(machine);
    *ppDI.unwrap() = IDirectInput::new(machine);

    DI::OK
}
