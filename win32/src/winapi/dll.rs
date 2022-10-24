#![allow(non_snake_case)]
#![allow(unused_imports)]
/// Generated code, do not edit.
use crate::{memory::Memory, winapi, x86::X86};
pub unsafe fn smuggle<T: ?Sized>(x: &T) -> &'static T {
    std::mem::transmute(x)
}
pub unsafe fn smuggle_mut<T: ?Sized>(x: &mut T) -> &'static mut T {
    std::mem::transmute(x)
}
pub unsafe trait FromX86 {
    fn from_x86(x86: &mut X86) -> Self;
}
unsafe impl FromX86 for u32 {
    fn from_x86(x86: &mut X86) -> Self {
        x86.pop()
    }
}
unsafe impl<T: From<u32>> FromX86 for Option<T> {
    fn from_x86(x86: &mut X86) -> Self {
        let val = x86.pop();
        if val == 0 {
            None
        } else {
            Some(T::from(val))
        }
    }
}
unsafe impl FromX86 for &mut [u8] {
    fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86.pop() as usize;
        let len = x86.pop() as usize;
        unsafe { smuggle_mut(&mut x86.mem[ofs..ofs + len]) }
    }
}
unsafe impl FromX86 for &str {
    fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86.pop() as usize;
        let strz = x86.mem[ofs..].read_strz();
        unsafe { smuggle(strz) }
    }
}
#[allow(dead_code)]
pub fn from_x86<T: FromX86>(x86: &mut X86) -> T {
    T::from_x86(x86)
}
pub mod kernel32 {
    use super::*;
    use winapi::kernel32::*;
    fn ExitProcess(x86: &mut X86) {
        let uExitCode: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::ExitProcess(x86, uExitCode) as u32;
    }
    fn GetACP(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetACP(x86) as u32;
    }
    fn GetCommandLineA(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetCommandLineA(x86) as u32;
    }
    fn GetCPInfo(x86: &mut X86) {
        let _CodePage: u32 = from_x86(x86);
        let _lpCPInfo: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetCPInfo(x86, _CodePage, _lpCPInfo) as u32;
    }
    fn GetEnvironmentStrings(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetEnvironmentStrings(x86) as u32;
    }
    fn FreeEnvironmentStringsA(x86: &mut X86) {
        let _penv: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::FreeEnvironmentStringsA(x86, _penv) as u32;
    }
    fn GetEnvironmentStringsW(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetEnvironmentStringsW(x86) as u32;
    }
    fn GetEnvironmentVariableA(x86: &mut X86) {
        let name: &str = from_x86(x86);
        let buf: &mut [u8] = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetEnvironmentVariableA(x86, name, buf) as u32;
    }
    fn GetFileType(x86: &mut X86) {
        let hFile: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetFileType(x86, hFile) as u32;
    }
    fn GetModuleFileNameA(x86: &mut X86) {
        let hModule: Option<HMODULE> = from_x86(x86);
        let filename: &mut [u8] = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetModuleFileNameA(x86, hModule, filename) as u32;
    }
    fn GetModuleHandleA(x86: &mut X86) {
        let lpModuleName: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetModuleHandleA(x86, lpModuleName) as u32;
    }
    fn GetStartupInfoA(x86: &mut X86) {
        let lpStartupInfo: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetStartupInfoA(x86, lpStartupInfo) as u32;
    }
    fn GetStdHandle(x86: &mut X86) {
        let nStdHandle: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetStdHandle(x86, nStdHandle) as u32;
    }
    fn GetTickCount(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetTickCount(x86) as u32;
    }
    fn GetVersion(x86: &mut X86) {
        x86.regs.eax = winapi::kernel32::GetVersion(x86) as u32;
    }
    fn GetVersionExA(x86: &mut X86) {
        let lpVersionInformation: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::GetVersionExA(x86, lpVersionInformation) as u32;
    }
    fn HeapAlloc(x86: &mut X86) {
        let hHeap: u32 = from_x86(x86);
        let dwFlags: u32 = from_x86(x86);
        let dwBytes: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::HeapAlloc(x86, hHeap, dwFlags, dwBytes) as u32;
    }
    fn HeapFree(x86: &mut X86) {
        let _hHeap: u32 = from_x86(x86);
        let dwFlags: u32 = from_x86(x86);
        let _lpMem: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::HeapFree(x86, _hHeap, dwFlags, _lpMem) as u32;
    }
    fn HeapSize(x86: &mut X86) {
        let hHeap: u32 = from_x86(x86);
        let dwFlags: u32 = from_x86(x86);
        let lpMem: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::HeapSize(x86, hHeap, dwFlags, lpMem) as u32;
    }
    fn HeapReAlloc(x86: &mut X86) {
        let hHeap: u32 = from_x86(x86);
        let dwFlags: u32 = from_x86(x86);
        let lpMem: u32 = from_x86(x86);
        let dwBytes: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::HeapReAlloc(x86, hHeap, dwFlags, lpMem, dwBytes) as u32;
    }
    fn HeapCreate(x86: &mut X86) {
        let flOptions: u32 = from_x86(x86);
        let dwInitialSize: u32 = from_x86(x86);
        let dwMaximumSize: u32 = from_x86(x86);
        x86.regs.eax =
            winapi::kernel32::HeapCreate(x86, flOptions, dwInitialSize, dwMaximumSize) as u32;
    }
    fn HeapDestroy(x86: &mut X86) {
        let hHeap: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::HeapDestroy(x86, hHeap) as u32;
    }
    fn LoadLibraryA(x86: &mut X86) {
        let lpLibFileName: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::LoadLibraryA(x86, lpLibFileName) as u32;
    }
    fn SetHandleCount(x86: &mut X86) {
        let uNumber: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::SetHandleCount(x86, uNumber) as u32;
    }
    fn WriteFile(x86: &mut X86) {
        let hFile: u32 = from_x86(x86);
        let lpBuffer: u32 = from_x86(x86);
        let nNumberOfBytesToWrite: u32 = from_x86(x86);
        let lpNumberOfBytesWritten: u32 = from_x86(x86);
        let lpOverlapped: u32 = from_x86(x86);
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
        let lpAddress: u32 = from_x86(x86);
        let dwSize: u32 = from_x86(x86);
        let _flAllocationType: u32 = from_x86(x86);
        let _flProtec: u32 = from_x86(x86);
        x86.regs.eax =
            winapi::kernel32::VirtualAlloc(x86, lpAddress, dwSize, _flAllocationType, _flProtec)
                as u32;
    }
    fn VirtualFree(x86: &mut X86) {
        let lpAddress: u32 = from_x86(x86);
        let dwSize: u32 = from_x86(x86);
        let dwFreeType: u32 = from_x86(x86);
        x86.regs.eax = winapi::kernel32::VirtualFree(x86, lpAddress, dwSize, dwFreeType) as u32;
    }
    pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
        Some(match name {
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
            "GetStdHandle" => GetStdHandle,
            "GetTickCount" => GetTickCount,
            "GetVersion" => GetVersion,
            "GetVersionExA" => GetVersionExA,
            "HeapAlloc" => HeapAlloc,
            "HeapFree" => HeapFree,
            "HeapSize" => HeapSize,
            "HeapReAlloc" => HeapReAlloc,
            "HeapCreate" => HeapCreate,
            "HeapDestroy" => HeapDestroy,
            "LoadLibraryA" => LoadLibraryA,
            "SetHandleCount" => SetHandleCount,
            "WriteFile" => WriteFile,
            "VirtualAlloc" => VirtualAlloc,
            "VirtualFree" => VirtualFree,
            _ => return None,
        })
    }
}
