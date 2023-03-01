#![allow(non_snake_case)]
#![allow(unused_imports)]
#[doc = r" Generated code, do not edit."]
use crate::{
    machine::Machine,
    winapi,
    winapi::shims::{from_x86, ToX86},
    winapi::types::*,
    winapi::BuiltinDLL,
};
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    pub fn DirectDrawCreate(machine: &mut Machine) {
        let lpGuid: u32 = unsafe { from_x86(&mut machine.x86) };
        let lplpDD: u32 = unsafe { from_x86(&mut machine.x86) };
        let pUnkOuter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw();
    }
    pub fn DirectDrawCreateEx(machine: &mut Machine) {
        let lpGuid: u32 = unsafe { from_x86(&mut machine.x86) };
        let lplpDD: u32 = unsafe { from_x86(&mut machine.x86) };
        let iid: u32 = unsafe { from_x86(&mut machine.x86) };
        let pUnkOuter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw();
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
        let _lpGuid: u32 = unsafe { from_x86(&mut machine.x86) };
        let ppDS: u32 = unsafe { from_x86(&mut machine.x86) };
        let _pUnkOuter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::dsound::DirectSoundCreate(machine, _lpGuid, ppDS, _pUnkOuter).to_raw();
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
        let _i: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::GetStockObject(machine, _i).to_raw();
    }
    pub fn SelectObject(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        let hGdiObj: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw();
    }
    pub fn GetObjectA(machine: &mut Machine) {
        let handle: u32 = unsafe { from_x86(&mut machine.x86) };
        let _bytes: u32 = unsafe { from_x86(&mut machine.x86) };
        let _out: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out).to_raw();
    }
    pub fn CreateCompatibleDC(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw();
    }
    pub fn DeleteDC(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::DeleteDC(machine, hdc).to_raw();
    }
    pub fn BitBlt(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        let x: u32 = unsafe { from_x86(&mut machine.x86) };
        let y: u32 = unsafe { from_x86(&mut machine.x86) };
        let cx: u32 = unsafe { from_x86(&mut machine.x86) };
        let cy: u32 = unsafe { from_x86(&mut machine.x86) };
        let hdcSrc: u32 = unsafe { from_x86(&mut machine.x86) };
        let x1: u32 = unsafe { from_x86(&mut machine.x86) };
        let y1: u32 = unsafe { from_x86(&mut machine.x86) };
        let rop: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw();
    }
    pub fn StretchBlt(machine: &mut Machine) {
        let hdcDest: u32 = unsafe { from_x86(&mut machine.x86) };
        let xDest: u32 = unsafe { from_x86(&mut machine.x86) };
        let yDest: u32 = unsafe { from_x86(&mut machine.x86) };
        let wDest: u32 = unsafe { from_x86(&mut machine.x86) };
        let hDest: u32 = unsafe { from_x86(&mut machine.x86) };
        let hdcSrc: u32 = unsafe { from_x86(&mut machine.x86) };
        let xSrc: u32 = unsafe { from_x86(&mut machine.x86) };
        let ySrc: u32 = unsafe { from_x86(&mut machine.x86) };
        let wSrc: u32 = unsafe { from_x86(&mut machine.x86) };
        let hSrc: u32 = unsafe { from_x86(&mut machine.x86) };
        let rop: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::StretchBlt(
            machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        )
        .to_raw();
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
        let dwErrCode: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::SetLastError(machine, dwErrCode).to_raw();
    }
    pub fn GetLastError(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetLastError(machine).to_raw();
    }
    pub fn ExitProcess(machine: &mut Machine) {
        let uExitCode: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::ExitProcess(machine, uExitCode).to_raw();
    }
    pub fn GetACP(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetACP(machine).to_raw();
    }
    pub fn IsValidCodePage(machine: &mut Machine) {
        let CodePage: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw();
    }
    pub fn GetCPInfo(machine: &mut Machine) {
        let _CodePage: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpCPInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw();
    }
    pub fn GetCommandLineA(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCommandLineA(machine).to_raw();
    }
    pub fn GetCommandLineW(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCommandLineW(machine).to_raw();
    }
    pub fn GetEnvironmentStrings(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetEnvironmentStrings(machine).to_raw();
    }
    pub fn FreeEnvironmentStringsA(machine: &mut Machine) {
        let _penv: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw();
    }
    pub fn GetEnvironmentStringsW(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetEnvironmentStringsW(machine).to_raw();
    }
    pub fn GetEnvironmentVariableA(machine: &mut Machine) {
        let name: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        let buf: Option<&mut [u8]> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw();
    }
    pub fn GetFileType(machine: &mut Machine) {
        let hFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetFileType(machine, hFile).to_raw();
    }
    pub fn GetModuleFileNameA(machine: &mut Machine) {
        let hModule: HMODULE = unsafe { from_x86(&mut machine.x86) };
        let filename: Option<&mut [u8]> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw();
    }
    pub fn GetModuleFileNameW(machine: &mut Machine) {
        let hModule: HMODULE = unsafe { from_x86(&mut machine.x86) };
        let _lpFilename: u32 = unsafe { from_x86(&mut machine.x86) };
        let _nSize: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw();
    }
    pub fn GetModuleHandleA(machine: &mut Machine) {
        let lpModuleName: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw();
    }
    pub fn GetModuleHandleW(machine: &mut Machine) {
        let lpModuleName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw();
    }
    pub fn GetModuleHandleExW(machine: &mut Machine) {
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpModuleName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        let hModule: Option<&mut HMODULE> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw();
    }
    pub fn GetStartupInfoA(machine: &mut Machine) {
        let lpStartupInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw();
    }
    pub fn GetStartupInfoW(machine: &mut Machine) {
        let lpStartupInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw();
    }
    pub fn IsProcessorFeaturePresent(machine: &mut Machine) {
        let feature: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw();
    }
    pub fn IsDebuggerPresent(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::IsDebuggerPresent(machine).to_raw();
    }
    pub fn GetCurrentThreadId(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCurrentThreadId(machine).to_raw();
    }
    pub fn GetCurrentProcessId(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCurrentProcessId(machine).to_raw();
    }
    pub fn GetStdHandle(machine: &mut Machine) {
        let nStdHandle: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw();
    }
    pub fn GetTickCount(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetTickCount(machine).to_raw();
    }
    pub fn QueryPerformanceCounter(machine: &mut Machine) {
        let _ptr: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::QueryPerformanceCounter(machine, _ptr).to_raw();
    }
    pub fn GetSystemTimeAsFileTime(machine: &mut Machine) {
        let _time: Option<&mut FILETIME> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time).to_raw();
    }
    pub fn GetVersion(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetVersion(machine).to_raw();
    }
    pub fn GetVersionExA(machine: &mut Machine) {
        let lpVersionInformation: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw();
    }
    pub fn HeapAlloc(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwBytes: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw();
    }
    pub fn HeapFree(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMem: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw();
    }
    pub fn HeapSize(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMem: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw();
    }
    pub fn HeapReAlloc(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMem: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwBytes: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw();
    }
    pub fn HeapCreate(machine: &mut Machine) {
        let flOptions: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwInitialSize: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwMaximumSize: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, _dwMaximumSize)
                .to_raw();
    }
    pub fn HeapDestroy(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::HeapDestroy(machine, hHeap).to_raw();
    }
    pub fn GetProcessHeap(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetProcessHeap(machine).to_raw();
    }
    pub fn LoadLibraryA(machine: &mut Machine) {
        let filename: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::LoadLibraryA(machine, filename).to_raw();
    }
    pub fn LoadLibraryExW(machine: &mut Machine) {
        let lpLibFileName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        let hFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw();
    }
    pub fn GetProcAddress(machine: &mut Machine) {
        let hModule: HMODULE = unsafe { from_x86(&mut machine.x86) };
        let lpProcName: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw();
    }
    pub fn SetHandleCount(machine: &mut Machine) {
        let uNumber: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::SetHandleCount(machine, uNumber).to_raw();
    }
    pub fn CreateFileW(machine: &mut Machine) {
        let lpFileName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        let dwDesiredAccess: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwShareMode: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpSecurityAttributes: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwCreationDisposition: Result<CreationDisposition, u32> =
            unsafe { from_x86(&mut machine.x86) };
        let dwFlagsAndAttributes: u32 = unsafe { from_x86(&mut machine.x86) };
        let hTemplateFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::CreateFileW(
            machine,
            lpFileName,
            dwDesiredAccess,
            _dwShareMode,
            _lpSecurityAttributes,
            dwCreationDisposition,
            dwFlagsAndAttributes,
            hTemplateFile,
        )
        .to_raw();
    }
    pub fn WriteFile(machine: &mut Machine) {
        let hFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        let lpBuffer: Option<&[u8]> = unsafe { from_x86(&mut machine.x86) };
        let lpNumberOfBytesWritten: Option<&mut u32> = unsafe { from_x86(&mut machine.x86) };
        let lpOverlapped: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::WriteFile(
            machine,
            hFile,
            lpBuffer,
            lpNumberOfBytesWritten,
            lpOverlapped,
        )
        .to_raw();
    }
    pub fn VirtualAlloc(machine: &mut Machine) {
        let lpAddress: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwSize: u32 = unsafe { from_x86(&mut machine.x86) };
        let _flAllocationType: u32 = unsafe { from_x86(&mut machine.x86) };
        let _flProtec: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::VirtualAlloc(
            machine,
            lpAddress,
            dwSize,
            _flAllocationType,
            _flProtec,
        )
        .to_raw();
    }
    pub fn VirtualFree(machine: &mut Machine) {
        let lpAddress: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwSize: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFreeType: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw();
    }
    pub fn OutputDebugStringA(machine: &mut Machine) {
        let msg: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::OutputDebugStringA(machine, msg).to_raw();
    }
    pub fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwSpinCount: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        )
        .to_raw();
    }
    pub fn DeleteCriticalSection(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection).to_raw();
    }
    pub fn EnterCriticalSection(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection).to_raw();
    }
    pub fn LeaveCriticalSection(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection).to_raw();
    }
    pub fn SetUnhandledExceptionFilter(machine: &mut Machine) {
        let _lpTopLevelExceptionFilter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw();
    }
    pub fn UnhandledExceptionFilter(machine: &mut Machine) {
        let _exceptionInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw();
    }
    pub fn NtCurrentTeb(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::NtCurrentTeb(machine).to_raw();
    }
    pub fn TlsAlloc(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::TlsAlloc(machine).to_raw();
    }
    pub fn TlsFree(machine: &mut Machine) {
        let dwTlsIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw();
    }
    pub fn TlsSetValue(machine: &mut Machine) {
        let dwTlsIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpTlsValue: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw();
    }
    pub fn TlsGetValue(machine: &mut Machine) {
        let dwTlsIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw();
    }
    pub fn InitializeSListHead(machine: &mut Machine) {
        let ListHead: Option<&mut SLIST_HEADER> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw();
    }
    pub fn MultiByteToWideChar(machine: &mut Machine) {
        let CodePage: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMultiByteStr: u32 = unsafe { from_x86(&mut machine.x86) };
        let cbMultiByte: i32 = unsafe { from_x86(&mut machine.x86) };
        let lpWideCharStr: Option<&mut [u16]> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::MultiByteToWideChar(
            machine,
            CodePage,
            _dwFlags,
            lpMultiByteStr,
            cbMultiByte,
            lpWideCharStr,
        )
        .to_raw();
    }
    pub fn WriteConsoleW(machine: &mut Machine) {
        let hConsoleOutput: HFILE = unsafe { from_x86(&mut machine.x86) };
        let lpBuffer: Option<&[u16]> = unsafe { from_x86(&mut machine.x86) };
        let lpNumberOfCharsWritten: Option<&mut u32> = unsafe { from_x86(&mut machine.x86) };
        let _lpReserved: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::WriteConsoleW(
            machine,
            hConsoleOutput,
            lpBuffer,
            lpNumberOfCharsWritten,
            _lpReserved,
        )
        .to_raw();
    }
    pub fn CreateThread(machine: &mut Machine) {
        let lpThreadAttributes: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwStackSize: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpStartAddress: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpParameter: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwCreationFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpThreadId: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::CreateThread(
            machine,
            lpThreadAttributes,
            dwStackSize,
            lpStartAddress,
            lpParameter,
            dwCreationFlags,
            lpThreadId,
        )
        .to_raw();
    }
    pub fn SetThreadPriority(machine: &mut Machine) {
        let _hThread: u32 = unsafe { from_x86(&mut machine.x86) };
        let _nPriority: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::SetThreadPriority(machine, _hThread, _nPriority).to_raw();
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
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    pub fn RegisterClassA(machine: &mut Machine) {
        let lpWndClass: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::RegisterClassA(machine, lpWndClass).to_raw();
    }
    pub fn CreateWindowExA(machine: &mut Machine) {
        let dwExStyle: u32 = unsafe { from_x86(&mut machine.x86) };
        let className: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        let windowName: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        let dwStyle: u32 = unsafe { from_x86(&mut machine.x86) };
        let X: u32 = unsafe { from_x86(&mut machine.x86) };
        let Y: u32 = unsafe { from_x86(&mut machine.x86) };
        let nWidth: u32 = unsafe { from_x86(&mut machine.x86) };
        let nHeight: u32 = unsafe { from_x86(&mut machine.x86) };
        let hWndParent: u32 = unsafe { from_x86(&mut machine.x86) };
        let hMenu: u32 = unsafe { from_x86(&mut machine.x86) };
        let hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpParam: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::CreateWindowExA(
            machine, dwExStyle, className, windowName, dwStyle, X, Y, nWidth, nHeight, hWndParent,
            hMenu, hInstance, lpParam,
        )
        .to_raw();
    }
    pub fn UpdateWindow(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::UpdateWindow(machine, _hWnd).to_raw();
    }
    pub fn ShowWindow(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        let _nCmdShow: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::ShowWindow(machine, _hWnd, _nCmdShow).to_raw();
    }
    pub fn SetFocus(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::SetFocus(machine, _hWnd).to_raw();
    }
    pub fn MessageBoxA(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpText: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        let lpCaption: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        let uType: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::user32::MessageBoxA(machine, _hWnd, lpText, lpCaption, uType).to_raw();
    }
    pub fn DialogBoxParamA(machine: &mut Machine) {
        let hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpTemplateName: u32 = unsafe { from_x86(&mut machine.x86) };
        let hWndParent: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpDialogFunc: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwInitParam: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::DialogBoxParamA(
            machine,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        )
        .to_raw();
    }
    pub fn PeekMessageA(machine: &mut Machine) {
        let _lpMsg: u32 = unsafe { from_x86(&mut machine.x86) };
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        let _wMsgFilterMin: u32 = unsafe { from_x86(&mut machine.x86) };
        let _wMsgFilterMax: u32 = unsafe { from_x86(&mut machine.x86) };
        let _wRemoveMs: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::PeekMessageA(
            machine,
            _lpMsg,
            _hWnd,
            _wMsgFilterMin,
            _wMsgFilterMax,
            _wRemoveMs,
        )
        .to_raw();
    }
    pub fn LoadIconA(machine: &mut Machine) {
        let _hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpIconName: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName).to_raw();
    }
    pub fn LoadCursorA(machine: &mut Machine) {
        let _hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpCursorName: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName).to_raw();
    }
    pub fn ShowCursor(machine: &mut Machine) {
        let _bShow: bool = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::ShowCursor(machine, _bShow).to_raw();
    }
    pub fn LoadImageA(machine: &mut Machine) {
        let hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let name: u32 = unsafe { from_x86(&mut machine.x86) };
        let typ: u32 = unsafe { from_x86(&mut machine.x86) };
        let _cx: u32 = unsafe { from_x86(&mut machine.x86) };
        let _cy: u32 = unsafe { from_x86(&mut machine.x86) };
        let fuLoad: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::user32::LoadImageA(machine, hInstance, name, typ, _cx, _cy, fuLoad).to_raw();
    }
    pub fn GetSystemMetrics(machine: &mut Machine) {
        let nIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::GetSystemMetrics(machine, nIndex).to_raw();
    }
    fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
        Some(match *sym {
            winapi::ImportSymbol::Name(name) => match name {
                "RegisterClassA" => RegisterClassA,
                "CreateWindowExA" => CreateWindowExA,
                "UpdateWindow" => UpdateWindow,
                "ShowWindow" => ShowWindow,
                "SetFocus" => SetFocus,
                "MessageBoxA" => MessageBoxA,
                "DialogBoxParamA" => DialogBoxParamA,
                "PeekMessageA" => PeekMessageA,
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
        let _uDelay: u32 = unsafe { from_x86(&mut machine.x86) };
        let _uResolution: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpTimeProc: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwUser: u32 = unsafe { from_x86(&mut machine.x86) };
        let _fuEvent: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::winmm::timeSetEvent(
            machine,
            _uDelay,
            _uResolution,
            _lpTimeProc,
            _dwUser,
            _fuEvent,
        )
        .to_raw();
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
