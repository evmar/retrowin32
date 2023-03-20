#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#[doc = r" Generated code, do not edit."]
use crate::{
    machine::Machine,
    winapi::{self, stack_args::*, types::*, BuiltinDLL},
};
use x86::Memory;
pub mod bass {
    use super::*;
    use winapi::bass::*;
    pub fn BASS_Init(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let arg1 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg2 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg3 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg4 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn BASS_MusicLoad(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let arg1 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg2 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg3 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg4 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let arg5 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn BASS_Start(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::bass::BASS_Start(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn BASS_MusicPlay(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let arg1 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicPlay(machine, arg1);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "BASS_Init" => BASS_Init,
                "BASS_MusicLoad" => BASS_MusicLoad,
                "BASS_Start" => BASS_Start,
                "BASS_MusicPlay" => BASS_MusicPlay,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        resolve,
    };
}
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    pub fn DirectDrawCreate(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpGuid =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lplpDD =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn DirectDrawCreateEx(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpGuid =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lplpDD =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let iid =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "DirectDrawCreate" => DirectDrawCreate,
                "DirectDrawCreateEx" => DirectDrawCreateEx,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        resolve,
    };
}
pub mod dsound {
    use super::*;
    use winapi::dsound::*;
    pub fn DirectSoundCreate(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpGuid =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ppDS =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _pUnkOuter =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "DirectSoundCreate" => DirectSoundCreate,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        resolve,
    };
}
pub mod gdi32 {
    use super::*;
    use winapi::gdi32::*;
    pub fn GetStockObject(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _i =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetStockObject(machine, _i);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn SelectObject(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hGdiObj =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetObjectA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let handle =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _bytes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _out =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn CreateCompatibleDC(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn DeleteDC(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::DeleteDC(machine, hdc);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn BitBlt(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let x =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let y =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cx =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cy =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hdcSrc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let x1 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let y1 =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let rop =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn StretchBlt(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdcDest =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let xDest =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let yDest =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wDest =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hDest =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hdcSrc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let xSrc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ySrc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let wSrc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hSrc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let rop =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::StretchBlt(
            machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "GetStockObject" => GetStockObject,
                "SelectObject" => SelectObject,
                "GetObjectA" => GetObjectA,
                "CreateCompatibleDC" => CreateCompatibleDC,
                "DeleteDC" => DeleteDC,
                "BitBlt" => BitBlt,
                "StretchBlt" => StretchBlt,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        resolve,
    };
}
pub mod kernel32 {
    use super::*;
    use winapi::kernel32::*;
    pub fn SetLastError(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwErrCode =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetLastError(machine, dwErrCode);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetLastError(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetLastError(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn ExitProcess(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let uExitCode =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::ExitProcess(machine, uExitCode);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetACP(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetACP(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn IsValidCodePage(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let CodePage =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetCPInfo(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _CodePage =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpCPInfo =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetCommandLineA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineA(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetCommandLineW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineW(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetEnvironmentStrings(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStrings(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn FreeEnvironmentStringsA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _penv =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetEnvironmentStringsW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStringsW(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetEnvironmentVariableA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let name = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let buf = unsafe {
            <Option<&mut [u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetFileType(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::GetFileType(machine, hFile);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn SetFilePointer(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HFILE>::stack_consumed();
        let lDistanceToMove =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpDistanceToMoveHigh = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let dwMoveMethod =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetFilePointer(
            machine,
            hFile,
            lDistanceToMove,
            lpDistanceToMoveHigh,
            dwMoveMethod,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn ReadFile(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe {
            <Option<&mut [u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let lpNumberOfBytesRead = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetModuleFileNameA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hModule = unsafe {
            <HMODULE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HMODULE>::stack_consumed();
        let filename = unsafe {
            <Option<&mut [u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetModuleFileNameW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hModule = unsafe {
            <HMODULE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HMODULE>::stack_consumed();
        let _lpFilename =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _nSize =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetModuleHandleA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpModuleName = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetModuleHandleW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpModuleName = unsafe {
            <Option<Str16>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetModuleHandleExW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpModuleName = unsafe {
            <Option<Str16>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let hModule = unsafe {
            <Option<&mut HMODULE>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut HMODULE>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetStartupInfoA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpStartupInfo =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetStartupInfoW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpStartupInfo =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn IsProcessorFeaturePresent(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let feature =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn IsDebuggerPresent(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::IsDebuggerPresent(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetCurrentThreadId(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentThreadId(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetCurrentProcessId(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentProcessId(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetStdHandle(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let nStdHandle =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetTickCount(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetTickCount(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn QueryPerformanceCounter(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _ptr =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceCounter(machine, _ptr);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetSystemTimeAsFileTime(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _time = unsafe {
            <Option<&mut FILETIME>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut FILETIME>>::stack_consumed();
        let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetVersion(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetVersion(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetVersionExA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpVersionInformation =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn HeapAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwBytes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn HeapFree(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMem =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn HeapSize(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMem =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn HeapReAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMem =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwBytes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn HeapCreate(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let flOptions = unsafe {
            <Result<HeapCreateFlags, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<HeapCreateFlags, u32>>::stack_consumed();
        let dwInitialSize =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _dwMaximumSize =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, _dwMaximumSize);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn HeapDestroy(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapDestroy(machine, hHeap);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetProcessHeap(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetProcessHeap(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn LoadLibraryA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let filename = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryA(machine, filename);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn LoadLibraryExW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpLibFileName = unsafe {
            <Option<Str16>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let hFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HFILE>::stack_consumed();
        let dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetProcAddress(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hModule = unsafe {
            <HMODULE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HMODULE>::stack_consumed();
        let lpProcName = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn SetHandleCount(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let uNumber =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetHandleCount(machine, uNumber);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn CreateFileA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpFileName = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let dwDesiredAccess =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwShareMode =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition = unsafe {
            <Result<CreationDisposition, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
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
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn CreateFileW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpFileName = unsafe {
            <Option<Str16>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let dwDesiredAccess =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwShareMode =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition = unsafe {
            <Result<CreationDisposition, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
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
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn WriteFile(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe {
            <Option<&[u8]>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&[u8]>>::stack_consumed();
        let lpNumberOfBytesWritten = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn VirtualAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpAddress =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwSize =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _flAllocationType =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _flProtec =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualAlloc(
            machine,
            lpAddress,
            dwSize,
            _flAllocationType,
            _flProtec,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn VirtualFree(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpAddress =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwSize =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwFreeType =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn OutputDebugStringA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let msg = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::OutputDebugStringA(machine, msg);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _dwSpinCount =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn DeleteCriticalSection(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn EnterCriticalSection(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn LeaveCriticalSection(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn SetUnhandledExceptionFilter(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpTopLevelExceptionFilter =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn UnhandledExceptionFilter(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _exceptionInfo =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn NtCurrentTeb(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::NtCurrentTeb(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn TlsAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::TlsAlloc(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn TlsFree(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwTlsIndex =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn TlsSetValue(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwTlsIndex =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpTlsValue =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn TlsGetValue(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwTlsIndex =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn InitializeSListHead(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let ListHead = unsafe {
            <Option<&mut SLIST_HEADER>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut SLIST_HEADER>>::stack_consumed();
        let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn MultiByteToWideChar(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let CodePage =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _dwFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpMultiByteStr =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let cbMultiByte =
            unsafe { <i32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <i32>::stack_consumed();
        let lpWideCharStr = unsafe {
            <Option<&mut [u16]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u16]>>::stack_consumed();
        let result = winapi::kernel32::MultiByteToWideChar(
            machine,
            CodePage,
            _dwFlags,
            lpMultiByteStr,
            cbMultiByte,
            lpWideCharStr,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn WriteConsoleW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hConsoleOutput = unsafe {
            <HFILE>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe {
            <Option<&[u16]>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&[u16]>>::stack_consumed();
        let lpNumberOfCharsWritten = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let _lpReserved =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn CreateThread(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpThreadAttributes =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwStackSize =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpStartAddress =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpParameter =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwCreationFlags =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpThreadId =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
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
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn SetThreadPriority(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _hThread =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _nPriority =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn IsBadReadPtr(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lp =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ucb =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn IsBadWritePtr(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lp =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let ucb =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn QueryPerformanceFrequency(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpFrequency =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "SetLastError" => SetLastError,
                "GetLastError" => GetLastError,
                "ExitProcess" => ExitProcess,
                "GetACP" => GetACP,
                "IsValidCodePage" => IsValidCodePage,
                "GetCPInfo" => GetCPInfo,
                "GetCommandLineA" => GetCommandLineA,
                "GetCommandLineW" => GetCommandLineW,
                "GetEnvironmentStrings" => GetEnvironmentStrings,
                "FreeEnvironmentStringsA" => FreeEnvironmentStringsA,
                "GetEnvironmentStringsW" => GetEnvironmentStringsW,
                "GetEnvironmentVariableA" => GetEnvironmentVariableA,
                "GetFileType" => GetFileType,
                "SetFilePointer" => SetFilePointer,
                "ReadFile" => ReadFile,
                "GetModuleFileNameA" => GetModuleFileNameA,
                "GetModuleFileNameW" => GetModuleFileNameW,
                "GetModuleHandleA" => GetModuleHandleA,
                "GetModuleHandleW" => GetModuleHandleW,
                "GetModuleHandleExW" => GetModuleHandleExW,
                "GetStartupInfoA" => GetStartupInfoA,
                "GetStartupInfoW" => GetStartupInfoW,
                "IsProcessorFeaturePresent" => IsProcessorFeaturePresent,
                "IsDebuggerPresent" => IsDebuggerPresent,
                "GetCurrentThreadId" => GetCurrentThreadId,
                "GetCurrentProcessId" => GetCurrentProcessId,
                "GetStdHandle" => GetStdHandle,
                "GetTickCount" => GetTickCount,
                "QueryPerformanceCounter" => QueryPerformanceCounter,
                "GetSystemTimeAsFileTime" => GetSystemTimeAsFileTime,
                "GetVersion" => GetVersion,
                "GetVersionExA" => GetVersionExA,
                "HeapAlloc" => HeapAlloc,
                "HeapFree" => HeapFree,
                "HeapSize" => HeapSize,
                "HeapReAlloc" => HeapReAlloc,
                "HeapCreate" => HeapCreate,
                "HeapDestroy" => HeapDestroy,
                "GetProcessHeap" => GetProcessHeap,
                "LoadLibraryA" => LoadLibraryA,
                "LoadLibraryExW" => LoadLibraryExW,
                "GetProcAddress" => GetProcAddress,
                "SetHandleCount" => SetHandleCount,
                "CreateFileA" => CreateFileA,
                "CreateFileW" => CreateFileW,
                "WriteFile" => WriteFile,
                "VirtualAlloc" => VirtualAlloc,
                "VirtualFree" => VirtualFree,
                "OutputDebugStringA" => OutputDebugStringA,
                "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount,
                "DeleteCriticalSection" => DeleteCriticalSection,
                "EnterCriticalSection" => EnterCriticalSection,
                "LeaveCriticalSection" => LeaveCriticalSection,
                "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter,
                "UnhandledExceptionFilter" => UnhandledExceptionFilter,
                "NtCurrentTeb" => NtCurrentTeb,
                "TlsAlloc" => TlsAlloc,
                "TlsFree" => TlsFree,
                "TlsSetValue" => TlsSetValue,
                "TlsGetValue" => TlsGetValue,
                "InitializeSListHead" => InitializeSListHead,
                "MultiByteToWideChar" => MultiByteToWideChar,
                "WriteConsoleW" => WriteConsoleW,
                "CreateThread" => CreateThread,
                "SetThreadPriority" => SetThreadPriority,
                "IsBadReadPtr" => IsBadReadPtr,
                "IsBadWritePtr" => IsBadWritePtr,
                "QueryPerformanceFrequency" => QueryPerformanceFrequency,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        resolve,
    };
}
pub mod ole32 {
    use super::*;
    use winapi::ole32::*;
    fn resolve(_sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        None
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        resolve,
    };
}
pub mod oleaut32 {
    use super::*;
    use winapi::oleaut32::*;
    fn resolve(_sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        None
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        resolve,
    };
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    pub fn RegisterClassA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpWndClass =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::RegisterClassA(machine, lpWndClass);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn RegisterClassExA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpWndClassEx =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn CreateWindowExA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwExStyle =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let className = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let windowName = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let dwStyle =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let X =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let Y =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let nWidth =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let nHeight =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hWndParent = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let hMenu =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hInstance =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpParam =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::CreateWindowExA(
            machine, dwExStyle, className, windowName, dwStyle, X, Y, nWidth, nHeight, hWndParent,
            hMenu, hInstance, lpParam,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetForegroundWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetForegroundWindow(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetActiveWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetActiveWindow(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetLastActivePopup(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetLastActivePopup(machine);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn UpdateWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::UpdateWindow(machine, hWnd);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn ShowWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let _nCmdShow =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::ShowWindow(machine, hWnd, _nCmdShow);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn SetFocus(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::SetFocus(machine, hWnd);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn MessageBoxA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let lpText = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let lpCaption = unsafe {
            <Option<&str>>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let uType =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn DialogBoxParamA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hInstance =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let lpTemplateName =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hWndParent = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let lpDialogFunc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let dwInitParam =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DialogBoxParamA(
            machine,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn PeekMessageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpMsg =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hWnd = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let _wMsgFilterMin =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _wMsgFilterMax =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _wRemoveMs =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::PeekMessageA(
            machine,
            _lpMsg,
            hWnd,
            _wMsgFilterMin,
            _wMsgFilterMax,
            _wRemoveMs,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetMessageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpMsg =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let hWnd = unsafe {
            <HWND>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <HWND>::stack_consumed();
        let _wMsgFilterMin =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _wMsgFilterMax =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::user32::GetMessageA(machine, _lpMsg, hWnd, _wMsgFilterMin, _wMsgFilterMax);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn TranslateMessage(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpMsg =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::TranslateMessage(machine, _lpMsg);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn DispatchMessageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpMsg =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DispatchMessageA(machine, _lpMsg);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn LoadIconA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _hInstance =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpIconName =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn LoadCursorA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _hInstance =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpCursorName =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn ShowCursor(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _bShow = unsafe {
            <bool>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset)
        };
        stack_offset += <bool>::stack_consumed();
        let result = winapi::user32::ShowCursor(machine, _bShow);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn LoadImageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hInstance =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let name =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let typ =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _cx =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _cy =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let fuLoad =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, _cx, _cy, fuLoad);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    pub fn GetSystemMetrics(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let nIndex =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::GetSystemMetrics(machine, nIndex);
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "RegisterClassA" => RegisterClassA,
                "RegisterClassExA" => RegisterClassExA,
                "CreateWindowExA" => CreateWindowExA,
                "GetForegroundWindow" => GetForegroundWindow,
                "GetActiveWindow" => GetActiveWindow,
                "GetLastActivePopup" => GetLastActivePopup,
                "UpdateWindow" => UpdateWindow,
                "ShowWindow" => ShowWindow,
                "SetFocus" => SetFocus,
                "MessageBoxA" => MessageBoxA,
                "DialogBoxParamA" => DialogBoxParamA,
                "PeekMessageA" => PeekMessageA,
                "GetMessageA" => GetMessageA,
                "TranslateMessage" => TranslateMessage,
                "DispatchMessageA" => DispatchMessageA,
                "LoadIconA" => LoadIconA,
                "LoadCursorA" => LoadCursorA,
                "ShowCursor" => ShowCursor,
                "LoadImageA" => LoadImageA,
                "GetSystemMetrics" => GetSystemMetrics,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        resolve,
    };
}
pub mod winmm {
    use super::*;
    use winapi::winmm::*;
    pub fn timeSetEvent(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _uDelay =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _uResolution =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _lpTimeProc =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _dwUser =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let _fuEvent =
            unsafe { <u32>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::winmm::timeSetEvent(
            machine,
            _uDelay,
            _uResolution,
            _lpTimeProc,
            _dwUser,
            _fuEvent,
        );
        let return_address = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
        machine.x86.regs.eax = result.to_raw();
        machine.x86.regs.eip = return_address;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "timeSetEvent" => timeSetEvent,
                _ => return None,
            },
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        resolve,
    };
}
