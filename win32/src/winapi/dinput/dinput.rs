use crate::winapi::{com::vtable, kernel32, stack_args};
use crate::Machine;
use memory::ExtensionsMut;

pub use crate::winapi::com::GUID;

const ENABLE: bool = true;

const GUID_SysMouse: GUID = GUID((
    0x6F1D2B60,
    0xD5A0,
    0x11CF,
    [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
));

const GUID_SysKeyboard: GUID = GUID((
    0x6F1D2B61,
    0xD5A0,
    0x11CF,
    [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
));

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum DI {
    OK = 0,
    ERR_DEVICENOTREG = 0x80040154,
}

impl stack_args::ToX86 for DI {
    fn to_raw(&self) -> u32 {
        *self as u32
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DIDATAFORMAT {
    dwSize: u32,
    dwObjSize: u32,
    dwFlags: u32,
    dwDataSize: u32,
    dwNumObjs: u32,
    rgodf: u32,
}
unsafe impl ::memory::Pod for DIDATAFORMAT {}

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
        if !ENABLE {
            return DI::ERR_DEVICENOTREG;
        }
        match lpGUID.unwrap() {
            &GUID_SysMouse => {}
            &GUID_SysKeyboard => {}
            _ => return DI::ERR_DEVICENOTREG,
        }
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
    use crate::winapi::types::HWND;

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
        Acquire: ok,
        Unacquire: todo,
        GetDeviceState: todo,
        GetDeviceData: ok,
        SetDataFormat: ok,
        SetEventNotification: ok,
        SetCooperativeLevel: ok,
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
    pub fn SetProperty(
        machine: &mut Machine,
        this: u32,
        rguidProp: Option<&GUID>,
        pdiph: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn Acquire(machine: &mut Machine, this: u32) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceData(
        machine: &mut Machine,
        this: u32,
        cbObjectData: u32,
        rgdod: u32,
        pdwInOut: Option<&mut u32>,
        dwFlags: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDataFormat(machine: &mut Machine, this: u32, lpdf: Option<&DIDATAFORMAT>) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetEventNotification(machine: &mut Machine, this: u32, hEvent: u32) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(machine: &mut Machine, this: u32, hwnd: HWND, dwFlags: u32) -> DI {
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
