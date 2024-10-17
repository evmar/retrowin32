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
pub mod IDirectInput {
    use super::*;

    pub fn new(machine: &mut Machine) -> u32 {
        let lpDirectInput = machine
            .state
            .kernel32
            .get_process_heap(&mut machine.emu.memory)
            .alloc(machine.emu.memory.mem(), 4);
        let vtable = kernel32::get_symbol(machine, "dinput.dll", "IDirectInput");
        machine.mem().put_pod::<u32>(lpDirectInput, vtable);
        lpDirectInput
    }

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: todo,

        CreateDevice: ok,
        EnumDevices: todo,
        GetDeviceStatus: todo,
        RunControlPanel: todo,
        Initialize: todo,
    ];

    #[win32_derive::dllexport]
    pub fn CreateDevice(
        machine: &mut Machine,
        this: u32,
        lplpDirectInputDevice: Option<&mut u32>,
        pUnkOuter: u32,
    ) -> DI {
        *lplpDirectInputDevice.unwrap() = IDirectInputDevice::new(machine);
        DI::OK
    }
}

#[win32_derive::dllexport]
pub mod IDirectInputDevice {
    use super::*;

    pub fn new(machine: &mut Machine) -> u32 {
        let lpDirectInputDevice = machine
            .state
            .kernel32
            .get_process_heap(&mut machine.emu.memory)
            .alloc(machine.emu.memory.mem(), 4);
        let vtable = kernel32::get_symbol(machine, "dinput.dll", "IDirectInputDevice");
        machine.mem().put_pod::<u32>(lpDirectInputDevice, vtable);
        lpDirectInputDevice
    }

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: todo,

        GetCapabilities: todo,
        EnumObjects: todo,
        GetProperty: todo,
        SetProperty: todo,
        Acquire: todo,
        Unacquire: todo,
        GetDeviceState: todo,
        GetDeviceData: todo,
        SetDataFormat: todo,
        SetEventNotification: todo,
        SetCooperativeLevel: todo,
        GetObjectInfo: todo,
        GetDeviceInfo: todo,
        RunControlPanel: todo,
        Initialize: todo,
    ];
}

#[win32_derive::dllexport]
pub fn DirectInputCreateA(
    machine: &mut Machine,
    hinst: u32,
    version: u32,
    ppDI: Option<&mut u32>,
    pUnkOuter: u32,
) -> DI {
    *ppDI.unwrap() = IDirectInput::new(machine);

    DI::OK
}
