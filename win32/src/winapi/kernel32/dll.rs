use super::{file::HFILE, get_state};
use crate::Machine;
use memory::{Extensions, Pod};
use pe::ImportSymbol;
use win32_system::System;
use win32_winapi::{
    DWORD, ERROR, HMODULE, Str16,
    calling_convention::{self, ArrayOut},
    encoding::*,
};

#[win32_derive::dllexport]
pub fn GetModuleHandleA(sys: &dyn System, lpModuleName: Option<&str>) -> HMODULE {
    let name = {
        let state = get_state(sys);
        match lpModuleName {
            None => return HMODULE::from_raw(state.image_base),
            Some(name) => name,
        }
    };
    let hmodule = sys.get_library(name);
    if hmodule.is_null() {
        sys.set_last_error(ERROR::MOD_NOT_FOUND);
    }
    return HMODULE::null();
}

#[win32_derive::dllexport]
pub fn GetModuleHandleW(sys: &dyn System, lpModuleName: Option<&Str16>) -> HMODULE {
    let ascii = lpModuleName.map(|str| str.to_string());
    GetModuleHandleA(sys, ascii.as_deref())
}

#[win32_derive::dllexport]
pub fn GetModuleHandleExW(
    sys: &dyn System,
    dwFlags: u32,
    lpModuleName: Option<&Str16>,
    hModule: Option<&mut HMODULE>,
) -> bool {
    if dwFlags != 0 {
        unimplemented!("GetModuleHandleExW flags {dwFlags:x}");
    }
    let hMod = GetModuleHandleW(sys, lpModuleName);
    if let Some(out) = hModule {
        *out = hMod;
    }
    return !hMod.is_null();
}

fn get_module_file_name(sys: &dyn System, hModule: HMODULE, filename: &mut dyn Encoder) -> u32 {
    let state = get_state(sys);
    if hModule.is_null() || hModule.to_raw() == state.image_base {
        let exe = state.cmdline.exe_name();
        filename.write_nul(&exe);
        match filename.status() {
            Ok(n) => n - 1,
            Err(_) => {
                sys.set_last_error(ERROR::INSUFFICIENT_BUFFER);
                // TODO: nul termination behavior.
                // Docs say this returns buffer size, not needed space.
                return filename.capacity() as u32;
            }
        }
    } else {
        todo!();
    }
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(sys: &dyn System, hModule: HMODULE, mut filename: ArrayOut<u8>) -> u32 {
    get_module_file_name(sys, hModule, &mut EncoderAnsi::new(&mut filename))
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameW(sys: &dyn System, hModule: HMODULE, lpFilename: u32, nSize: u32) -> u32 {
    // TODO: figure out what we should do with ArrayWithSize for u16.
    let mut filename = EncoderWide::from_mem(unsafe { sys.mem().detach() }, lpFilename, nSize);
    get_module_file_name(sys, hModule, &mut filename)
}

async fn load_library(sys: &mut dyn System, filename: &str) -> HMODULE {
    let hmodule = sys.load_library(filename).await;
    if hmodule.is_null() {
        // TODO: other error codes?
        sys.set_last_error(ERROR::MOD_NOT_FOUND);
    }
    hmodule
}

#[win32_derive::dllexport]
pub async fn LoadLibraryW(sys: &mut dyn System, filename: Option<&Str16>) -> HMODULE {
    load_library(sys, &filename.unwrap().to_string()).await
}

#[win32_derive::dllexport]
pub async fn LoadLibraryA(sys: &mut dyn System, filename: Option<&str>) -> HMODULE {
    load_library(sys, filename.unwrap()).await
}

#[win32_derive::dllexport]
pub async fn LoadLibraryExW(
    sys: &mut dyn System,
    lpLibFileName: Option<&Str16>,
    hFile: HFILE,
    dwFlags: u32,
) -> HMODULE {
    let filename = lpLibFileName.map(|f| f.to_string());
    load_library(sys, &filename.unwrap()).await
}

#[win32_derive::dllexport]
pub fn FreeLibrary(sys: &dyn System, hLibModule: HMODULE) -> bool {
    true // success
}

/// The argument to GetProcAddress is an ImportSymbol stuffed into a u32.
pub struct GetProcAddressArg<'a>(pub ImportSymbol<'a>);

impl<'a> std::fmt::Debug for GetProcAddressArg<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ImportSymbol::Name(name) => write!(f, "{:?}", std::str::from_utf8(name).unwrap()),
            ImportSymbol::Ordinal(ordinal) => write!(f, "0x{:x}", ordinal),
        }
    }
}

impl<'a> calling_convention::FromStack<'a> for GetProcAddressArg<'a> {
    fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let lpProcName = <u32>::from_stack(mem, sp);
        if lpProcName & 0xFFFF_0000 == 0 {
            GetProcAddressArg(ImportSymbol::Ordinal(lpProcName))
        } else {
            let proc_name = mem.slicez(lpProcName);
            GetProcAddressArg(ImportSymbol::Name(proc_name))
        }
    }
}

#[win32_derive::dllexport]
pub fn GetProcAddress(
    machine: &mut Machine,
    hModule: HMODULE,
    lpProcName: GetProcAddressArg,
) -> u32 {
    let module = machine.process.modules.get_mut(&hModule).unwrap();
    let Some(addr) = module.exports.resolve(&lpProcName.0) else {
        log::warn!("GetProcAddress({:?}, {:?}) failed", module.name, lpProcName);
        return 0; // fail
    };
    module.dynamic_imports.insert(addr);
    addr
}

#[repr(C)]
#[derive(Debug)]
pub struct STARTUPINFOA {
    cb: DWORD,
    lpReserved: DWORD,
    lpDesktop: DWORD,
    lpTitle: DWORD,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: u16,
    cbReserved2: u16,
    lpReserved2: DWORD,
    hStdInput: DWORD,
    hStdOutput: DWORD,
    hStdError: DWORD,
}
unsafe impl ::memory::Pod for STARTUPINFOA {}

#[win32_derive::dllexport]
pub fn GetStartupInfoA(sys: &dyn System, lpStartupInfo: Option<&mut STARTUPINFOA>) -> u32 {
    // MSVC runtime library passes in uninitialized memory for lpStartupInfo, so don't trust info.cb.
    let info = lpStartupInfo.unwrap();
    let len = std::mem::size_of::<STARTUPINFOA>() as u32;
    unsafe { info.clear_memory(len) };

    info.cb = len;

    0
}

#[win32_derive::dllexport]
pub fn GetStartupInfoW(sys: &dyn System, lpStartupInfo: Option<&mut STARTUPINFOA>) -> u32 {
    // STARTUPINFOA is the same shape as the W one, just the strings are different...
    GetStartupInfoA(sys, lpStartupInfo)
}

#[win32_derive::dllexport]
pub fn DisableThreadLibraryCalls(sys: &dyn System, hLibModule: HMODULE) -> bool {
    true // succeed
}
