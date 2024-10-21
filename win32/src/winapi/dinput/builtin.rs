#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        machine::Machine,
        winapi::{self, stack_args::*, types::*},
    };
    use ::memory::Extensions;
    use winapi::dinput::*;
    pub unsafe fn DirectInputCreateA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let hinst = <u32>::from_stack(mem, stack_args + 0u32);
        let version = <u32>::from_stack(mem, stack_args + 4u32);
        let ppDI = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::DirectInputCreateA_pos,
                "dinput/dinput",
                "DirectInputCreateA",
                &[
                    ("hinst", &hinst),
                    ("version", &version),
                    ("ppDI", &ppDI),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::DirectInputCreateA(machine, hinst, version, ppDI, pUnkOuter);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInputDevice_EnumObjects(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpCallback = <u32>::from_stack(mem, stack_args + 4u32);
        let pvRef = <u32>::from_stack(mem, stack_args + 8u32);
        let dwFlag = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInputDevice::EnumObjects_pos,
                "dinput/dinput",
                "IDirectInputDevice::EnumObjects",
                &[
                    ("this", &this),
                    ("lpCallback", &lpCallback),
                    ("pvRef", &pvRef),
                    ("dwFlag", &dwFlag),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::IDirectInputDevice::EnumObjects(
            machine, this, lpCallback, pvRef, dwFlag,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInputDevice_SetDataFormat(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpdf = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInputDevice::SetDataFormat_pos,
                "dinput/dinput",
                "IDirectInputDevice::SetDataFormat",
                &[("this", &this), ("lpdf", &lpdf)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::IDirectInputDevice::SetDataFormat(machine, this, lpdf);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInputDevice_SetEventNotification(
        machine: &mut Machine,
        stack_args: u32,
    ) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let hEvent = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInputDevice::SetEventNotification_pos,
                "dinput/dinput",
                "IDirectInputDevice::SetEventNotification",
                &[("this", &this), ("hEvent", &hEvent)],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::dinput::IDirectInputDevice::SetEventNotification(machine, this, hEvent);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInputDevice_SetProperty(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let rguidProp = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
        let pdiph = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInputDevice::SetProperty_pos,
                "dinput/dinput",
                "IDirectInputDevice::SetProperty",
                &[
                    ("this", &this),
                    ("rguidProp", &rguidProp),
                    ("pdiph", &pdiph),
                ],
            )
            .enter()
        } else {
            None
        };
        let result =
            winapi::dinput::IDirectInputDevice::SetProperty(machine, this, rguidProp, pdiph);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInput_AddRef(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInput::AddRef_pos,
                "dinput/dinput",
                "IDirectInput::AddRef",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::IDirectInput::AddRef(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInput_CreateDevice(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpGUID = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
        let lplpDirectInputDevice = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInput::CreateDevice_pos,
                "dinput/dinput",
                "IDirectInput::CreateDevice",
                &[
                    ("this", &this),
                    ("lpGUID", &lpGUID),
                    ("lplpDirectInputDevice", &lplpDirectInputDevice),
                    ("pUnkOuter", &pUnkOuter),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::IDirectInput::CreateDevice(
            machine,
            this,
            lpGUID,
            lplpDirectInputDevice,
            pUnkOuter,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInput_EnumDevices(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let dwDevType = <u32>::from_stack(mem, stack_args + 4u32);
        let callback = <u32>::from_stack(mem, stack_args + 8u32);
        let pvRef = <u32>::from_stack(mem, stack_args + 12u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInput::EnumDevices_pos,
                "dinput/dinput",
                "IDirectInput::EnumDevices",
                &[
                    ("this", &this),
                    ("dwDevType", &dwDevType),
                    ("callback", &callback),
                    ("pvRef", &pvRef),
                    ("dwFlags", &dwFlags),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::IDirectInput::EnumDevices(
            machine, this, dwDevType, callback, pvRef, dwFlags,
        );
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
    pub unsafe fn IDirectInput_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("dinput/dinput") {
            crate::trace::Record::new(
                winapi::dinput::IDirectInput::Release_pos,
                "dinput/dinput",
                "IDirectInput::Release",
                &[("this", &this)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::dinput::IDirectInput::Release(machine, this);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 9usize] = [
    Shim {
        name: "DirectInputCreateA",
        func: Handler::Sync(wrappers::DirectInputCreateA),
    },
    Shim {
        name: "IDirectInputDevice::EnumObjects",
        func: Handler::Sync(wrappers::IDirectInputDevice_EnumObjects),
    },
    Shim {
        name: "IDirectInputDevice::SetDataFormat",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetDataFormat),
    },
    Shim {
        name: "IDirectInputDevice::SetEventNotification",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetEventNotification),
    },
    Shim {
        name: "IDirectInputDevice::SetProperty",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetProperty),
    },
    Shim {
        name: "IDirectInput::AddRef",
        func: Handler::Sync(wrappers::IDirectInput_AddRef),
    },
    Shim {
        name: "IDirectInput::CreateDevice",
        func: Handler::Sync(wrappers::IDirectInput_CreateDevice),
    },
    Shim {
        name: "IDirectInput::EnumDevices",
        func: Handler::Sync(wrappers::IDirectInput_EnumDevices),
    },
    Shim {
        name: "IDirectInput::Release",
        func: Handler::Sync(wrappers::IDirectInput_Release),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "dinput.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/dinput.dll"),
};
