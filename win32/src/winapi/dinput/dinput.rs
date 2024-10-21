use crate::winapi::{com::vtable, kernel32, stack_args};
use crate::Machine;
use memory::ExtensionsMut;

pub use crate::winapi::com::GUID;

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
        AddRef: ok,
        Release: ok,

        CreateDevice: ok,
        EnumDevices: ok,
        GetDeviceStatus: todo,
        RunControlPanel: todo,
        Initialize: todo,
    ];

    #[win32_derive::dllexport]
    pub fn CreateDevice(
        machine: &mut Machine,
        this: u32,
        lpGUID: Option<&GUID>,
        lplpDirectInputDevice: Option<&mut u32>,
        pUnkOuter: u32,
    ) -> DI {
        *lplpDirectInputDevice.unwrap() = IDirectInputDevice::new(machine);
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn AddRef(machine: &mut Machine, this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(machine: &mut Machine, this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn EnumDevices(
        machine: &mut Machine,
        this: u32,
        dwDevType: u32,
        callback: u32,
        pvRef: u32,
        dwFlags: u32,
    ) -> DI {
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
        EnumObjects: ok,
        GetProperty: todo,
        SetProperty: ok,
        Acquire: todo,
        Unacquire: todo,
        GetDeviceState: todo,
        GetDeviceData: todo,
        SetDataFormat: ok,
        SetEventNotification: ok,
        SetCooperativeLevel: todo,
        GetObjectInfo: todo,
        GetDeviceInfo: todo,
        RunControlPanel: todo,
        Initialize: todo,
    ];

    #[win32_derive::dllexport]
    pub fn EnumObjects(
        machine: &mut Machine,
        this: u32,
        lpCallback: u32,
        pvRef: u32,
        dwFlag: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDataFormat(machine: &mut Machine, this: u32, lpdf: u32) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetProperty(
        machine: &mut Machine,
        this: u32,
        rguidProp: Option<&GUID>,
        pdiph: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetEventNotification(machine: &mut Machine, this: u32, hEvent: u32) -> DI {
        DI::OK
    }
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
