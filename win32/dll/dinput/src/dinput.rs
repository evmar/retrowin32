use win32_system::System;
use win32_winapi::{HWND, calling_convention, vtable};

pub use win32_winapi::com::GUID;

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

impl Into<calling_convention::ABIReturn> for DI {
    fn into(self) -> calling_convention::ABIReturn {
        (self as u32).into()
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

    pub fn new(sys: &mut dyn System) -> u32 {
        let vtable = sys.get_symbol("dinput.dll", "IDirectInput");
        sys.memory().store(vtable)
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
        sys: &mut dyn System,
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
        *lplpDirectInputDevice.unwrap() = IDirectInputDevice::new(sys);
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn AddRef(sys: &mut dyn System, this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &mut dyn System, this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn EnumDevices(
        sys: &mut dyn System,
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

    pub fn new(sys: &mut dyn System) -> u32 {
        let vtable = sys.get_symbol("dinput.dll", "IDirectInputDevice");
        sys.memory().store(vtable)
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
        sys: &mut dyn System,
        this: u32,
        lpCallback: u32,
        pvRef: u32,
        dwFlag: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetProperty(
        sys: &mut dyn System,
        this: u32,
        rguidProp: Option<&GUID>,
        pdiph: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn Acquire(sys: &mut dyn System, this: u32) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceData(
        sys: &mut dyn System,
        this: u32,
        cbObjectData: u32,
        rgdod: u32,
        pdwInOut: Option<&mut u32>,
        dwFlags: u32,
    ) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetDataFormat(sys: &mut dyn System, this: u32, lpdf: Option<&DIDATAFORMAT>) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetEventNotification(sys: &mut dyn System, this: u32, hEvent: u32) -> DI {
        DI::OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(sys: &mut dyn System, this: u32, hwnd: HWND, dwFlags: u32) -> DI {
        DI::OK
    }
}

#[win32_derive::dllexport]
pub fn DirectInputCreateA(
    sys: &mut dyn System,
    hinst: u32,
    version: u32,
    ppDI: Option<&mut u32>,
    pUnkOuter: u32,
) -> DI {
    *ppDI.unwrap() = IDirectInput::new(sys);

    DI::OK
}
