#![allow(non_snake_case)]
#![allow(unused_imports)]
#[doc = r" Generated code, do not edit."]
use crate::{
    machine::Machine,
    winapi,
    winapi::shims::{from_x86, ToX86},
    winapi::types::*,
};
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    fn DirectDrawCreate(machine: &mut Machine) {
        let lpGuid: u32 = unsafe { from_x86(&mut machine.x86) };
        let lplpDD: u32 = unsafe { from_x86(&mut machine.x86) };
        let pUnkOuter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw();
    }
    fn DirectDrawCreateEx(machine: &mut Machine) {
        let lpGuid: u32 = unsafe { from_x86(&mut machine.x86) };
        let lplpDD: u32 = unsafe { from_x86(&mut machine.x86) };
        let iid: u32 = unsafe { from_x86(&mut machine.x86) };
        let pUnkOuter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw();
    }
    pub fn resolve(name: &str) -> Option<fn(&mut Machine)> {
        Some(match name {
            "DirectDrawCreate" => DirectDrawCreate,
            "DirectDrawCreateEx" => DirectDrawCreateEx,
            _ => return None,
        })
    }
}
pub mod gdi32 {
    use super::*;
    use winapi::gdi32::*;
    fn GetStockObject(machine: &mut Machine) {
        let _i: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::GetStockObject(machine, _i).to_raw();
    }
    fn SelectObject(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        let hGdiObj: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw();
    }
    fn GetObjectA(machine: &mut Machine) {
        let handle: u32 = unsafe { from_x86(&mut machine.x86) };
        let _bytes: u32 = unsafe { from_x86(&mut machine.x86) };
        let _out: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::GetObjectA(machine, handle, _bytes, _out).to_raw();
    }
    fn CreateCompatibleDC(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw();
    }
    fn DeleteDC(machine: &mut Machine) {
        let hdc: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::gdi32::DeleteDC(machine, hdc).to_raw();
    }
    fn BitBlt(machine: &mut Machine) {
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
    fn StretchBlt(machine: &mut Machine) {
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
    pub fn resolve(name: &str) -> Option<fn(&mut Machine)> {
        Some(match name {
            "GetStockObject" => GetStockObject,
            "SelectObject" => SelectObject,
            "GetObjectA" => GetObjectA,
            "CreateCompatibleDC" => CreateCompatibleDC,
            "DeleteDC" => DeleteDC,
            "BitBlt" => BitBlt,
            "StretchBlt" => StretchBlt,
            _ => return None,
        })
    }
}
pub mod kernel32 {
    use super::*;
    use winapi::kernel32::*;
    fn SetLastError(machine: &mut Machine) {
        let dwErrCode: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::SetLastError(machine, dwErrCode).to_raw();
    }
    fn GetLastError(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetLastError(machine).to_raw();
    }
    fn ExitProcess(machine: &mut Machine) {
        let uExitCode: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::ExitProcess(machine, uExitCode).to_raw();
    }
    fn GetACP(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetACP(machine).to_raw();
    }
    fn IsValidCodePage(machine: &mut Machine) {
        let CodePage: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw();
    }
    fn GetCPInfo(machine: &mut Machine) {
        let _CodePage: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpCPInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw();
    }
    fn GetCommandLineA(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCommandLineA(machine).to_raw();
    }
    fn GetCommandLineW(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCommandLineW(machine).to_raw();
    }
    fn GetEnvironmentStrings(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetEnvironmentStrings(machine).to_raw();
    }
    fn FreeEnvironmentStringsA(machine: &mut Machine) {
        let _penv: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw();
    }
    fn GetEnvironmentStringsW(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetEnvironmentStringsW(machine).to_raw();
    }
    fn GetEnvironmentVariableA(machine: &mut Machine) {
        let name: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        let buf: &mut [u8] = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw();
    }
    fn GetFileType(machine: &mut Machine) {
        let hFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetFileType(machine, hFile).to_raw();
    }
    fn GetModuleFileNameA(machine: &mut Machine) {
        let hModule: HMODULE = unsafe { from_x86(&mut machine.x86) };
        let filename: &mut [u8] = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw();
    }
    fn GetModuleFileNameW(machine: &mut Machine) {
        let hModule: HMODULE = unsafe { from_x86(&mut machine.x86) };
        let _lpFilename: u32 = unsafe { from_x86(&mut machine.x86) };
        let _nSize: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw();
    }
    fn GetModuleHandleA(machine: &mut Machine) {
        let lpModuleName: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw();
    }
    fn GetModuleHandleW(machine: &mut Machine) {
        let lpModuleName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw();
    }
    fn GetModuleHandleExW(machine: &mut Machine) {
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpModuleName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        let hModule: Option<&mut HMODULE> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw();
    }
    fn GetStartupInfoA(machine: &mut Machine) {
        let lpStartupInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw();
    }
    fn GetStartupInfoW(machine: &mut Machine) {
        let lpStartupInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw();
    }
    fn IsProcessorFeaturePresent(machine: &mut Machine) {
        let feature: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw();
    }
    fn IsDebuggerPresent(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::IsDebuggerPresent(machine).to_raw();
    }
    fn GetCurrentThreadId(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCurrentThreadId(machine).to_raw();
    }
    fn GetCurrentProcessId(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetCurrentProcessId(machine).to_raw();
    }
    fn GetStdHandle(machine: &mut Machine) {
        let nStdHandle: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw();
    }
    fn GetTickCount(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetTickCount(machine).to_raw();
    }
    fn QueryPerformanceCounter(machine: &mut Machine) {
        let _ptr: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::QueryPerformanceCounter(machine, _ptr).to_raw();
    }
    fn GetSystemTimeAsFileTime(machine: &mut Machine) {
        let _time: Option<&mut FILETIME> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::GetSystemTimeAsFileTime(machine, _time).to_raw();
    }
    fn GetVersion(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetVersion(machine).to_raw();
    }
    fn GetVersionExA(machine: &mut Machine) {
        let lpVersionInformation: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw();
    }
    fn HeapAlloc(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwBytes: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw();
    }
    fn HeapFree(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMem: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw();
    }
    fn HeapSize(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMem: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw();
    }
    fn HeapReAlloc(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpMem: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwBytes: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw();
    }
    fn HeapCreate(machine: &mut Machine) {
        let flOptions: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwInitialSize: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwMaximumSize: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, _dwMaximumSize)
                .to_raw();
    }
    fn HeapDestroy(machine: &mut Machine) {
        let hHeap: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::HeapDestroy(machine, hHeap).to_raw();
    }
    fn GetProcessHeap(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::GetProcessHeap(machine).to_raw();
    }
    fn LoadLibraryA(machine: &mut Machine) {
        let filename: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::LoadLibraryA(machine, filename).to_raw();
    }
    fn LoadLibraryExW(machine: &mut Machine) {
        let lpLibFileName: Option<Str16> = unsafe { from_x86(&mut machine.x86) };
        let hFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        let dwFlags: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw();
    }
    fn SetHandleCount(machine: &mut Machine) {
        let uNumber: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::SetHandleCount(machine, uNumber).to_raw();
    }
    fn CreateFileW(machine: &mut Machine) {
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
    fn WriteFile(machine: &mut Machine) {
        let hFile: HFILE = unsafe { from_x86(&mut machine.x86) };
        let lpBuffer: &[u8] = unsafe { from_x86(&mut machine.x86) };
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
    fn VirtualAlloc(machine: &mut Machine) {
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
    fn VirtualFree(machine: &mut Machine) {
        let lpAddress: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwSize: u32 = unsafe { from_x86(&mut machine.x86) };
        let dwFreeType: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw();
    }
    fn OutputDebugStringA(machine: &mut Machine) {
        let msg: Option<&str> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::OutputDebugStringA(machine, msg).to_raw();
    }
    fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        let _dwSpinCount: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            machine,
            _lpCriticalSection,
            _dwSpinCount,
        )
        .to_raw();
    }
    fn DeleteCriticalSection(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::DeleteCriticalSection(machine, _lpCriticalSection).to_raw();
    }
    fn EnterCriticalSection(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::EnterCriticalSection(machine, _lpCriticalSection).to_raw();
    }
    fn LeaveCriticalSection(machine: &mut Machine) {
        let _lpCriticalSection: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::LeaveCriticalSection(machine, _lpCriticalSection).to_raw();
    }
    fn SetUnhandledExceptionFilter(machine: &mut Machine) {
        let _lpTopLevelExceptionFilter: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw();
    }
    fn UnhandledExceptionFilter(machine: &mut Machine) {
        let _exceptionInfo: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw();
    }
    fn NtCurrentTeb(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::NtCurrentTeb(machine).to_raw();
    }
    fn TlsAlloc(machine: &mut Machine) {
        machine.x86.regs.eax = winapi::kernel32::TlsAlloc(machine).to_raw();
    }
    fn TlsFree(machine: &mut Machine) {
        let dwTlsIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw();
    }
    fn TlsSetValue(machine: &mut Machine) {
        let dwTlsIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpTlsValue: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw();
    }
    fn TlsGetValue(machine: &mut Machine) {
        let dwTlsIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw();
    }
    fn InitializeSListHead(machine: &mut Machine) {
        let ListHead: Option<&mut SLIST_HEADER> = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw();
    }
    fn MultiByteToWideChar(machine: &mut Machine) {
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
    fn WriteConsoleW(machine: &mut Machine) {
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
    pub fn resolve(name: &str) -> Option<fn(&mut Machine)> {
        Some(match name {
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
            _ => return None,
        })
    }
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    fn RegisterClassA(machine: &mut Machine) {
        let lpWndClass: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::RegisterClassA(machine, lpWndClass).to_raw();
    }
    fn CreateWindowExA(machine: &mut Machine) {
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
    fn UpdateWindow(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::UpdateWindow(machine, _hWnd).to_raw();
    }
    fn ShowWindow(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        let _nCmdShow: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::ShowWindow(machine, _hWnd, _nCmdShow).to_raw();
    }
    fn SetFocus(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::SetFocus(machine, _hWnd).to_raw();
    }
    fn MessageBoxA(machine: &mut Machine) {
        let _hWnd: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpText: u32 = unsafe { from_x86(&mut machine.x86) };
        let lpCaption: u32 = unsafe { from_x86(&mut machine.x86) };
        let _uType: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::user32::MessageBoxA(machine, _hWnd, lpText, lpCaption, _uType).to_raw();
    }
    fn DialogBoxParamA(machine: &mut Machine) {
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
    fn PeekMessageA(machine: &mut Machine) {
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
    fn LoadIconA(machine: &mut Machine) {
        let _hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpIconName: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::LoadIconA(machine, _hInstance, _lpIconName).to_raw();
    }
    fn LoadCursorA(machine: &mut Machine) {
        let _hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let _lpCursorName: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::user32::LoadCursorA(machine, _hInstance, _lpCursorName).to_raw();
    }
    fn ShowCursor(machine: &mut Machine) {
        let _bShow: bool = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::ShowCursor(machine, _bShow).to_raw();
    }
    fn LoadImageA(machine: &mut Machine) {
        let hInstance: u32 = unsafe { from_x86(&mut machine.x86) };
        let name: u32 = unsafe { from_x86(&mut machine.x86) };
        let typ: u32 = unsafe { from_x86(&mut machine.x86) };
        let _cx: u32 = unsafe { from_x86(&mut machine.x86) };
        let _cy: u32 = unsafe { from_x86(&mut machine.x86) };
        let fuLoad: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax =
            winapi::user32::LoadImageA(machine, hInstance, name, typ, _cx, _cy, fuLoad).to_raw();
    }
    fn GetSystemMetrics(machine: &mut Machine) {
        let nIndex: u32 = unsafe { from_x86(&mut machine.x86) };
        machine.x86.regs.eax = winapi::user32::GetSystemMetrics(machine, nIndex).to_raw();
    }
    pub fn resolve(name: &str) -> Option<fn(&mut Machine)> {
        Some(match name {
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
        })
    }
}
