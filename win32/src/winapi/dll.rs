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
        let arg1 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg2 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg3 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg4 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn BASS_MusicLoad(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let arg1 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg2 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg3 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg4 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let arg5 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn BASS_Start(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::bass::BASS_Start(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn BASS_MusicPlay(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let arg1 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_MusicPlay(machine, arg1);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn BASS_ChannelGetPosition(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let arg1 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::bass::BASS_ChannelGetPosition(machine, arg1);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("BASS_Init") => BASS_Init,
            winapi::ImportSymbol::Name("BASS_MusicLoad") => BASS_MusicLoad,
            winapi::ImportSymbol::Name("BASS_Start") => BASS_Start,
            winapi::ImportSymbol::Name("BASS_MusicPlay") => BASS_MusicPlay,
            winapi::ImportSymbol::Name("BASS_ChannelGetPosition") => BASS_ChannelGetPosition,
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
        let lpGuid = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lplpDD = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn DirectDrawCreateEx(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpGuid = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lplpDD = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let iid = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let pUnkOuter = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("DirectDrawCreate") => DirectDrawCreate,
            winapi::ImportSymbol::Name("DirectDrawCreateEx") => DirectDrawCreateEx,
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
        let _lpGuid = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let ppDS = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _pUnkOuter = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("DirectSoundCreate") => DirectSoundCreate,
            winapi::ImportSymbol::Ordinal(1u32) => DirectSoundCreate,
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
        let _i = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetStockObject(machine, _i);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SelectObject(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hGdiObj = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetObjectA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let handle = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _bytes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _out = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn CreateCompatibleDC(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn DeleteDC(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::DeleteDC(machine, hdc);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn BitBlt(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let x = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let y = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let cx = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let cy = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hdcSrc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let x1 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let y1 = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let rop = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn StretchBlt(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hdcDest = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let xDest = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let yDest = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let wDest = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hDest = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hdcSrc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let xSrc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let ySrc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let wSrc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hSrc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let rop = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::gdi32::StretchBlt(
            machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("GetStockObject") => GetStockObject,
            winapi::ImportSymbol::Name("SelectObject") => SelectObject,
            winapi::ImportSymbol::Name("GetObjectA") => GetObjectA,
            winapi::ImportSymbol::Name("CreateCompatibleDC") => CreateCompatibleDC,
            winapi::ImportSymbol::Name("DeleteDC") => DeleteDC,
            winapi::ImportSymbol::Name("BitBlt") => BitBlt,
            winapi::ImportSymbol::Name("StretchBlt") => StretchBlt,
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
    pub fn GetModuleHandleA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpModuleName = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetModuleHandleW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpModuleName = unsafe {
            <Option<Str16>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetModuleHandleExW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpModuleName = unsafe {
            <Option<Str16>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let hModule = unsafe {
            <Option<&mut HMODULE>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut HMODULE>>::stack_consumed();
        let result = winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn LoadLibraryA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let filename = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryA(machine, filename);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn LoadLibraryExW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpLibFileName = unsafe {
            <Option<Str16>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let hFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HFILE>::stack_consumed();
        let dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetProcAddress(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hModule = unsafe {
            <HMODULE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HMODULE>::stack_consumed();
        let lpProcName = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetStdHandle(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let nStdHandle = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn CreateFileA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpFileName = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let dwDesiredAccess = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwShareMode = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition = unsafe {
            <Result<CreationDisposition, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
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
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn CreateFileW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpFileName = unsafe {
            <Option<Str16>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<Str16>>::stack_consumed();
        let dwDesiredAccess = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwShareMode = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpSecurityAttributes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwCreationDisposition = unsafe {
            <Result<CreationDisposition, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<CreationDisposition, u32>>::stack_consumed();
        let dwFlagsAndAttributes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hTemplateFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
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
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetFileType(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HFILE>::stack_consumed();
        let result = winapi::kernel32::GetFileType(machine, hFile);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetFilePointer(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HFILE>::stack_consumed();
        let lDistanceToMove = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpDistanceToMoveHigh = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let dwMoveMethod = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetFilePointer(
            machine,
            hFile,
            lDistanceToMove,
            lpDistanceToMoveHigh,
            dwMoveMethod,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn ReadFile(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe {
            <Option<&mut [u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let lpNumberOfBytesRead = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn WriteFile(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hFile = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe {
            <Option<&[u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&[u8]>>::stack_consumed();
        let lpNumberOfBytesWritten = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let lpOverlapped = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn HeapAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwBytes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn HeapFree(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpMem = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn HeapSize(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpMem = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn HeapReAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpMem = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwBytes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn HeapCreate(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let flOptions = unsafe {
            <Result<HeapCreateFlags, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<HeapCreateFlags, u32>>::stack_consumed();
        let dwInitialSize = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwMaximumSize = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn HeapDestroy(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hHeap = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::HeapDestroy(machine, hHeap);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn VirtualAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpAddress = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwSize = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _flAllocationType = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _flProtec = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualAlloc(
            machine,
            lpAddress,
            dwSize,
            _flAllocationType,
            _flProtec,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn VirtualFree(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpAddress = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwSize = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwFreeType = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn IsBadReadPtr(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lp = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let ucb = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn IsBadWritePtr(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lp = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let ucb = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetLastError(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwErrCode = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetLastError(machine, dwErrCode);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetLastError(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetLastError(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn ExitProcess(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let uExitCode = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::ExitProcess(machine, uExitCode);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetACP(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetACP(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn IsValidCodePage(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let CodePage = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetCPInfo(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _CodePage = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _lpCPInfo = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetCommandLineA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineA(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetCommandLineW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCommandLineW(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetEnvironmentStrings(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStrings(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn FreeEnvironmentStringsA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _penv = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetEnvironmentStringsW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetEnvironmentStringsW(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetEnvironmentVariableA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let name = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let buf = unsafe {
            <Option<&mut [u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetModuleFileNameA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hModule = unsafe {
            <HMODULE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HMODULE>::stack_consumed();
        let filename = unsafe {
            <Option<&mut [u8]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut [u8]>>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetModuleFileNameW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hModule = unsafe {
            <HMODULE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HMODULE>::stack_consumed();
        let _lpFilename = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _nSize = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetStartupInfoA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpStartupInfo = unsafe {
            <Option<&mut STARTUPINFOA>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut STARTUPINFOA>>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetStartupInfoW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpStartupInfo = unsafe {
            <Option<&mut STARTUPINFOA>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut STARTUPINFOA>>::stack_consumed();
        let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn IsProcessorFeaturePresent(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let feature = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn IsDebuggerPresent(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::IsDebuggerPresent(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetCurrentProcessId(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentProcessId(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetTickCount(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetTickCount(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn QueryPerformanceCounter(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpPerformanceCount = unsafe {
            <Option<&mut u64>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u64>>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn QueryPerformanceFrequency(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpFrequency = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetSystemTimeAsFileTime(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _time = unsafe {
            <Option<&mut FILETIME>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut FILETIME>>::stack_consumed();
        let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetVersion(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetVersion(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetVersionExA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpVersionInformation = unsafe {
            <Option<&mut OSVERSIONINFO>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut OSVERSIONINFO>>::stack_consumed();
        let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetProcessHeap(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetProcessHeap(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetHandleCount(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let uNumber = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetHandleCount(machine, uNumber);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn OutputDebugStringA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let msg = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let result = winapi::kernel32::OutputDebugStringA(machine, msg);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _dwSpinCount = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn DeleteCriticalSection(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn EnterCriticalSection(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn LeaveCriticalSection(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpCriticalSection = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetUnhandledExceptionFilter(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _lpTopLevelExceptionFilter = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn UnhandledExceptionFilter(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _exceptionInfo = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn NtCurrentTeb(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::NtCurrentTeb(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn InitializeSListHead(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let ListHead = unsafe {
            <Option<&mut SLIST_HEADER>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut SLIST_HEADER>>::stack_consumed();
        let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn MultiByteToWideChar(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let CodePage = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _dwFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpMultiByteStr = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let cbMultiByte = unsafe {
            <i32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <i32>::stack_consumed();
        let lpWideCharStr = unsafe {
            <Option<&mut [u16]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
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
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn WriteConsoleW(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hConsoleOutput = unsafe {
            <HFILE>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HFILE>::stack_consumed();
        let lpBuffer = unsafe {
            <Option<&[u16]>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&[u16]>>::stack_consumed();
        let lpNumberOfCharsWritten = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let _lpReserved = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetCurrentThreadId(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::GetCurrentThreadId(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn TlsAlloc(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::kernel32::TlsAlloc(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn TlsFree(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwTlsIndex = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn TlsSetValue(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwTlsIndex = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpTlsValue = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn TlsGetValue(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwTlsIndex = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn CreateThread(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpThreadAttributes = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwStackSize = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpStartAddress = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpParameter = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwCreationFlags = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpThreadId = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
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
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetThreadPriority(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _hThread = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _nPriority = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn InterlockedIncrement(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let addend = unsafe {
            <Option<&mut u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut u32>>::stack_consumed();
        let result = winapi::kernel32::InterlockedIncrement(machine, addend);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("GetModuleHandleA") => GetModuleHandleA,
            winapi::ImportSymbol::Name("GetModuleHandleW") => GetModuleHandleW,
            winapi::ImportSymbol::Name("GetModuleHandleExW") => GetModuleHandleExW,
            winapi::ImportSymbol::Name("LoadLibraryA") => LoadLibraryA,
            winapi::ImportSymbol::Name("LoadLibraryExW") => LoadLibraryExW,
            winapi::ImportSymbol::Name("GetProcAddress") => GetProcAddress,
            winapi::ImportSymbol::Name("GetStdHandle") => GetStdHandle,
            winapi::ImportSymbol::Name("CreateFileA") => CreateFileA,
            winapi::ImportSymbol::Name("CreateFileW") => CreateFileW,
            winapi::ImportSymbol::Name("GetFileType") => GetFileType,
            winapi::ImportSymbol::Name("SetFilePointer") => SetFilePointer,
            winapi::ImportSymbol::Name("ReadFile") => ReadFile,
            winapi::ImportSymbol::Name("WriteFile") => WriteFile,
            winapi::ImportSymbol::Name("HeapAlloc") => HeapAlloc,
            winapi::ImportSymbol::Name("HeapFree") => HeapFree,
            winapi::ImportSymbol::Name("HeapSize") => HeapSize,
            winapi::ImportSymbol::Name("HeapReAlloc") => HeapReAlloc,
            winapi::ImportSymbol::Name("HeapCreate") => HeapCreate,
            winapi::ImportSymbol::Name("HeapDestroy") => HeapDestroy,
            winapi::ImportSymbol::Name("VirtualAlloc") => VirtualAlloc,
            winapi::ImportSymbol::Name("VirtualFree") => VirtualFree,
            winapi::ImportSymbol::Name("IsBadReadPtr") => IsBadReadPtr,
            winapi::ImportSymbol::Name("IsBadWritePtr") => IsBadWritePtr,
            winapi::ImportSymbol::Name("SetLastError") => SetLastError,
            winapi::ImportSymbol::Name("GetLastError") => GetLastError,
            winapi::ImportSymbol::Name("ExitProcess") => ExitProcess,
            winapi::ImportSymbol::Name("GetACP") => GetACP,
            winapi::ImportSymbol::Name("IsValidCodePage") => IsValidCodePage,
            winapi::ImportSymbol::Name("GetCPInfo") => GetCPInfo,
            winapi::ImportSymbol::Name("GetCommandLineA") => GetCommandLineA,
            winapi::ImportSymbol::Name("GetCommandLineW") => GetCommandLineW,
            winapi::ImportSymbol::Name("GetEnvironmentStrings") => GetEnvironmentStrings,
            winapi::ImportSymbol::Name("FreeEnvironmentStringsA") => FreeEnvironmentStringsA,
            winapi::ImportSymbol::Name("GetEnvironmentStringsW") => GetEnvironmentStringsW,
            winapi::ImportSymbol::Name("GetEnvironmentVariableA") => GetEnvironmentVariableA,
            winapi::ImportSymbol::Name("GetModuleFileNameA") => GetModuleFileNameA,
            winapi::ImportSymbol::Name("GetModuleFileNameW") => GetModuleFileNameW,
            winapi::ImportSymbol::Name("GetStartupInfoA") => GetStartupInfoA,
            winapi::ImportSymbol::Name("GetStartupInfoW") => GetStartupInfoW,
            winapi::ImportSymbol::Name("IsProcessorFeaturePresent") => IsProcessorFeaturePresent,
            winapi::ImportSymbol::Name("IsDebuggerPresent") => IsDebuggerPresent,
            winapi::ImportSymbol::Name("GetCurrentProcessId") => GetCurrentProcessId,
            winapi::ImportSymbol::Name("GetTickCount") => GetTickCount,
            winapi::ImportSymbol::Name("QueryPerformanceCounter") => QueryPerformanceCounter,
            winapi::ImportSymbol::Name("QueryPerformanceFrequency") => QueryPerformanceFrequency,
            winapi::ImportSymbol::Name("GetSystemTimeAsFileTime") => GetSystemTimeAsFileTime,
            winapi::ImportSymbol::Name("GetVersion") => GetVersion,
            winapi::ImportSymbol::Name("GetVersionExA") => GetVersionExA,
            winapi::ImportSymbol::Name("GetProcessHeap") => GetProcessHeap,
            winapi::ImportSymbol::Name("SetHandleCount") => SetHandleCount,
            winapi::ImportSymbol::Name("OutputDebugStringA") => OutputDebugStringA,
            winapi::ImportSymbol::Name("InitializeCriticalSectionAndSpinCount") => {
                InitializeCriticalSectionAndSpinCount
            }
            winapi::ImportSymbol::Name("DeleteCriticalSection") => DeleteCriticalSection,
            winapi::ImportSymbol::Name("EnterCriticalSection") => EnterCriticalSection,
            winapi::ImportSymbol::Name("LeaveCriticalSection") => LeaveCriticalSection,
            winapi::ImportSymbol::Name("SetUnhandledExceptionFilter") => {
                SetUnhandledExceptionFilter
            }
            winapi::ImportSymbol::Name("UnhandledExceptionFilter") => UnhandledExceptionFilter,
            winapi::ImportSymbol::Name("NtCurrentTeb") => NtCurrentTeb,
            winapi::ImportSymbol::Name("InitializeSListHead") => InitializeSListHead,
            winapi::ImportSymbol::Name("MultiByteToWideChar") => MultiByteToWideChar,
            winapi::ImportSymbol::Name("WriteConsoleW") => WriteConsoleW,
            winapi::ImportSymbol::Name("GetCurrentThreadId") => GetCurrentThreadId,
            winapi::ImportSymbol::Name("TlsAlloc") => TlsAlloc,
            winapi::ImportSymbol::Name("TlsFree") => TlsFree,
            winapi::ImportSymbol::Name("TlsSetValue") => TlsSetValue,
            winapi::ImportSymbol::Name("TlsGetValue") => TlsGetValue,
            winapi::ImportSymbol::Name("CreateThread") => CreateThread,
            winapi::ImportSymbol::Name("SetThreadPriority") => SetThreadPriority,
            winapi::ImportSymbol::Name("InterlockedIncrement") => InterlockedIncrement,
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
        let lpWndClass = unsafe {
            <Option<&WNDCLASSA>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&WNDCLASSA>>::stack_consumed();
        let result = winapi::user32::RegisterClassA(machine, lpWndClass);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn RegisterClassExA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpWndClassEx = unsafe {
            <Option<&WNDCLASSEXA>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&WNDCLASSEXA>>::stack_consumed();
        let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn CreateWindowExA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let dwExStyle = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpClassName = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpWindowName = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let dwStyle = unsafe {
            <Result<WindowStyle, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<WindowStyle, u32>>::stack_consumed();
        let X = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let Y = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let nWidth = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let nHeight = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hWndParent = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let hMenu = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hInstance = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpParam = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
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
            machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
            machine.x86.cpu.regs.esp += stack_offset;
        };
        crate::shims::become_async(machine, Box::pin(result));
    }
    pub fn GetForegroundWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetForegroundWindow(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetActiveWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetActiveWindow(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetLastActivePopup(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::GetLastActivePopup(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn UpdateWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::UpdateWindow(machine, hWnd);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn ShowWindow(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let _nCmdShow = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::ShowWindow(machine, hWnd, _nCmdShow);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetFocus(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let result = winapi::user32::SetFocus(machine, hWnd);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn SetCursor(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hCursor = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::SetCursor(machine, hCursor);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn MessageBoxA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let lpText = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let lpCaption = unsafe {
            <Option<&str>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&str>>::stack_consumed();
        let uType = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn DialogBoxParamA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hInstance = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lpTemplateName = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let hWndParent = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let lpDialogFunc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let dwInitParam = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
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
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn PeekMessageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe {
            <Option<&mut MSG>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut MSG>>::stack_consumed();
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let wMsgFilterMin = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let wMsgFilterMax = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let wRemoveMsg = unsafe {
            <Result<RemoveMsg, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
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
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetMessageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe {
            <Option<&mut MSG>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&mut MSG>>::stack_consumed();
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let wMsgFilterMin = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let wMsgFilterMax = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result =
            winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn WaitMessage(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let result = winapi::user32::WaitMessage(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn TranslateMessage(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe {
            <Option<&MSG>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&MSG>>::stack_consumed();
        let result = winapi::user32::TranslateMessage(machine, lpMsg);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn DispatchMessageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let lpMsg = unsafe {
            <Option<&MSG>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Option<&MSG>>::stack_consumed();
        let m: *mut Machine = machine;
        let result = async move {
            let machine = unsafe { &mut *m };
            let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
            machine.x86.cpu.regs.eax = result.to_raw();
            machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
            machine.x86.cpu.regs.esp += stack_offset;
        };
        crate::shims::become_async(machine, Box::pin(result));
    }
    pub fn DefWindowProcA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hWnd = unsafe {
            <HWND>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <HWND>::stack_consumed();
        let msg = unsafe {
            <Result<WM, u32>>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <Result<WM, u32>>::stack_consumed();
        let wParam = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let lParam = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn LoadIconA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _hInstance = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _lpIconName = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn LoadCursorA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _hInstance = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _lpCursorName = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn ShowCursor(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let _bShow = unsafe {
            <bool>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <bool>::stack_consumed();
        let result = winapi::user32::ShowCursor(machine, _bShow);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn LoadImageA(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let hInstance = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let name = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let typ = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _cx = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _cy = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let fuLoad = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, _cx, _cy, fuLoad);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    pub fn GetSystemMetrics(machine: &mut Machine) {
        let mut stack_offset = 4u32;
        let nIndex = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let result = winapi::user32::GetSystemMetrics(machine, nIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("RegisterClassA") => RegisterClassA,
            winapi::ImportSymbol::Name("RegisterClassExA") => RegisterClassExA,
            winapi::ImportSymbol::Name("CreateWindowExA") => CreateWindowExA,
            winapi::ImportSymbol::Name("GetForegroundWindow") => GetForegroundWindow,
            winapi::ImportSymbol::Name("GetActiveWindow") => GetActiveWindow,
            winapi::ImportSymbol::Name("GetLastActivePopup") => GetLastActivePopup,
            winapi::ImportSymbol::Name("UpdateWindow") => UpdateWindow,
            winapi::ImportSymbol::Name("ShowWindow") => ShowWindow,
            winapi::ImportSymbol::Name("SetFocus") => SetFocus,
            winapi::ImportSymbol::Name("SetCursor") => SetCursor,
            winapi::ImportSymbol::Name("MessageBoxA") => MessageBoxA,
            winapi::ImportSymbol::Name("DialogBoxParamA") => DialogBoxParamA,
            winapi::ImportSymbol::Name("PeekMessageA") => PeekMessageA,
            winapi::ImportSymbol::Name("GetMessageA") => GetMessageA,
            winapi::ImportSymbol::Name("WaitMessage") => WaitMessage,
            winapi::ImportSymbol::Name("TranslateMessage") => TranslateMessage,
            winapi::ImportSymbol::Name("DispatchMessageA") => DispatchMessageA,
            winapi::ImportSymbol::Name("DefWindowProcA") => DefWindowProcA,
            winapi::ImportSymbol::Name("LoadIconA") => LoadIconA,
            winapi::ImportSymbol::Name("LoadCursorA") => LoadCursorA,
            winapi::ImportSymbol::Name("ShowCursor") => ShowCursor,
            winapi::ImportSymbol::Name("LoadImageA") => LoadImageA,
            winapi::ImportSymbol::Name("GetSystemMetrics") => GetSystemMetrics,
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
        let _uDelay = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _uResolution = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _lpTimeProc = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _dwUser = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
        stack_offset += <u32>::stack_consumed();
        let _fuEvent = unsafe {
            <u32>::from_stack(
                &mut machine.x86.mem,
                machine.x86.cpu.regs.esp + stack_offset,
            )
        };
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
        machine.x86.cpu.regs.eip = machine.x86.mem.get::<u32>(machine.x86.cpu.regs.esp);
        machine.x86.cpu.regs.esp += stack_offset;
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name("timeSetEvent") => timeSetEvent,
            _ => return None,
        })
    }
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        resolve,
    };
}
