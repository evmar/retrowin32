#![allow(non_snake_case)]
#![allow(unused_imports)]
#[doc = r" Generated code, do not edit."]
use crate::{memory::Memory, winapi, winapi::shims::from_x86, x86::X86};
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    fn DirectDrawCreate(x86: &mut X86) {
        let lpGuid: u32 = unsafe { from_x86(x86) };
        let lplpDD: u32 = unsafe { from_x86(x86) };
        let pUnkOuter: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::ddraw::DirectDrawCreate(x86, lpGuid, lplpDD, pUnkOuter) as u32;
    }
    fn DirectDrawCreateEx(x86: &mut X86) {
        let lpGuid: u32 = unsafe { from_x86(x86) };
        let lplpDD: u32 = unsafe { from_x86(x86) };
        let iid: u32 = unsafe { from_x86(x86) };
        let pUnkOuter: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::ddraw::DirectDrawCreateEx(x86, lpGuid, lplpDD, iid, pUnkOuter) as u32;
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
        x86.regs.eax = winapi::gdi32::GetStockObject(x86, _i) as u32;
    }
    fn SelectObject(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        let hGdiObj: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::SelectObject(x86, hdc, hGdiObj) as u32;
    }
    fn GetObjectA(x86: &mut X86) {
        let handle: u32 = unsafe { from_x86(x86) };
        let _bytes: u32 = unsafe { from_x86(x86) };
        let _out: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::GetObjectA(x86, handle, _bytes, _out) as u32;
    }
    fn CreateCompatibleDC(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::CreateCompatibleDC(x86, hdc) as u32;
    }
    fn DeleteDC(x86: &mut X86) {
        let hdc: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::gdi32::DeleteDC(x86, hdc) as u32;
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
        x86.regs.eax = winapi::gdi32::BitBlt(x86, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop) as u32;
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
        ) as u32;
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
    fn GetLastError(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetLastError(x86) as u32;
    }
    fn ExitProcess(x86: &mut X86) {
        let uExitCode: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::ExitProcess(x86, uExitCode) as u32;
    }
    fn GetACP(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetACP(x86) as u32;
    }
    fn GetCommandLineA(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCommandLineA(x86) as u32;
    }
    fn GetCPInfo(x86: &mut X86) {
        let _CodePage: u32 = unsafe { from_x86(x86) };
        let _lpCPInfo: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetCPInfo(x86, _CodePage, _lpCPInfo) as u32;
    }
    fn GetEnvironmentStrings(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetEnvironmentStrings(x86) as u32;
    }
    fn FreeEnvironmentStringsA(x86: &mut X86) {
        let _penv: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::FreeEnvironmentStringsA(x86, _penv) as u32;
    }
    fn GetEnvironmentStringsW(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetEnvironmentStringsW(x86) as u32;
    }
    fn GetEnvironmentVariableA(x86: &mut X86) {
        let name: &str = unsafe { from_x86(x86) };
        let buf: &mut [u8] = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetEnvironmentVariableA(x86, name, buf) as u32;
    }
    fn GetFileType(x86: &mut X86) {
        let hFile: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetFileType(x86, hFile) as u32;
    }
    fn GetModuleFileNameA(x86: &mut X86) {
        let hModule: Option<HMODULE> = unsafe { from_x86(x86) };
        let filename: &mut [u8] = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetModuleFileNameA(x86, hModule, filename) as u32;
    }
    fn GetModuleHandleA(x86: &mut X86) {
        let lpModuleName: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetModuleHandleA(x86, lpModuleName) as u32;
    }
    fn GetStartupInfoA(x86: &mut X86) {
        let lpStartupInfo: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetStartupInfoA(x86, lpStartupInfo) as u32;
    }
    fn IsProcessorFeaturePresent(x86: &mut X86) {
        let _feature: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::IsProcessorFeaturePresent(x86, _feature) as u32;
    }
    fn IsDebuggerPresent(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::IsDebuggerPresent(x86) as u32;
    }
    fn GetCurrentThreadId(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCurrentThreadId(x86) as u32;
    }
    fn GetCurrentProcessId(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCurrentProcessId(x86) as u32;
    }
    fn GetStdHandle(x86: &mut X86) {
        let nStdHandle: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetStdHandle(x86, nStdHandle) as u32;
    }
    fn GetTickCount(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetTickCount(x86) as u32;
    }
    fn QueryPerformanceCounter(x86: &mut X86) {
        let _ptr: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::QueryPerformanceCounter(x86, _ptr) as u32;
    }
    fn GetSystemTimeAsFileTime(x86: &mut X86) {
        let _time: &mut FILETIME = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetSystemTimeAsFileTime(x86, _time) as u32;
    }
    fn GetVersion(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetVersion(x86) as u32;
    }
    fn GetVersionExA(x86: &mut X86) {
        let lpVersionInformation: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::GetVersionExA(x86, lpVersionInformation) as u32;
    }
    fn HeapAlloc(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let dwBytes: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapAlloc(x86, hHeap, dwFlags, dwBytes) as u32;
    }
    fn HeapFree(x86: &mut X86) {
        let _hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let _lpMem: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapFree(x86, _hHeap, dwFlags, _lpMem) as u32;
    }
    fn HeapSize(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let lpMem: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapSize(x86, hHeap, dwFlags, lpMem) as u32;
    }
    fn HeapReAlloc(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        let lpMem: u32 = unsafe { from_x86(x86) };
        let dwBytes: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapReAlloc(x86, hHeap, dwFlags, lpMem, dwBytes) as u32;
    }
    fn HeapCreate(x86: &mut X86) {
        let flOptions: u32 = unsafe { from_x86(x86) };
        let dwInitialSize: u32 = unsafe { from_x86(x86) };
        let dwMaximumSize: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::HeapCreate(x86, flOptions, dwInitialSize, dwMaximumSize) as u32;
    }
    fn HeapDestroy(x86: &mut X86) {
        let hHeap: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::HeapDestroy(x86, hHeap) as u32;
    }
    fn LoadLibraryA(x86: &mut X86) {
        let lpLibFileName: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::LoadLibraryA(x86, lpLibFileName) as u32;
    }
    fn LoadLibraryExW(x86: &mut X86) {
        let lpLibFileName: u32 = unsafe { from_x86(x86) };
        let hFile: u32 = unsafe { from_x86(x86) };
        let dwFlags: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::LoadLibraryExW(x86, lpLibFileName, hFile, dwFlags) as u32;
    }
    fn SetHandleCount(x86: &mut X86) {
        let uNumber: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::SetHandleCount(x86, uNumber) as u32;
    }
    fn WriteFile(x86: &mut X86) {
        let hFile: u32 = unsafe { from_x86(x86) };
        let lpBuffer: u32 = unsafe { from_x86(x86) };
        let nNumberOfBytesToWrite: u32 = unsafe { from_x86(x86) };
        let lpNumberOfBytesWritten: u32 = unsafe { from_x86(x86) };
        let lpOverlapped: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::WriteFile(
            x86,
            hFile,
            lpBuffer,
            nNumberOfBytesToWrite,
            lpNumberOfBytesWritten,
            lpOverlapped,
        ) as u32;
    }
    fn VirtualAlloc(x86: &mut X86) {
        let lpAddress: u32 = unsafe { from_x86(x86) };
        let dwSize: u32 = unsafe { from_x86(x86) };
        let _flAllocationType: u32 = unsafe { from_x86(x86) };
        let _flProtec: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::kernel32::VirtualAlloc(x86, lpAddress, dwSize, _flAllocationType, _flProtec)
                as u32;
    }
    fn VirtualFree(x86: &mut X86) {
        let lpAddress: u32 = unsafe { from_x86(x86) };
        let dwSize: u32 = unsafe { from_x86(x86) };
        let dwFreeType: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::VirtualFree(x86, lpAddress, dwSize, dwFreeType) as u32;
    }
    fn OutputDebugStringA(x86: &mut X86) {
        let msg: &str = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::OutputDebugStringA(x86, msg) as u32;
    }
    fn InitializeCriticalSectionAndSpinCount(x86: &mut X86) {
        let _lpCriticalSection: u32 = unsafe { from_x86(x86) };
        let _dwSpinCount: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::kernel32::InitializeCriticalSectionAndSpinCount(
            x86,
            _lpCriticalSection,
            _dwSpinCount,
        ) as u32;
    }
    pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
        Some(match name {
            "GetLastError" => GetLastError,
            "ExitProcess" => ExitProcess,
            "GetACP" => GetACP,
            "GetCommandLineA" => GetCommandLineA,
            "GetCPInfo" => GetCPInfo,
            "GetEnvironmentStrings" => GetEnvironmentStrings,
            "FreeEnvironmentStringsA" => FreeEnvironmentStringsA,
            "GetEnvironmentStringsW" => GetEnvironmentStringsW,
            "GetEnvironmentVariableA" => GetEnvironmentVariableA,
            "GetFileType" => GetFileType,
            "GetModuleFileNameA" => GetModuleFileNameA,
            "GetModuleHandleA" => GetModuleHandleA,
            "GetStartupInfoA" => GetStartupInfoA,
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
            "LoadLibraryA" => LoadLibraryA,
            "LoadLibraryExW" => LoadLibraryExW,
            "SetHandleCount" => SetHandleCount,
            "WriteFile" => WriteFile,
            "VirtualAlloc" => VirtualAlloc,
            "VirtualFree" => VirtualFree,
            "OutputDebugStringA" => OutputDebugStringA,
            "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount,
            _ => return None,
        })
    }
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    fn RegisterClassA(x86: &mut X86) {
        let lpWndClass: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::RegisterClassA(x86, lpWndClass) as u32;
    }
    fn CreateWindowExA(x86: &mut X86) {
        let dwExStyle: u32 = unsafe { from_x86(x86) };
        let className: &str = unsafe { from_x86(x86) };
        let windowName: &str = unsafe { from_x86(x86) };
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
        ) as u32;
    }
    fn UpdateWindow(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::UpdateWindow(x86, _hWnd) as u32;
    }
    fn ShowWindow(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        let _nCmdShow: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::ShowWindow(x86, _hWnd, _nCmdShow) as u32;
    }
    fn SetFocus(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::SetFocus(x86, _hWnd) as u32;
    }
    fn MessageBoxA(x86: &mut X86) {
        let _hWnd: u32 = unsafe { from_x86(x86) };
        let lpText: u32 = unsafe { from_x86(x86) };
        let lpCaption: u32 = unsafe { from_x86(x86) };
        let _uType: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::MessageBoxA(x86, _hWnd, lpText, lpCaption, _uType) as u32;
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
        ) as u32;
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
        ) as u32;
    }
    fn LoadIconA(x86: &mut X86) {
        let _hInstance: u32 = unsafe { from_x86(x86) };
        let _lpIconName: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::LoadIconA(x86, _hInstance, _lpIconName) as u32;
    }
    fn LoadCursorA(x86: &mut X86) {
        let _hInstance: u32 = unsafe { from_x86(x86) };
        let _lpCursorName: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::LoadCursorA(x86, _hInstance, _lpCursorName) as u32;
    }
    fn ShowCursor(x86: &mut X86) {
        let _bShow: bool = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::ShowCursor(x86, _bShow) as u32;
    }
    fn LoadImageA(x86: &mut X86) {
        let hInstance: u32 = unsafe { from_x86(x86) };
        let name: u32 = unsafe { from_x86(x86) };
        let typ: u32 = unsafe { from_x86(x86) };
        let _cx: u32 = unsafe { from_x86(x86) };
        let _cy: u32 = unsafe { from_x86(x86) };
        let fuLoad: u32 = unsafe { from_x86(x86) };
        x86.regs.eax =
            winapi::user32::LoadImageA(x86, hInstance, name, typ, _cx, _cy, fuLoad) as u32;
    }
    fn GetSystemMetrics(x86: &mut X86) {
        let nIndex: u32 = unsafe { from_x86(x86) };
        x86.regs.eax = winapi::user32::GetSystemMetrics(x86, nIndex) as u32;
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
