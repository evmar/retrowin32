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
    pub ordinal: Option<usize>,
    pub shim: shims::Shim,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
}
pub mod bass {
    use super::*;
    use winapi::bass::*;
    pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) {
        let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
        let arg2 = <u32>::from_stack(machine.mem(), esp + 8u32);
        let arg3 = <u32>::from_stack(machine.mem(), esp + 12u32);
        let arg4 = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) {
        let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
        let arg2 = <u32>::from_stack(machine.mem(), esp + 8u32);
        let arg3 = <u32>::from_stack(machine.mem(), esp + 12u32);
        let arg4 = <u32>::from_stack(machine.mem(), esp + 16u32);
        let arg5 = <u32>::from_stack(machine.mem(), esp + 20u32);
        let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 24u32;
    }
    pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) {
        let result = winapi::bass::BASS_Start(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) {
        let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::bass::BASS_MusicPlay(machine, arg1);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) {
        let arg1 = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::bass::BASS_ChannelGetPosition(machine, arg1);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    const EXPORTS: [Symbol; 5usize] = [
        Symbol {
            shim: shims::Shim {
                name: "BASS_Init",
                func: BASS_Init,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "BASS_MusicLoad",
                func: BASS_MusicLoad,
                stack_consumed: 24u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "BASS_Start",
                func: BASS_Start,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "BASS_MusicPlay",
                func: BASS_MusicPlay,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "BASS_ChannelGetPosition",
                func: BASS_ChannelGetPosition,
                stack_consumed: 8u32,
            },
            ordinal: None,
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
        let lpGuid = <u32>::from_stack(machine.mem(), esp + 4u32);
        let lplpDD = <u32>::from_stack(machine.mem(), esp + 8u32);
        let pUnkOuter = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) {
        let lpGuid = <u32>::from_stack(machine.mem(), esp + 4u32);
        let lplpDD = <u32>::from_stack(machine.mem(), esp + 8u32);
        let iid = <u32>::from_stack(machine.mem(), esp + 12u32);
        let pUnkOuter = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            shim: shims::Shim {
                name: "DirectDrawCreate",
                func: DirectDrawCreate,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "DirectDrawCreateEx",
                func: DirectDrawCreateEx,
                stack_consumed: 20u32,
            },
            ordinal: None,
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
        let _lpGuid = <u32>::from_stack(machine.mem(), esp + 4u32);
        let ppDS = <u32>::from_stack(machine.mem(), esp + 8u32);
        let _pUnkOuter = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        shim: shims::Shim {
            name: "DirectSoundCreate",
            func: DirectSoundCreate,
            stack_consumed: 16u32,
        },
        ordinal: Some(1usize),
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
        let _i = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::gdi32::GetStockObject(machine, _i);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) {
        let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
        let hGdiObj = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::gdi32::SelectObject(machine, hdc, hGdiObj);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) {
        let handle = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _bytes = <u32>::from_stack(machine.mem(), esp + 8u32);
        let _out = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) {
        let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::gdi32::CreateCompatibleDC(machine, hdc);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) {
        let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::gdi32::DeleteDC(machine, hdc);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) {
        let hdc = <u32>::from_stack(machine.mem(), esp + 4u32);
        let x = <u32>::from_stack(machine.mem(), esp + 8u32);
        let y = <u32>::from_stack(machine.mem(), esp + 12u32);
        let cx = <u32>::from_stack(machine.mem(), esp + 16u32);
        let cy = <u32>::from_stack(machine.mem(), esp + 20u32);
        let hdcSrc = <u32>::from_stack(machine.mem(), esp + 24u32);
        let x1 = <u32>::from_stack(machine.mem(), esp + 28u32);
        let y1 = <u32>::from_stack(machine.mem(), esp + 32u32);
        let rop = <u32>::from_stack(machine.mem(), esp + 36u32);
        let result = winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 40u32;
    }
    pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) {
        let hdcDest = <u32>::from_stack(machine.mem(), esp + 4u32);
        let xDest = <u32>::from_stack(machine.mem(), esp + 8u32);
        let yDest = <u32>::from_stack(machine.mem(), esp + 12u32);
        let wDest = <u32>::from_stack(machine.mem(), esp + 16u32);
        let hDest = <u32>::from_stack(machine.mem(), esp + 20u32);
        let hdcSrc = <u32>::from_stack(machine.mem(), esp + 24u32);
        let xSrc = <u32>::from_stack(machine.mem(), esp + 28u32);
        let ySrc = <u32>::from_stack(machine.mem(), esp + 32u32);
        let wSrc = <u32>::from_stack(machine.mem(), esp + 36u32);
        let hSrc = <u32>::from_stack(machine.mem(), esp + 40u32);
        let rop = <u32>::from_stack(machine.mem(), esp + 44u32);
        let result = winapi::gdi32::StretchBlt(
            machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 48u32;
    }
    const EXPORTS: [Symbol; 7usize] = [
        Symbol {
            shim: shims::Shim {
                name: "GetStockObject",
                func: GetStockObject,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SelectObject",
                func: SelectObject,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetObjectA",
                func: GetObjectA,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "CreateCompatibleDC",
                func: CreateCompatibleDC,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "DeleteDC",
                func: DeleteDC,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "BitBlt",
                func: BitBlt,
                stack_consumed: 40u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "StretchBlt",
                func: StretchBlt,
                stack_consumed: 48u32,
            },
            ordinal: None,
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
        let lpModuleName = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetModuleHandleA(machine, lpModuleName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) {
        let lpModuleName = <Option<Str16>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetModuleHandleW(machine, lpModuleName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) {
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 4u32);
        let lpModuleName = <Option<Str16>>::from_stack(machine.mem(), esp + 8u32);
        let hModule = <Option<&mut HMODULE>>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) {
        let filename = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::LoadLibraryA(machine, filename);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) {
        let lpLibFileName = <Option<Str16>>::from_stack(machine.mem(), esp + 4u32);
        let hFile = <HFILE>::from_stack(machine.mem(), esp + 8u32);
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) {
        let hModule = <HMODULE>::from_stack(machine.mem(), esp + 4u32);
        let lpProcName = <Option<&str>>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::GetProcAddress(machine, hModule, lpProcName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) {
        let nStdHandle = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetStdHandle(machine, nStdHandle);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) {
        let lpFileName = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
        let dwDesiredAccess = <u32>::from_stack(machine.mem(), esp + 8u32);
        let dwShareMode = <u32>::from_stack(machine.mem(), esp + 12u32);
        let lpSecurityAttributes = <u32>::from_stack(machine.mem(), esp + 16u32);
        let dwCreationDisposition =
            <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + 20u32);
        let dwFlagsAndAttributes = <u32>::from_stack(machine.mem(), esp + 24u32);
        let hTemplateFile = <HFILE>::from_stack(machine.mem(), esp + 28u32);
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
        machine.x86.cpu.regs.esp += 32u32;
    }
    pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) {
        let lpFileName = <Option<Str16>>::from_stack(machine.mem(), esp + 4u32);
        let dwDesiredAccess = <u32>::from_stack(machine.mem(), esp + 8u32);
        let dwShareMode = <u32>::from_stack(machine.mem(), esp + 12u32);
        let lpSecurityAttributes = <u32>::from_stack(machine.mem(), esp + 16u32);
        let dwCreationDisposition =
            <Result<CreationDisposition, u32>>::from_stack(machine.mem(), esp + 20u32);
        let dwFlagsAndAttributes = <u32>::from_stack(machine.mem(), esp + 24u32);
        let hTemplateFile = <HFILE>::from_stack(machine.mem(), esp + 28u32);
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
        machine.x86.cpu.regs.esp += 32u32;
    }
    pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) {
        let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetFileType(machine, hFile);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) {
        let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
        let lDistanceToMove = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpDistanceToMoveHigh = <Option<&mut u32>>::from_stack(machine.mem(), esp + 12u32);
        let dwMoveMethod = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::kernel32::SetFilePointer(
            machine,
            hFile,
            lDistanceToMove,
            lpDistanceToMoveHigh,
            dwMoveMethod,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) {
        let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
        let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(machine.mem(), esp + 8u32);
        let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
        let lpOverlapped = <u32>::from_stack(machine.mem(), esp + 20u32);
        let result =
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 24u32;
    }
    pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) {
        let hFile = <HFILE>::from_stack(machine.mem(), esp + 4u32);
        let lpBuffer = <ArrayWithSize<u8>>::from_stack(machine.mem(), esp + 8u32);
        let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
        let lpOverlapped = <u32>::from_stack(machine.mem(), esp + 20u32);
        let result = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 24u32;
    }
    pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) {
        let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
        let dwBytes = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) {
        let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpMem = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) {
        let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpMem = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) {
        let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpMem = <u32>::from_stack(machine.mem(), esp + 12u32);
        let dwBytes = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) {
        let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(machine.mem(), esp + 4u32);
        let dwInitialSize = <u32>::from_stack(machine.mem(), esp + 8u32);
        let dwMaximumSize = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) {
        let hHeap = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::HeapDestroy(machine, hHeap);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) {
        let lpAddress = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwSize = <u32>::from_stack(machine.mem(), esp + 8u32);
        let _flAllocationType = <u32>::from_stack(machine.mem(), esp + 12u32);
        let _flProtec = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::kernel32::VirtualAlloc(
            machine,
            lpAddress,
            dwSize,
            _flAllocationType,
            _flProtec,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) {
        let lpAddress = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwSize = <u32>::from_stack(machine.mem(), esp + 8u32);
        let dwFreeType = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) {
        let lp = <u32>::from_stack(machine.mem(), esp + 4u32);
        let ucb = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::IsBadReadPtr(machine, lp, ucb);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) {
        let lp = <u32>::from_stack(machine.mem(), esp + 4u32);
        let ucb = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::IsBadWritePtr(machine, lp, ucb);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) {
        let dwErrCode = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::SetLastError(machine, dwErrCode);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetLastError(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) {
        let uExitCode = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::ExitProcess(machine, uExitCode);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetACP(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetACP(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) {
        let CodePage = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::IsValidCodePage(machine, CodePage);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) {
        let _CodePage = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _lpCPInfo = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetCommandLineA(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetCommandLineW(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetEnvironmentStrings(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) {
        let _penv = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetEnvironmentStringsW(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) {
        let name = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
        let buf = <ArrayWithSize<u8>>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::GetEnvironmentVariableA(machine, name, buf);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) {
        let hModule = <HMODULE>::from_stack(machine.mem(), esp + 4u32);
        let filename = <ArrayWithSizeMut<u8>>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::GetModuleFileNameA(machine, hModule, filename);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) {
        let hModule = <HMODULE>::from_stack(machine.mem(), esp + 4u32);
        let _lpFilename = <u32>::from_stack(machine.mem(), esp + 8u32);
        let _nSize = <u32>::from_stack(machine.mem(), esp + 12u32);
        let result = winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 16u32;
    }
    pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) {
        let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) {
        let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) {
        let feature = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::IsProcessorFeaturePresent(machine, feature);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::IsDebuggerPresent(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetCurrentProcessId(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetTickCount(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) {
        let lpPerformanceCount =
            <Option<&mut LARGE_INTEGER>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) {
        let lpFrequency = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) {
        let _time = <Option<&mut FILETIME>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetVersion(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) {
        let lpVersionInformation =
            <Option<&mut OSVERSIONINFO>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::GetVersionExA(machine, lpVersionInformation);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetProcessHeap(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) {
        let uNumber = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::SetHandleCount(machine, uNumber);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) {
        let msg = <Option<&str>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::OutputDebugStringA(machine, msg);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine, esp: u32) {
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _dwSpinCount = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) {
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) {
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) {
        let _lpCriticalSection = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) {
        let _lpTopLevelExceptionFilter = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) {
        let _exceptionInfo = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::NtCurrentTeb(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) {
        let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::InitializeSListHead(machine, ListHead);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) {
        let CodePage = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwFlags = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpMultiByteStr = <u32>::from_stack(machine.mem(), esp + 12u32);
        let cbMultiByte = <i32>::from_stack(machine.mem(), esp + 16u32);
        let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(machine.mem(), esp + 20u32);
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
        machine.x86.cpu.regs.esp += 28u32;
    }
    pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) {
        let hConsoleOutput = <HFILE>::from_stack(machine.mem(), esp + 4u32);
        let lpBuffer = <ArrayWithSize<u16>>::from_stack(machine.mem(), esp + 8u32);
        let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(machine.mem(), esp + 16u32);
        let _lpReserved = <u32>::from_stack(machine.mem(), esp + 20u32);
        let result = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        );
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 24u32;
    }
    pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::GetCurrentThreadId(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) {
        let result = winapi::kernel32::TlsAlloc(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) {
        let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::TlsFree(machine, dwTlsIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) {
        let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
        let lpTlsValue = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) {
        let dwTlsIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::TlsGetValue(machine, dwTlsIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn CreateThread(machine: &mut Machine, esp: u32) {
        let lpThreadAttributes = <u32>::from_stack(machine.mem(), esp + 4u32);
        let dwStackSize = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpStartAddress = <u32>::from_stack(machine.mem(), esp + 12u32);
        let lpParameter = <u32>::from_stack(machine.mem(), esp + 16u32);
        let dwCreationFlags = <u32>::from_stack(machine.mem(), esp + 20u32);
        let lpThreadId = <u32>::from_stack(machine.mem(), esp + 24u32);
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
        machine.x86.cpu.regs.esp += 28u32;
    }
    pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) {
        let _hThread = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _nPriority = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) {
        let addend = <Option<&mut u32>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::kernel32::InterlockedIncrement(machine, addend);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    const EXPORTS: [Symbol; 69usize] = [
        Symbol {
            shim: shims::Shim {
                name: "GetModuleHandleA",
                func: GetModuleHandleA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetModuleHandleW",
                func: GetModuleHandleW,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetModuleHandleExW",
                func: GetModuleHandleExW,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "LoadLibraryA",
                func: LoadLibraryA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "LoadLibraryExW",
                func: LoadLibraryExW,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetProcAddress",
                func: GetProcAddress,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetStdHandle",
                func: GetStdHandle,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "CreateFileA",
                func: CreateFileA,
                stack_consumed: 32u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "CreateFileW",
                func: CreateFileW,
                stack_consumed: 32u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetFileType",
                func: GetFileType,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetFilePointer",
                func: SetFilePointer,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "ReadFile",
                func: ReadFile,
                stack_consumed: 24u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "WriteFile",
                func: WriteFile,
                stack_consumed: 24u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "HeapAlloc",
                func: HeapAlloc,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "HeapFree",
                func: HeapFree,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "HeapSize",
                func: HeapSize,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "HeapReAlloc",
                func: HeapReAlloc,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "HeapCreate",
                func: HeapCreate,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "HeapDestroy",
                func: HeapDestroy,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "VirtualAlloc",
                func: VirtualAlloc,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "VirtualFree",
                func: VirtualFree,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "IsBadReadPtr",
                func: IsBadReadPtr,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "IsBadWritePtr",
                func: IsBadWritePtr,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetLastError",
                func: SetLastError,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetLastError",
                func: GetLastError,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "ExitProcess",
                func: ExitProcess,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetACP",
                func: GetACP,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "IsValidCodePage",
                func: IsValidCodePage,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetCPInfo",
                func: GetCPInfo,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetCommandLineA",
                func: GetCommandLineA,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetCommandLineW",
                func: GetCommandLineW,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetEnvironmentStrings",
                func: GetEnvironmentStrings,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "FreeEnvironmentStringsA",
                func: FreeEnvironmentStringsA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetEnvironmentStringsW",
                func: GetEnvironmentStringsW,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetEnvironmentVariableA",
                func: GetEnvironmentVariableA,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetModuleFileNameA",
                func: GetModuleFileNameA,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetModuleFileNameW",
                func: GetModuleFileNameW,
                stack_consumed: 16u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetStartupInfoA",
                func: GetStartupInfoA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetStartupInfoW",
                func: GetStartupInfoW,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "IsProcessorFeaturePresent",
                func: IsProcessorFeaturePresent,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "IsDebuggerPresent",
                func: IsDebuggerPresent,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetCurrentProcessId",
                func: GetCurrentProcessId,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetTickCount",
                func: GetTickCount,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "QueryPerformanceCounter",
                func: QueryPerformanceCounter,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "QueryPerformanceFrequency",
                func: QueryPerformanceFrequency,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetSystemTimeAsFileTime",
                func: GetSystemTimeAsFileTime,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetVersion",
                func: GetVersion,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetVersionExA",
                func: GetVersionExA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetProcessHeap",
                func: GetProcessHeap,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetHandleCount",
                func: SetHandleCount,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "OutputDebugStringA",
                func: OutputDebugStringA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "InitializeCriticalSectionAndSpinCount",
                func: InitializeCriticalSectionAndSpinCount,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "DeleteCriticalSection",
                func: DeleteCriticalSection,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "EnterCriticalSection",
                func: EnterCriticalSection,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "LeaveCriticalSection",
                func: LeaveCriticalSection,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetUnhandledExceptionFilter",
                func: SetUnhandledExceptionFilter,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "UnhandledExceptionFilter",
                func: UnhandledExceptionFilter,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "NtCurrentTeb",
                func: NtCurrentTeb,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "InitializeSListHead",
                func: InitializeSListHead,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "MultiByteToWideChar",
                func: MultiByteToWideChar,
                stack_consumed: 28u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "WriteConsoleW",
                func: WriteConsoleW,
                stack_consumed: 24u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetCurrentThreadId",
                func: GetCurrentThreadId,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "TlsAlloc",
                func: TlsAlloc,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "TlsFree",
                func: TlsFree,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "TlsSetValue",
                func: TlsSetValue,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "TlsGetValue",
                func: TlsGetValue,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "CreateThread",
                func: CreateThread,
                stack_consumed: 28u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetThreadPriority",
                func: SetThreadPriority,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "InterlockedIncrement",
                func: InterlockedIncrement,
                stack_consumed: 8u32,
            },
            ordinal: None,
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
        let lpWndClass = <Option<&WNDCLASSA>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::RegisterClassA(machine, lpWndClass);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) {
        let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::RegisterClassExA(machine, lpWndClassEx);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn CreateWindowExA(machine: &mut Machine, esp: u32) {
        let dwExStyle = <u32>::from_stack(machine.mem(), esp + 4u32);
        let lpClassName = <u32>::from_stack(machine.mem(), esp + 8u32);
        let lpWindowName = <Option<&str>>::from_stack(machine.mem(), esp + 12u32);
        let dwStyle = <Result<WindowStyle, u32>>::from_stack(machine.mem(), esp + 16u32);
        let X = <u32>::from_stack(machine.mem(), esp + 20u32);
        let Y = <u32>::from_stack(machine.mem(), esp + 24u32);
        let nWidth = <u32>::from_stack(machine.mem(), esp + 28u32);
        let nHeight = <u32>::from_stack(machine.mem(), esp + 32u32);
        let hWndParent = <HWND>::from_stack(machine.mem(), esp + 36u32);
        let hMenu = <u32>::from_stack(machine.mem(), esp + 40u32);
        let hInstance = <u32>::from_stack(machine.mem(), esp + 44u32);
        let lpParam = <u32>::from_stack(machine.mem(), esp + 48u32);
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
            machine.x86.cpu.regs.esp += 52u32;
        };
        crate::shims::become_async(machine, Box::pin(result));
    }
    pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) {
        let result = winapi::user32::GetForegroundWindow(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) {
        let result = winapi::user32::GetActiveWindow(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) {
        let result = winapi::user32::GetLastActivePopup(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn UpdateWindow(machine: &mut Machine, esp: u32) {
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::UpdateWindow(machine, hWnd);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn ShowWindow(machine: &mut Machine, esp: u32) {
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
        let _nCmdShow = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::user32::ShowWindow(machine, hWnd, _nCmdShow);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) {
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::SetFocus(machine, hWnd);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) {
        let hCursor = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::SetCursor(machine, hCursor);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) {
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
        let lpText = <Option<&str>>::from_stack(machine.mem(), esp + 8u32);
        let lpCaption = <Option<&str>>::from_stack(machine.mem(), esp + 12u32);
        let uType = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) {
        let hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
        let lpTemplateName = <u32>::from_stack(machine.mem(), esp + 8u32);
        let hWndParent = <HWND>::from_stack(machine.mem(), esp + 12u32);
        let lpDialogFunc = <u32>::from_stack(machine.mem(), esp + 16u32);
        let dwInitParam = <u32>::from_stack(machine.mem(), esp + 20u32);
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
        machine.x86.cpu.regs.esp += 24u32;
    }
    pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) {
        let lpMsg = <Option<&mut MSG>>::from_stack(machine.mem(), esp + 4u32);
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 8u32);
        let wMsgFilterMin = <u32>::from_stack(machine.mem(), esp + 12u32);
        let wMsgFilterMax = <u32>::from_stack(machine.mem(), esp + 16u32);
        let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(machine.mem(), esp + 20u32);
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
        machine.x86.cpu.regs.esp += 24u32;
    }
    pub unsafe fn GetMessageA(machine: &mut Machine, esp: u32) {
        let lpMsg = <Option<&mut MSG>>::from_stack(machine.mem(), esp + 4u32);
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 8u32);
        let wMsgFilterMin = <u32>::from_stack(machine.mem(), esp + 12u32);
        let wMsgFilterMax = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result =
            winapi::user32::GetMessageA(machine, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) {
        let result = winapi::user32::WaitMessage(machine);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 4u32;
    }
    pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) {
        let lpMsg = <Option<&MSG>>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::TranslateMessage(machine, lpMsg);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn DispatchMessageA(machine: &mut Machine, esp: u32) {
        let lpMsg = <Option<&MSG>>::from_stack(machine.mem(), esp + 4u32);
        let m: *mut Machine = machine;
        let result = async move {
            let machine = unsafe { &mut *m };
            let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
            machine.x86.cpu.regs.eax = result.to_raw();
            machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
            machine.x86.cpu.regs.esp += 8u32;
        };
        crate::shims::become_async(machine, Box::pin(result));
    }
    pub unsafe fn DefWindowProcA(machine: &mut Machine, esp: u32) {
        let hWnd = <HWND>::from_stack(machine.mem(), esp + 4u32);
        let msg = <Result<WM, u32>>::from_stack(machine.mem(), esp + 8u32);
        let wParam = <u32>::from_stack(machine.mem(), esp + 12u32);
        let lParam = <u32>::from_stack(machine.mem(), esp + 16u32);
        let result = winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 20u32;
    }
    pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) {
        let _hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _lpIconName = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) {
        let _hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _lpCursorName = <u32>::from_stack(machine.mem(), esp + 8u32);
        let result = winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 12u32;
    }
    pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) {
        let _bShow = <bool>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::ShowCursor(machine, _bShow);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) {
        let hInstance = <u32>::from_stack(machine.mem(), esp + 4u32);
        let name = <u32>::from_stack(machine.mem(), esp + 8u32);
        let typ = <u32>::from_stack(machine.mem(), esp + 12u32);
        let cx = <u32>::from_stack(machine.mem(), esp + 16u32);
        let cy = <u32>::from_stack(machine.mem(), esp + 20u32);
        let fuLoad = <u32>::from_stack(machine.mem(), esp + 24u32);
        let result = winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 28u32;
    }
    pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) {
        let nIndex = <u32>::from_stack(machine.mem(), esp + 4u32);
        let result = winapi::user32::GetSystemMetrics(machine, nIndex);
        machine.x86.cpu.regs.eax = result.to_raw();
        machine.x86.cpu.regs.eip = machine.mem().get::<u32>(esp);
        machine.x86.cpu.regs.esp += 8u32;
    }
    const EXPORTS: [Symbol; 23usize] = [
        Symbol {
            shim: shims::Shim {
                name: "RegisterClassA",
                func: RegisterClassA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "RegisterClassExA",
                func: RegisterClassExA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "CreateWindowExA",
                func: CreateWindowExA,
                stack_consumed: 52u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetForegroundWindow",
                func: GetForegroundWindow,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetActiveWindow",
                func: GetActiveWindow,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetLastActivePopup",
                func: GetLastActivePopup,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "UpdateWindow",
                func: UpdateWindow,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "ShowWindow",
                func: ShowWindow,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetFocus",
                func: SetFocus,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "SetCursor",
                func: SetCursor,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "MessageBoxA",
                func: MessageBoxA,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "DialogBoxParamA",
                func: DialogBoxParamA,
                stack_consumed: 24u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "PeekMessageA",
                func: PeekMessageA,
                stack_consumed: 24u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetMessageA",
                func: GetMessageA,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "WaitMessage",
                func: WaitMessage,
                stack_consumed: 4u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "TranslateMessage",
                func: TranslateMessage,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "DispatchMessageA",
                func: DispatchMessageA,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "DefWindowProcA",
                func: DefWindowProcA,
                stack_consumed: 20u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "LoadIconA",
                func: LoadIconA,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "LoadCursorA",
                func: LoadCursorA,
                stack_consumed: 12u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "ShowCursor",
                func: ShowCursor,
                stack_consumed: 8u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "LoadImageA",
                func: LoadImageA,
                stack_consumed: 28u32,
            },
            ordinal: None,
        },
        Symbol {
            shim: shims::Shim {
                name: "GetSystemMetrics",
                func: GetSystemMetrics,
                stack_consumed: 8u32,
            },
            ordinal: None,
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
        let _uDelay = <u32>::from_stack(machine.mem(), esp + 4u32);
        let _uResolution = <u32>::from_stack(machine.mem(), esp + 8u32);
        let _lpTimeProc = <u32>::from_stack(machine.mem(), esp + 12u32);
        let _dwUser = <u32>::from_stack(machine.mem(), esp + 16u32);
        let _fuEvent = <u32>::from_stack(machine.mem(), esp + 20u32);
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
        machine.x86.cpu.regs.esp += 24u32;
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        shim: shims::Shim {
            name: "timeSetEvent",
            func: timeSetEvent,
            stack_consumed: 24u32,
        },
        ordinal: None,
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
    };
}
