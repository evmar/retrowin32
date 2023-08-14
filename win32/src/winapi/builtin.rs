#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#[doc = r" Generated code, do not edit."]
use crate::{
    machine::Machine,
    shims,
    winapi::{self, stack_args::*, types::*},
};
pub struct Symbol {
    pub name: &'static str,
    pub ordinal: Option<usize>,
    pub func: shims::Handler,
    pub stack_consumed: fn() -> u32,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
}
pub mod bass {
    use super::*;
    use winapi::bass::*;
    pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let arg1 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg2 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg3 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg4 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let arg1 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg2 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg3 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg4 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let arg5 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::bass::BASS_Start(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let arg1 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicPlay(machine, arg1);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let arg1 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_ChannelGetPosition(machine, arg1);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 5usize] = [
        Symbol {
            name: "BASS_Init",
            ordinal: None,
            func: BASS_Init,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "BASS_MusicLoad",
            ordinal: None,
            func: BASS_MusicLoad,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "BASS_Start",
            ordinal: None,
            func: BASS_Start,
            stack_consumed: || 0,
        },
        Symbol {
            name: "BASS_MusicPlay",
            ordinal: None,
            func: BASS_MusicPlay,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "BASS_ChannelGetPosition",
            ordinal: None,
            func: BASS_ChannelGetPosition,
            stack_consumed: || <u32>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        exports: &EXPORTS,
    };
}
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    pub unsafe fn DirectDrawCreate(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpGuid = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lplpDD = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpGuid = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lplpDD = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let iid = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            name: "DirectDrawCreate",
            ordinal: None,
            func: DirectDrawCreate,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "DirectDrawCreateEx",
            ordinal: None,
            func: DirectDrawCreateEx,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        exports: &EXPORTS,
    };
}
pub mod dsound {
    use super::*;
    use winapi::dsound::*;
    pub unsafe fn DirectSoundCreate(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _lpGuid = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let ppDS = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _pUnkOuter = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        name: "DirectSoundCreate",
        ordinal: Some(1usize),
        func: DirectSoundCreate,
        stack_consumed: || {
            <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
        },
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        exports: &EXPORTS,
    };
}
pub mod gdi32 {
    use super::*;
    use winapi::gdi32::*;
    pub unsafe fn GetStockObject(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _i = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetStockObject(machine, _i);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hdc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hGdiObj = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let handle = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _bytes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _out = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hdc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hdc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::DeleteDC(machine, hdc);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hdc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let x = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let y = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let cx = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let cy = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hdcSrc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let x1 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let y1 = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let rop = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hdcDest = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let xDest = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let yDest = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let wDest = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hDest = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hdcSrc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let xSrc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let ySrc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let wSrc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hSrc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let rop = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::StretchBlt(
            machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 7usize] = [
        Symbol {
            name: "GetStockObject",
            ordinal: None,
            func: GetStockObject,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "SelectObject",
            ordinal: None,
            func: SelectObject,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetObjectA",
            ordinal: None,
            func: GetObjectA,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "CreateCompatibleDC",
            ordinal: None,
            func: CreateCompatibleDC,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "DeleteDC",
            ordinal: None,
            func: DeleteDC,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "BitBlt",
            ordinal: None,
            func: BitBlt,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "StretchBlt",
            ordinal: None,
            func: StretchBlt,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        exports: &EXPORTS,
    };
}
pub mod kernel32 {
    use super::*;
    use winapi::kernel32::*;
    pub unsafe fn GetModuleHandleA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpModuleName = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpModuleName = <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<Str16>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpModuleName = <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<Str16>>::stack_consumed();
        let hModule = <Option<&mut HMODULE>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut HMODULE>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let filename = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryA(machine, filename);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpLibFileName = <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<Str16>>::stack_consumed();
        let hFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hModule = <HMODULE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HMODULE>::stack_consumed();
        let lpProcName = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let nStdHandle = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpFileName = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let dwDesiredAccess = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwShareMode = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition =
            <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::CreateFileA(
            machine,
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpFileName = <Option<Str16>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<Str16>>::stack_consumed();
        let dwDesiredAccess = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwShareMode = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition =
            <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::CreateFileW(
            machine,
            lpFileName,
            dwDesiredAccess,
            dwShareMode,
            lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::GetFileType(machine, hFile);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let lDistanceToMove = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpDistanceToMoveHigh =
            <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let dwMoveMethod = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetFilePointer(
            machine,
            hFile,
            lDistanceToMove,
            lpDistanceToMoveHigh,
            dwMoveMethod,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = <Option<&mut [u8]>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hFile = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = <Option<&[u8]>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&[u8]>>::stack_consumed();
        let lpNumberOfBytesWritten =
            <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hHeap = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwBytes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hHeap = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpMem = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hHeap = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpMem = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hHeap = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpMem = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwBytes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let flOptions =
            <Result<HeapCreateFlags, u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Result<HeapCreateFlags, u32>>::stack_consumed();
        let dwInitialSize = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwMaximumSize = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hHeap = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapDestroy(machine, hHeap);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpAddress = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwSize = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _flAllocationType = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _flProtec = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualAlloc(
            machine,
            lpAddress,
            dwSize,
            _flAllocationType,
            _flProtec,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpAddress = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwSize = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwFreeType = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lp = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let ucb = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lp = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let ucb = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let dwErrCode = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetLastError(machine, dwErrCode);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetLastError(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let uExitCode = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::ExitProcess(machine, uExitCode);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetACP(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetACP(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let CodePage = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _CodePage = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _lpCPInfo = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineA(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineW(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStrings(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _penv = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStringsW(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let name = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let buf = <Option<&mut [u8]>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hModule = <HMODULE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HMODULE>::stack_consumed();
        let filename = <Option<&mut [u8]>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hModule = <HMODULE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HMODULE>::stack_consumed();
        let _lpFilename = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _nSize = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpStartupInfo =
            <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut STARTUPINFOA>>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpStartupInfo =
            <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut STARTUPINFOA>>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let feature = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::IsDebuggerPresent(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentProcessId(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetTickCount(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpPerformanceCount =
            <Option<&mut LARGE_INTEGER>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut LARGE_INTEGER>>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpFrequency = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _time = <Option<&mut FILETIME>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut FILETIME>>::stack_consumed();
        let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetVersion(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpVersionInformation =
            <Option<&mut OSVERSIONINFO>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut OSVERSIONINFO>>::stack_consumed();
        let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetProcessHeap(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let uNumber = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetHandleCount(machine, uNumber);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let msg = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::OutputDebugStringA(machine, msg);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _dwSpinCount = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _lpTopLevelExceptionFilter = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _exceptionInfo = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::NtCurrentTeb(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut SLIST_HEADER>>::stack_consumed();
        let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let CodePage = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpMultiByteStr = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let cbMultiByte = <i32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <i32>::stack_consumed();
        let lpWideCharStr = <Option<&mut [u16]>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut [u16]>>::stack_consumed();
        let result = winapi::kernel32::MultiByteToWideChar(
            machine,
            CodePage,
            dwFlags,
            lpMultiByteStr,
            cbMultiByte,
            lpWideCharStr,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hConsoleOutput = <HFILE>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = <Option<&[u16]>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&[u16]>>::stack_consumed();
        let lpNumberOfCharsWritten =
            <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let _lpReserved = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentThreadId(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::TlsAlloc(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpTlsValue = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn CreateThread(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpThreadAttributes = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwStackSize = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpStartAddress = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpParameter = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwCreationFlags = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpThreadId = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::CreateThread(
            machine,
            lpThreadAttributes,
            dwStackSize,
            lpStartAddress,
            lpParameter,
            dwCreationFlags,
            lpThreadId,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _hThread = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _nPriority = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let addend = <Option<&mut u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let result = winapi::kernel32::InterlockedIncrement(machine, addend);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 69usize] = [
        Symbol {
            name: "GetModuleHandleA",
            ordinal: None,
            func: GetModuleHandleA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleHandleW",
            ordinal: None,
            func: GetModuleHandleW,
            stack_consumed: || <Option<Str16>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleHandleExW",
            ordinal: None,
            func: GetModuleHandleExW,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <Option<Str16>>::stack_consumed()
                    + <Option<&mut HMODULE>>::stack_consumed()
            },
        },
        Symbol {
            name: "LoadLibraryA",
            ordinal: None,
            func: LoadLibraryA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "LoadLibraryExW",
            ordinal: None,
            func: LoadLibraryExW,
            stack_consumed: || {
                <Option<Str16>>::stack_consumed()
                    + <HFILE>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetProcAddress",
            ordinal: None,
            func: GetProcAddress,
            stack_consumed: || <HMODULE>::stack_consumed() + <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "GetStdHandle",
            ordinal: None,
            func: GetStdHandle,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "CreateFileA",
            ordinal: None,
            func: CreateFileA,
            stack_consumed: || {
                <Option<&str>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<CreationDisposition, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HFILE>::stack_consumed()
            },
        },
        Symbol {
            name: "CreateFileW",
            ordinal: None,
            func: CreateFileW,
            stack_consumed: || {
                <Option<Str16>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<CreationDisposition, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HFILE>::stack_consumed()
            },
        },
        Symbol {
            name: "GetFileType",
            ordinal: None,
            func: GetFileType,
            stack_consumed: || <HFILE>::stack_consumed(),
        },
        Symbol {
            name: "SetFilePointer",
            ordinal: None,
            func: SetFilePointer,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "ReadFile",
            ordinal: None,
            func: ReadFile,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&mut [u8]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "WriteFile",
            ordinal: None,
            func: WriteFile,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&[u8]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapAlloc",
            ordinal: None,
            func: HeapAlloc,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapFree",
            ordinal: None,
            func: HeapFree,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapSize",
            ordinal: None,
            func: HeapSize,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapReAlloc",
            ordinal: None,
            func: HeapReAlloc,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapCreate",
            ordinal: None,
            func: HeapCreate,
            stack_consumed: || {
                <Result<HeapCreateFlags, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapDestroy",
            ordinal: None,
            func: HeapDestroy,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "VirtualAlloc",
            ordinal: None,
            func: VirtualAlloc,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "VirtualFree",
            ordinal: None,
            func: VirtualFree,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "IsBadReadPtr",
            ordinal: None,
            func: IsBadReadPtr,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "IsBadWritePtr",
            ordinal: None,
            func: IsBadWritePtr,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetLastError",
            ordinal: None,
            func: SetLastError,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetLastError",
            ordinal: None,
            func: GetLastError,
            stack_consumed: || 0,
        },
        Symbol {
            name: "ExitProcess",
            ordinal: None,
            func: ExitProcess,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetACP",
            ordinal: None,
            func: GetACP,
            stack_consumed: || 0,
        },
        Symbol {
            name: "IsValidCodePage",
            ordinal: None,
            func: IsValidCodePage,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetCPInfo",
            ordinal: None,
            func: GetCPInfo,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetCommandLineA",
            ordinal: None,
            func: GetCommandLineA,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetCommandLineW",
            ordinal: None,
            func: GetCommandLineW,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetEnvironmentStrings",
            ordinal: None,
            func: GetEnvironmentStrings,
            stack_consumed: || 0,
        },
        Symbol {
            name: "FreeEnvironmentStringsA",
            ordinal: None,
            func: FreeEnvironmentStringsA,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetEnvironmentStringsW",
            ordinal: None,
            func: GetEnvironmentStringsW,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetEnvironmentVariableA",
            ordinal: None,
            func: GetEnvironmentVariableA,
            stack_consumed: || {
                <Option<&str>>::stack_consumed() + <Option<&mut [u8]>>::stack_consumed()
            },
        },
        Symbol {
            name: "GetModuleFileNameA",
            ordinal: None,
            func: GetModuleFileNameA,
            stack_consumed: || <HMODULE>::stack_consumed() + <Option<&mut [u8]>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleFileNameW",
            ordinal: None,
            func: GetModuleFileNameW,
            stack_consumed: || {
                <HMODULE>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetStartupInfoA",
            ordinal: None,
            func: GetStartupInfoA,
            stack_consumed: || <Option<&mut STARTUPINFOA>>::stack_consumed(),
        },
        Symbol {
            name: "GetStartupInfoW",
            ordinal: None,
            func: GetStartupInfoW,
            stack_consumed: || <Option<&mut STARTUPINFOA>>::stack_consumed(),
        },
        Symbol {
            name: "IsProcessorFeaturePresent",
            ordinal: None,
            func: IsProcessorFeaturePresent,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "IsDebuggerPresent",
            ordinal: None,
            func: IsDebuggerPresent,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetCurrentProcessId",
            ordinal: None,
            func: GetCurrentProcessId,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetTickCount",
            ordinal: None,
            func: GetTickCount,
            stack_consumed: || 0,
        },
        Symbol {
            name: "QueryPerformanceCounter",
            ordinal: None,
            func: QueryPerformanceCounter,
            stack_consumed: || <Option<&mut LARGE_INTEGER>>::stack_consumed(),
        },
        Symbol {
            name: "QueryPerformanceFrequency",
            ordinal: None,
            func: QueryPerformanceFrequency,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetSystemTimeAsFileTime",
            ordinal: None,
            func: GetSystemTimeAsFileTime,
            stack_consumed: || <Option<&mut FILETIME>>::stack_consumed(),
        },
        Symbol {
            name: "GetVersion",
            ordinal: None,
            func: GetVersion,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetVersionExA",
            ordinal: None,
            func: GetVersionExA,
            stack_consumed: || <Option<&mut OSVERSIONINFO>>::stack_consumed(),
        },
        Symbol {
            name: "GetProcessHeap",
            ordinal: None,
            func: GetProcessHeap,
            stack_consumed: || 0,
        },
        Symbol {
            name: "SetHandleCount",
            ordinal: None,
            func: SetHandleCount,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "OutputDebugStringA",
            ordinal: None,
            func: OutputDebugStringA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "InitializeCriticalSectionAndSpinCount",
            ordinal: None,
            func: InitializeCriticalSectionAndSpinCount,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "DeleteCriticalSection",
            ordinal: None,
            func: DeleteCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "EnterCriticalSection",
            ordinal: None,
            func: EnterCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "LeaveCriticalSection",
            ordinal: None,
            func: LeaveCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetUnhandledExceptionFilter",
            ordinal: None,
            func: SetUnhandledExceptionFilter,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "UnhandledExceptionFilter",
            ordinal: None,
            func: UnhandledExceptionFilter,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "NtCurrentTeb",
            ordinal: None,
            func: NtCurrentTeb,
            stack_consumed: || 0,
        },
        Symbol {
            name: "InitializeSListHead",
            ordinal: None,
            func: InitializeSListHead,
            stack_consumed: || <Option<&mut SLIST_HEADER>>::stack_consumed(),
        },
        Symbol {
            name: "MultiByteToWideChar",
            ordinal: None,
            func: MultiByteToWideChar,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <i32>::stack_consumed()
                    + <Option<&mut [u16]>>::stack_consumed()
            },
        },
        Symbol {
            name: "WriteConsoleW",
            ordinal: None,
            func: WriteConsoleW,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&[u16]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetCurrentThreadId",
            ordinal: None,
            func: GetCurrentThreadId,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TlsAlloc",
            ordinal: None,
            func: TlsAlloc,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TlsFree",
            ordinal: None,
            func: TlsFree,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "TlsSetValue",
            ordinal: None,
            func: TlsSetValue,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "TlsGetValue",
            ordinal: None,
            func: TlsGetValue,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "CreateThread",
            ordinal: None,
            func: CreateThread,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "SetThreadPriority",
            ordinal: None,
            func: SetThreadPriority,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "InterlockedIncrement",
            ordinal: None,
            func: InterlockedIncrement,
            stack_consumed: || <Option<&mut u32>>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        exports: &EXPORTS,
    };
}
pub mod ole32 {
    use super::*;
    use winapi::ole32::*;
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        exports: &EXPORTS,
    };
}
pub mod oleaut32 {
    use super::*;
    use winapi::oleaut32::*;
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        exports: &EXPORTS,
    };
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    pub unsafe fn RegisterClassA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpWndClass = <Option<&WNDCLASSA>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&WNDCLASSA>>::stack_consumed();
        let result = winapi::user32::RegisterClassA(machine, lpWndClass);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&WNDCLASSEXA>>::stack_consumed();
        let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn CreateWindowExA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let dwExStyle = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpClassName = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpWindowName = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let dwStyle = <Result<WindowStyle, u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Result<WindowStyle, u32>>::stack_consumed();
        let X = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let Y = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let nWidth = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let nHeight = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hWndParent = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let hMenu = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hInstance = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpParam = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let m: *mut Machine = machine;
        let result = async move {
            let machine = unsafe { &mut *m };
            let result = winapi::user32::CreateWindowExA(
                machine,
                dwExStyle,
                lpClassName,
                lpWindowName,
                dwStyle,
                X,
                Y,
                nWidth,
                nHeight,
                hWndParent,
                hMenu,
                hInstance,
                lpParam,
            )
            .await;
            machine.x86.cpu.regs.eax = result.to_raw();
            machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
            machine.x86.cpu.regs.esp += stack_offset;
        };
        crate::shims::become_async(machine, Box::pin(result));
    }
    pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetForegroundWindow(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetActiveWindow(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetLastActivePopup(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn UpdateWindow(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::UpdateWindow(machine, hWnd);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn ShowWindow(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let _nCmdShow = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::ShowWindow(machine, hWnd, _nCmdShow);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::SetFocus(machine, hWnd);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hCursor = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::SetCursor(machine, hCursor);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let lpText = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let lpCaption = <Option<&str>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&str>>::stack_consumed();
        let uType = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hInstance = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lpTemplateName = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let hWndParent = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let lpDialogFunc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let dwInitParam = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DialogBoxParamA(
            machine,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpMsg = <Option<&mut MSG>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut MSG>>::stack_consumed();
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let wMsgFilterMin = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let wMsgFilterMax = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Result<RemoveMsg, u32>>::stack_consumed();
        let result = winapi::user32::PeekMessageA(
            machine,
            lpMsg,
            hWnd,
            wMsgFilterMin,
            wMsgFilterMax,
            wRemoveMsg,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetMessageA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpMsg = <Option<&mut MSG>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&mut MSG>>::stack_consumed();
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let wMsgFilterMin = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let wMsgFilterMax = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::WaitMessage(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpMsg = <Option<&MSG>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&MSG>>::stack_consumed();
        let result = winapi::user32::TranslateMessage(machine, lpMsg);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn DispatchMessageA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let lpMsg = <Option<&MSG>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Option<&MSG>>::stack_consumed();
        let m: *mut Machine = machine;
        let result = async move {
            let machine = unsafe { &mut *m };
            let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
            machine.x86.cpu.regs.eax = result.to_raw();
            machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
            machine.x86.cpu.regs.esp += stack_offset;
        };
        crate::shims::become_async(machine, Box::pin(result));
    }
    pub unsafe fn DefWindowProcA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hWnd = <HWND>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <HWND>::stack_consumed();
        let msg = <Result<WM, u32>>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <Result<WM, u32>>::stack_consumed();
        let wParam = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let lParam = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _hInstance = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _lpIconName = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _hInstance = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _lpCursorName = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _bShow = <bool>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <bool>::stack_consumed();
        let result = winapi::user32::ShowCursor(machine, _bShow);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let hInstance = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let name = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let typ = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let cx = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let cy = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let fuLoad = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let nIndex = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::GetSystemMetrics(machine, nIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 23usize] = [
        Symbol {
            name: "RegisterClassA",
            ordinal: None,
            func: RegisterClassA,
            stack_consumed: || <Option<&WNDCLASSA>>::stack_consumed(),
        },
        Symbol {
            name: "RegisterClassExA",
            ordinal: None,
            func: RegisterClassExA,
            stack_consumed: || <Option<&WNDCLASSEXA>>::stack_consumed(),
        },
        Symbol {
            name: "CreateWindowExA",
            ordinal: None,
            func: CreateWindowExA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <Result<WindowStyle, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetForegroundWindow",
            ordinal: None,
            func: GetForegroundWindow,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetActiveWindow",
            ordinal: None,
            func: GetActiveWindow,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetLastActivePopup",
            ordinal: None,
            func: GetLastActivePopup,
            stack_consumed: || 0,
        },
        Symbol {
            name: "UpdateWindow",
            ordinal: None,
            func: UpdateWindow,
            stack_consumed: || <HWND>::stack_consumed(),
        },
        Symbol {
            name: "ShowWindow",
            ordinal: None,
            func: ShowWindow,
            stack_consumed: || <HWND>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetFocus",
            ordinal: None,
            func: SetFocus,
            stack_consumed: || <HWND>::stack_consumed(),
        },
        Symbol {
            name: "SetCursor",
            ordinal: None,
            func: SetCursor,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "MessageBoxA",
            ordinal: None,
            func: MessageBoxA,
            stack_consumed: || {
                <HWND>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "DialogBoxParamA",
            ordinal: None,
            func: DialogBoxParamA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "PeekMessageA",
            ordinal: None,
            func: PeekMessageA,
            stack_consumed: || {
                <Option<&mut MSG>>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<RemoveMsg, u32>>::stack_consumed()
            },
        },
        Symbol {
            name: "GetMessageA",
            ordinal: None,
            func: GetMessageA,
            stack_consumed: || {
                <Option<&mut MSG>>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "WaitMessage",
            ordinal: None,
            func: WaitMessage,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TranslateMessage",
            ordinal: None,
            func: TranslateMessage,
            stack_consumed: || <Option<&MSG>>::stack_consumed(),
        },
        Symbol {
            name: "DispatchMessageA",
            ordinal: None,
            func: DispatchMessageA,
            stack_consumed: || <Option<&MSG>>::stack_consumed(),
        },
        Symbol {
            name: "DefWindowProcA",
            ordinal: None,
            func: DefWindowProcA,
            stack_consumed: || {
                <HWND>::stack_consumed()
                    + <Result<WM, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "LoadIconA",
            ordinal: None,
            func: LoadIconA,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "LoadCursorA",
            ordinal: None,
            func: LoadCursorA,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "ShowCursor",
            ordinal: None,
            func: ShowCursor,
            stack_consumed: || <bool>::stack_consumed(),
        },
        Symbol {
            name: "LoadImageA",
            ordinal: None,
            func: LoadImageA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetSystemMetrics",
            ordinal: None,
            func: GetSystemMetrics,
            stack_consumed: || <u32>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        exports: &EXPORTS,
    };
}
pub mod winmm {
    use super::*;
    use winapi::winmm::*;
    pub unsafe fn timeSetEvent(machine: &mut Machine, esp: u32) {
        let mut stack_offset = 4u32;
        let _uDelay = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _uResolution = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _lpTimeProc = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _dwUser = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let _fuEvent = <u32>::from_stack(machine.mem(), esp + stack_offset);
        stack_offset += <u32>::stack_consumed();
        let result = winapi::winmm::timeSetEvent(
            machine,
            _uDelay,
            _uResolution,
            _lpTimeProc,
            _dwUser,
            _fuEvent,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        name: "timeSetEvent",
        ordinal: None,
        func: timeSetEvent,
        stack_consumed: || {
            <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
        },
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
    };
}
