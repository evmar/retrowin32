#![allow(non_snake_case)]
#![allow(unused_imports)]
#[doc = r" Generated code, do not edit."]
use crate::{
    memory::Memory,
    winapi,
    winapi::shims::{from_x86, ToX86},
    winapi::types::*,
    x86::X86,
};
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    fn DirectDrawCreate(x86: &mut X86) {
        let lpGuid: u32 = unsafe { from_x86(x86) };
        let lplpDD: u32 = unsafe { from_x86(x86) };
        let pUnkOuter: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::ddraw::DirectDrawCreate(x86, lpGuid, lplpDD, pUnkOuter).to_raw();
    }
    fn DirectDrawCreateEx(x86: &mut X86) {
        let lpGuid: u32 = unsafe { from_x86(x86) };
        let lplpDD: u32 = unsafe { from_x86(x86) };
        let iid: u32 = unsafe { from_x86(x86) };
        let pUnkOuter: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::ddraw::DirectDrawCreateEx(x86, lpGuid, lplpDD, iid, pUnkOuter).to_raw();
    }
    pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
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
    fn GetStockObject(x86: &mut X86) {
        let _i: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::GetStockObject(x86, _i).to_raw();
    }
    fn SelectObject(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        let hGdiObj: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::SelectObject(x86, hdc, hGdiObj).to_raw();
    }
    fn GetObjectA(x86: &mut X86) {
        let handle: u32 = unsafe { from_x86(x86) };
        let _bytes: u32 = unsafe { from_x86(x86) };
        let _out: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::GetObjectA(x86, handle, _bytes, _out).to_raw();
    }
    fn CreateCompatibleDC(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::CreateCompatibleDC(x86, hdc).to_raw();
    }
    fn DeleteDC(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::DeleteDC(x86, hdc).to_raw();
    }
    fn BitBlt(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        let x: u32 = unsafe { from_x86(x86) };
        let y: u32 = unsafe { from_x86(x86) };
        let cx: u32 = unsafe { from_x86(x86) };
        let cy: u32 = unsafe { from_x86(x86) };
        let hdcSrc: u32 = unsafe { from_x86(x86) };
        let x1: u32 = unsafe { from_x86(x86) };
        let y1: u32 = unsafe { from_x86(x86) };
        let rop: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::BitBlt(x86, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw();
    }
    fn StretchBlt(x86: &mut X86) {
        let hdcDest: u32 = unsafe { from_x86(x86) };
        let xDest: u32 = unsafe { from_x86(x86) };
        let yDest: u32 = unsafe { from_x86(x86) };
        let wDest: u32 = unsafe { from_x86(x86) };
        let hDest: u32 = unsafe { from_x86(x86) };
        let hdcSrc: u32 = unsafe { from_x86(x86) };
        let xSrc: u32 = unsafe { from_x86(x86) };
        let ySrc: u32 = unsafe { from_x86(x86) };
        let wSrc: u32 = unsafe { from_x86(x86) };
        let hSrc: u32 = unsafe { from_x86(x86) };
        let rop: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::StretchBlt(
            x86, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
        )
        .to_raw();
    }
    pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
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
    fn SetLastError(x86: &mut X86) {
        let dwErrCode: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::SetLastError(x86, dwErrCode).to_raw();
    }
    fn GetLastError(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetLastError(x86).to_raw();
    }
    fn ExitProcess(x86: &mut X86) {
        let uExitCode: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::ExitProcess(x86, uExitCode).to_raw();
    }
    fn GetACP(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetACP(x86).to_raw();
    }
    fn IsValidCodePage(x86: &mut X86) {
        let CodePage: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::IsValidCodePage(x86, CodePage).to_raw();
    }
    fn GetCPInfo(x86: &mut X86) {
        let _CodePage: u32 = unsafe { from_x86(x86) };
        let _lpCPInfo: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetCPInfo(x86, _CodePage, _lpCPInfo).to_raw();
    }
    fn GetCommandLineA(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCommandLineA(x86).to_raw();
    }
    fn GetCommandLineW(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCommandLineW(x86).to_raw();
    }
    fn GetEnvironmentStrings(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetEnvironmentStrings(x86).to_raw();
    }
    fn FreeEnvironmentStringsA(x86: &mut X86) {
        let _penv: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::FreeEnvironmentStringsA(x86, _penv).to_raw();
    }
    fn GetEnvironmentStringsW(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetEnvironmentStringsW(x86).to_raw();
    }
    fn GetEnvironmentVariableA(x86: &mut X86) {
        let name: Option<&str> = unsafe { from_x86(x86) };
        let buf: &mut [u8] = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetEnvironmentVariableA(x86, name, buf).to_raw();
    }
    fn GetFileType(x86: &mut X86) {
        let hFile: HFILE = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetFileType(x86, hFile).to_raw();
    }
    fn GetModuleFileNameA(x86: &mut X86) {
        let hModule: HMODULE = unsafe { from_x86(x86) };
        let filename: &mut [u8] = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetModuleFileNameA(x86, hModule, filename).to_raw();
    }
    fn GetModuleFileNameW(x86: &mut X86) {
        let hModule: HMODULE = unsafe { from_x86(x86) };
        let _lpFilename: u32 = unsafe { from_x86(x86) };
        let _nSize: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::GetModuleFileNameW(x86, hModule, _lpFilename, _nSize).to_raw();
    }
    fn GetModuleHandleA(x86: &mut X86) {
        let lpModuleName: Option<&str> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetModuleHandleA(x86, lpModuleName).to_raw();
    }
    fn GetModuleHandleW(x86: &mut X86) {
        let lpModuleName: Option<Str16> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetModuleHandleW(x86, lpModuleName).to_raw();
    }
    fn GetModuleHandleExW(x86: &mut X86) {
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let lpModuleName: Option<Str16> = unsafe { from_x86(x86) };
        let hModule: Option<&mut HMODULE> = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::GetModuleHandleExW(x86, dwFlags, lpModuleName, hModule).to_raw();
    }
    fn GetStartupInfoA(x86: &mut X86) {
        let lpStartupInfo: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetStartupInfoA(x86, lpStartupInfo).to_raw();
    }
    fn GetStartupInfoW(x86: &mut X86) {
        let lpStartupInfo: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetStartupInfoW(x86, lpStartupInfo).to_raw();
    }
    fn IsProcessorFeaturePresent(x86: &mut X86) {
        let feature: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::IsProcessorFeaturePresent(x86, feature).to_raw();
    }
    fn IsDebuggerPresent(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::IsDebuggerPresent(x86).to_raw();
    }
    fn GetCurrentThreadId(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCurrentThreadId(x86).to_raw();
    }
    fn GetCurrentProcessId(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCurrentProcessId(x86).to_raw();
    }
    fn GetStdHandle(x86: &mut X86) {
        let nStdHandle: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetStdHandle(x86, nStdHandle).to_raw();
    }
    fn GetTickCount(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetTickCount(x86).to_raw();
    }
    fn QueryPerformanceCounter(x86: &mut X86) {
        let _ptr: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::QueryPerformanceCounter(x86, _ptr).to_raw();
    }
    fn GetSystemTimeAsFileTime(x86: &mut X86) {
        let _time: Option<&mut FILETIME> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetSystemTimeAsFileTime(x86, _time).to_raw();
    }
    fn GetVersion(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetVersion(x86).to_raw();
    }
    fn GetVersionExA(x86: &mut X86) {
        let lpVersionInformation: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetVersionExA(x86, lpVersionInformation).to_raw();
    }
    fn HeapAlloc(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let dwBytes: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapAlloc(x86, hHeap, dwFlags, dwBytes).to_raw();
    }
    fn HeapFree(x86: &mut X86) {
        let _hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let _lpMem: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapFree(x86, _hHeap, dwFlags, _lpMem).to_raw();
    }
    fn HeapSize(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let lpMem: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapSize(x86, hHeap, dwFlags, lpMem).to_raw();
    }
    fn HeapReAlloc(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let lpMem: u32 = unsafe { from_x86(x86) };
        let dwBytes: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapReAlloc(x86, hHeap, dwFlags, lpMem, dwBytes).to_raw();
    }
    fn HeapCreate(x86: &mut X86) {
        let flOptions: u32 = unsafe { from_x86(x86) };
        let dwInitialSize: u32 = unsafe { from_x86(x86) };
        let dwMaximumSize: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::HeapCreate(x86, flOptions, dwInitialSize, dwMaximumSize).to_raw();
    }
    fn HeapDestroy(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapDestroy(x86, hHeap).to_raw();
    }
    fn GetProcessHeap(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetProcessHeap(x86).to_raw();
    }
    fn LoadLibraryA(x86: &mut X86) {
        let filename: Option<&str> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::LoadLibraryA(x86, filename).to_raw();
    }
    fn LoadLibraryExW(x86: &mut X86) {
        let lpLibFileName: Option<Str16> = unsafe { from_x86(x86) };
        let hFile: HFILE = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::LoadLibraryExW(x86, lpLibFileName, hFile, dwFlags).to_raw();
    }
    fn SetHandleCount(x86: &mut X86) {
        let uNumber: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::SetHandleCount(x86, uNumber).to_raw();
    }
    fn WriteFile(x86: &mut X86) {
        let hFile: HFILE = unsafe { from_x86(x86) };
        let lpBuffer: u32 = unsafe { from_x86(x86) };
        let nNumberOfBytesToWrite: u32 = unsafe { from_x86(x86) };
        let lpNumberOfBytesWritten: Option<&mut u32> = unsafe { from_x86(x86) };
        let lpOverlapped: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::WriteFile(
            x86,
            hFile,
            lpBuffer,
            nNumberOfBytesToWrite,
            lpNumberOfBytesWritten,
            lpOverlapped,
        )
        .to_raw();
    }
    fn VirtualAlloc(x86: &mut X86) {
        let lpAddress: u32 = unsafe { from_x86(x86) };
        let dwSize: u32 = unsafe { from_x86(x86) };
        let _flAllocationType: u32 = unsafe { from_x86(x86) };
        let _flProtec: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::VirtualAlloc(x86, lpAddress, dwSize, _flAllocationType, _flProtec)
                .to_raw();
    }
    fn VirtualFree(x86: &mut X86) {
        let lpAddress: u32 = unsafe { from_x86(x86) };
        let dwSize: u32 = unsafe { from_x86(x86) };
        let dwFreeType: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::VirtualFree(x86, lpAddress, dwSize, dwFreeType).to_raw();
    }
    fn OutputDebugStringA(x86: &mut X86) {
        let msg: Option<&str> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::OutputDebugStringA(x86, msg).to_raw();
    }
    fn InitializeCriticalSectionAndSpinCount(x86: &mut X86) {
        let _lpCriticalSection: u32 = unsafe { from_x86(x86) };
        let _dwSpinCount: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            x86,
            _lpCriticalSection,
            _dwSpinCount,
        )
        .to_raw();
    }
    fn DeleteCriticalSection(x86: &mut X86) {
        let _lpCriticalSection: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::DeleteCriticalSection(x86, _lpCriticalSection).to_raw();
    }
    fn EnterCriticalSection(x86: &mut X86) {
        let _lpCriticalSection: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::EnterCriticalSection(x86, _lpCriticalSection).to_raw();
    }
    fn LeaveCriticalSection(x86: &mut X86) {
        let _lpCriticalSection: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::LeaveCriticalSection(x86, _lpCriticalSection).to_raw();
    }
    fn SetUnhandledExceptionFilter(x86: &mut X86) {
        let _lpTopLevelExceptionFilter: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::SetUnhandledExceptionFilter(x86, _lpTopLevelExceptionFilter).to_raw();
    }
    fn UnhandledExceptionFilter(x86: &mut X86) {
        let _exceptionInfo: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::UnhandledExceptionFilter(x86, _exceptionInfo).to_raw();
    }
    fn NtCurrentTeb(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::NtCurrentTeb(x86).to_raw();
    }
    fn TlsAlloc(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::TlsAlloc(x86).to_raw();
    }
    fn TlsFree(x86: &mut X86) {
        let dwTlsIndex: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::TlsFree(x86, dwTlsIndex).to_raw();
    }
    fn TlsSetValue(x86: &mut X86) {
        let dwTlsIndex: u32 = unsafe { from_x86(x86) };
        let lpTlsValue: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::TlsSetValue(x86, dwTlsIndex, lpTlsValue).to_raw();
    }
    fn TlsGetValue(x86: &mut X86) {
        let dwTlsIndex: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::TlsGetValue(x86, dwTlsIndex).to_raw();
    }
    fn InitializeSListHead(x86: &mut X86) {
        let ListHead: Option<&mut SLIST_HEADER> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::InitializeSListHead(x86, ListHead).to_raw();
    }
    fn MultiByteToWideChar(x86: &mut X86) {
        let CodePage: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let lpMultiByteStr: u32 = unsafe { from_x86(x86) };
        let cbMultiByte: i32 = unsafe { from_x86(x86) };
        let lpWideCharStr: Option<&mut [u16]> = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::MultiByteToWideChar(
            x86,
            CodePage,
            dwFlags,
            lpMultiByteStr,
            cbMultiByte,
            lpWideCharStr,
        )
        .to_raw();
    }
    pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
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
            _ => return None,
        })
    }
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    fn RegisterClassA(x86: &mut X86) {
        let lpWndClass: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::RegisterClassA(x86, lpWndClass).to_raw();
    }
    fn CreateWindowExA(x86: &mut X86) {
        let dwExStyle: u32 = unsafe { from_x86(x86) };
        let className: Option<&str> = unsafe { from_x86(x86) };
        let windowName: Option<&str> = unsafe { from_x86(x86) };
        let dwStyle: u32 = unsafe { from_x86(x86) };
        let X: u32 = unsafe { from_x86(x86) };
        let Y: u32 = unsafe { from_x86(x86) };
        let nWidth: u32 = unsafe { from_x86(x86) };
        let nHeight: u32 = unsafe { from_x86(x86) };
        let hWndParent: u32 = unsafe { from_x86(x86) };
        let hMenu: u32 = unsafe { from_x86(x86) };
        let hInstance: u32 = unsafe { from_x86(x86) };
        let lpParam: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::CreateWindowExA(
            x86, dwExStyle, className, windowName, dwStyle, X, Y, nWidth, nHeight, hWndParent,
            hMenu, hInstance, lpParam,
        )
        .to_raw();
    }
    fn UpdateWindow(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::UpdateWindow(x86, _hWnd).to_raw();
    }
    fn ShowWindow(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        let _nCmdShow: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::ShowWindow(x86, _hWnd, _nCmdShow).to_raw();
    }
    fn SetFocus(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::SetFocus(x86, _hWnd).to_raw();
    }
    fn MessageBoxA(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        let lpText: u32 = unsafe { from_x86(x86) };
        let lpCaption: u32 = unsafe { from_x86(x86) };
        let _uType: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::MessageBoxA(x86, _hWnd, lpText, lpCaption, _uType).to_raw();
    }
    fn DialogBoxParamA(x86: &mut X86) {
        let hInstance: u32 = unsafe { from_x86(x86) };
        let lpTemplateName: u32 = unsafe { from_x86(x86) };
        let hWndParent: u32 = unsafe { from_x86(x86) };
        let lpDialogFunc: u32 = unsafe { from_x86(x86) };
        let dwInitParam: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::DialogBoxParamA(
            x86,
            hInstance,
            lpTemplateName,
            hWndParent,
            lpDialogFunc,
            dwInitParam,
        )
        .to_raw();
    }
    fn PeekMessageA(x86: &mut X86) {
        let _lpMsg: u32 = unsafe { from_x86(x86) };
        let _hWnd: u32 = unsafe { from_x86(x86) };
        let _wMsgFilterMin: u32 = unsafe { from_x86(x86) };
        let _wMsgFilterMax: u32 = unsafe { from_x86(x86) };
        let _wRemoveMs: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::PeekMessageA(
            x86,
            _lpMsg,
            _hWnd,
            _wMsgFilterMin,
            _wMsgFilterMax,
            _wRemoveMs,
        )
        .to_raw();
    }
    fn LoadIconA(x86: &mut X86) {
        let _hInstance: u32 = unsafe { from_x86(x86) };
        let _lpIconName: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::LoadIconA(x86, _hInstance, _lpIconName).to_raw();
    }
    fn LoadCursorA(x86: &mut X86) {
        let _hInstance: u32 = unsafe { from_x86(x86) };
        let _lpCursorName: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::LoadCursorA(x86, _hInstance, _lpCursorName).to_raw();
    }
    fn ShowCursor(x86: &mut X86) {
        let _bShow: bool = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::ShowCursor(x86, _bShow).to_raw();
    }
    fn LoadImageA(x86: &mut X86) {
        let hInstance: u32 = unsafe { from_x86(x86) };
        let name: u32 = unsafe { from_x86(x86) };
        let typ: u32 = unsafe { from_x86(x86) };
        let _cx: u32 = unsafe { from_x86(x86) };
        let _cy: u32 = unsafe { from_x86(x86) };
        let fuLoad: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::user32::LoadImageA(x86, hInstance, name, typ, _cx, _cy, fuLoad).to_raw();
    }
    fn GetSystemMetrics(x86: &mut X86) {
        let nIndex: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::GetSystemMetrics(x86, nIndex).to_raw();
    }
    pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
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
