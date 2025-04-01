use crate::{
    calling_convention::{self, ArrayOut},
    loader,
    machine::Machine,
    winapi::{
        self,
        kernel32::set_last_error,
        string::{BufWrite, BufWriteAnsi, BufWriteWide},
        *,
    },
};
use memory::{Extensions, Pod};
use pe::ImportSymbol;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HMODULET;
/// HMODULE is the address of the loaded PE image.
// (BASS.dll calls LoadLibrary and reads the PE header found at the returned address.)
pub type HMODULE = HANDLE<HMODULET>;

#[win32_derive::dllexport]
pub fn GetModuleHandleA(machine: &mut Machine, lpModuleName: Option<&str>) -> HMODULE {
    let name = match lpModuleName {
        None => return HMODULE::from_raw(machine.state.kernel32.image_base),
        Some(name) => name,
    };

    let name = loader::normalize_module_name(name);

    if let Some((hmodule, _)) = machine
        .state
        .kernel32
        .modules
        .iter()
        .find(|(_, dll)| dll.name == name)
    {
        return *hmodule;
    }

    set_last_error(machine, winapi::ERROR::MOD_NOT_FOUND);
    return HMODULE::null();
}

#[win32_derive::dllexport]
pub fn GetModuleHandleW(machine: &mut Machine, lpModuleName: Option<&Str16>) -> HMODULE {
    let ascii = lpModuleName.map(|str| str.to_string());
    GetModuleHandleA(machine, ascii.as_deref())
}

#[win32_derive::dllexport]
pub fn GetModuleHandleExW(
    machine: &mut Machine,
    dwFlags: u32,
    lpModuleName: Option<&Str16>,
    hModule: Option<&mut HMODULE>,
) -> bool {
    if dwFlags != 0 {
        unimplemented!("GetModuleHandleExW flags {dwFlags:x}");
    }
    let hMod = GetModuleHandleW(machine, lpModuleName);
    if let Some(out) = hModule {
        *out = hMod;
    }
    return !hMod.is_null();
}

fn get_module_file_name(
    machine: &mut Machine,
    hModule: HMODULE,
    filename: &mut dyn BufWrite,
) -> u32 {
    if hModule.is_null() || hModule.to_raw() == machine.state.kernel32.image_base {
        let exe = machine.state.kernel32.cmdline.exe_name();
        let n = filename.write(&exe);
        match n {
            Ok(n) => n - 1,
            Err(_) => {
                set_last_error(machine, ERROR::INSUFFICIENT_BUFFER);
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
pub fn GetModuleFileNameA(
    machine: &mut Machine,
    hModule: HMODULE,
    mut filename: ArrayOut<u8>,
) -> u32 {
    get_module_file_name(machine, hModule, &mut BufWriteAnsi::new(&mut filename))
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameW(
    machine: &mut Machine,
    hModule: HMODULE,
    lpFilename: u32,
    nSize: u32,
) -> u32 {
    // TODO: figure out what we should do with ArrayWithSize for u16.
    let mut filename = BufWriteWide::from_mem(unsafe { machine.mem().detach() }, lpFilename, nSize);
    get_module_file_name(machine, hModule, &mut filename)
}

fn load_library(machine: &mut Machine, filename: &str) -> HMODULE {
    let res = loader::resolve_dll(machine, filename);
    match loader::load_dll(machine, &res) {
        Ok(hmodule) => hmodule,
        Err(e) => {
            // TODO: set_last_error fails here if this happens before TEB setup
            log::error!("LoadLibraryA({:?}) failed: {:?}", filename, e);
            // set_last_error(machine, winapi::ERROR::MOD_NOT_FOUND);
            HMODULE::null()
        }
    }
}

#[win32_derive::dllexport]
pub fn LoadLibraryW(machine: &mut Machine, filename: Option<&Str16>) -> HMODULE {
    load_library(machine, &filename.unwrap().to_string())
}

#[win32_derive::dllexport]
pub fn LoadLibraryA(machine: &mut Machine, filename: Option<&str>) -> HMODULE {
    load_library(machine, filename.unwrap())
}

#[win32_derive::dllexport]
pub fn LoadLibraryExW(
    machine: &mut Machine,
    lpLibFileName: Option<&Str16>,
    hFile: HFILE,
    dwFlags: u32,
) -> HMODULE {
    let filename = lpLibFileName.map(|f| f.to_string());
    load_library(machine, &filename.unwrap())
}

#[win32_derive::dllexport]
pub fn FreeLibrary(_machine: &mut Machine, hLibModule: HMODULE) -> bool {
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
    unsafe fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let lpProcName = <u32>::from_stack(mem, sp);
        if lpProcName & 0xFFFF_0000 == 0 {
            GetProcAddressArg(ImportSymbol::Ordinal(lpProcName))
        } else {
            let proc_name = mem.slicez(lpProcName);
            GetProcAddressArg(ImportSymbol::Name(proc_name))
        }
    }
}

pub fn get_symbol(machine: &mut Machine, dll: &str, name: &str) -> u32 {
    let hmodule = load_library(machine, dll);
    let module = machine.state.kernel32.modules.get_mut(&hmodule).unwrap();
    module
        .exports
        .resolve(&ImportSymbol::Name(name.as_bytes()))
        .unwrap()
}

pub fn get_kernel32_builtin(machine: &mut Machine, name: &str) -> u32 {
    get_symbol(machine, "kernel32.dll", name)
}

#[win32_derive::dllexport]
pub fn GetProcAddress(
    machine: &mut Machine,
    hModule: HMODULE,
    lpProcName: GetProcAddressArg,
) -> u32 {
    let module = machine.state.kernel32.modules.get_mut(&hModule).unwrap();
    let Some(addr) = module.exports.resolve(&lpProcName.0) else {
        log::warn!("GetProcAddress({:?}, {:?}) failed", module.name, lpProcName);
        return 0; // fail
    };
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
pub fn GetStartupInfoA(_machine: &mut Machine, lpStartupInfo: Option<&mut STARTUPINFOA>) -> u32 {
    // MSVC runtime library passes in uninitialized memory for lpStartupInfo, so don't trust info.cb.
    let info = lpStartupInfo.unwrap();
    let len = std::mem::size_of::<STARTUPINFOA>() as u32;
    unsafe { info.clear_memory(len) };

    info.cb = len;

    0
}

#[win32_derive::dllexport]
pub fn GetStartupInfoW(machine: &mut Machine, lpStartupInfo: Option<&mut STARTUPINFOA>) -> u32 {
    // STARTUPINFOA is the same shape as the W one, just the strings are different...
    GetStartupInfoA(machine, lpStartupInfo)
}

#[win32_derive::dllexport]
pub fn DisableThreadLibraryCalls(_machine: &mut Machine, hLibModule: HMODULE) -> bool {
    true // succeed
}
