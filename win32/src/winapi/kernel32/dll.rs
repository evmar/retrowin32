use memory::{Extensions, Pod};

use crate::{
    host,
    machine::{Emulator, Machine},
    pe,
    str16::expect_ascii,
    winapi::{self, builtin::BuiltinDLL, stack_args::ArrayWithSizeMut, types::*, ImportSymbol},
};
use std::io::Write;

const TRACE_CONTEXT: &'static str = "kernel32/dll";

// HMODULE is index+1 into kernel32::State::dlls.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HMODULET;
pub type HMODULE = HANDLE<HMODULET>;

impl HMODULE {
    pub fn from_dll_index(index: usize) -> Self {
        return HMODULE::from_raw((index + 1) as u32);
    }

    pub fn to_dll_index(&self) -> Option<usize> {
        if self.is_null() {
            return None;
        }
        Some(self.raw as usize - 1)
    }
}

pub struct DLL {
    pub name: String,

    pub dll: pe::DLL,

    /// If present, DLL is one defined in winapi/...
    pub builtin: Option<&'static BuiltinDLL>,
}

impl DLL {
    fn resolve_from_pe(&self, sym: &ImportSymbol) -> Option<u32> {
        match *sym {
            ImportSymbol::Name(name) => self.dll.names.get(name).copied(),
            ImportSymbol::Ordinal(ord) => self.dll.ordinals.get(&ord).copied(),
        }
    }

    pub fn resolve_from_builtin(
        &mut self,
        sym: &ImportSymbol,
        register: impl FnOnce(Result<&'static crate::shims::Shim, String>) -> u32,
    ) -> Option<u32> {
        let builtin = self.builtin?;

        let export = match *sym {
            ImportSymbol::Name(name) => builtin
                .exports
                .iter()
                .find(|&export| export.shim.name == name),
            ImportSymbol::Ordinal(ord) => builtin
                .exports
                .iter()
                .find(|&export| export.ordinal == Some(ord as usize)),
        };

        let addr = match export {
            Some(export) => register(Ok(&export.shim)),
            None => {
                let name = format!("{}:{}", self.name, sym);
                log::warn!("unimplemented: {}", name);
                register(Err(name))
            }
        };

        match *sym {
            ImportSymbol::Name(name) => {
                self.dll.names.insert(name.to_string(), addr);
            }
            ImportSymbol::Ordinal(ord) => {
                self.dll.ordinals.insert(ord, addr);
            }
        }
        return Some(addr);
    }

    pub fn resolve(
        &mut self,
        sym: &ImportSymbol,
        register: impl FnOnce(Result<&'static crate::shims::Shim, String>) -> u32,
    ) -> u32 {
        if let Some(addr) = self.resolve_from_pe(&sym) {
            return addr;
        }
        if let Some(addr) = self.resolve_from_builtin(&sym, register) {
            return addr;
        }
        log::warn!("failed to resolve {}:{}", self.name, sym);
        0
    }
}

fn normalize_module_name(name: &str) -> String {
    let mut name = name.to_ascii_lowercase();
    if !name.ends_with(".dll") && !name.ends_with(".") {
        name.push_str(".dll");
    }
    name
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(machine: &mut Machine, lpModuleName: Option<&str>) -> HMODULE {
    let name = match lpModuleName {
        None => return HMODULE::from_raw(machine.state.kernel32.image_base),
        Some(name) => name,
    };

    let name = normalize_module_name(name);

    if let Some(index) = machine
        .state
        .kernel32
        .dlls
        .iter()
        .position(|dll| dll.name == name)
    {
        return HMODULE::from_dll_index(index);
    }

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

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(
    _machine: &mut Machine,
    hModule: HMODULE,
    filename: ArrayWithSizeMut<u8>,
) -> u32 {
    assert!(hModule.is_null());
    match filename.unwrap().write(b"TODO.exe\0") {
        Ok(n) => n as u32,
        Err(err) => {
            log::warn!("GetModuleFileNameA(): {}", err);
            0
        }
    }
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameW(
    _machine: &mut Machine,
    hModule: HMODULE,
    _lpFilename: u32,
    _nSize: u32,
) -> u32 {
    if !hModule.is_null() {
        log::error!("unimplemented: GetModuleFileNameW(non-null)")
    }
    0 // fail
}

#[win32_derive::dllexport]
pub fn LoadLibraryA(machine: &mut Machine, filename: Option<&str>) -> HMODULE {
    let mut filename = normalize_module_name(filename.unwrap());

    // See if already loaded.
    if let Some(index) = machine
        .state
        .kernel32
        .dlls
        .iter()
        .position(|dll| dll.name == filename)
    {
        return HMODULE::from_dll_index(index);
    }

    if filename.starts_with("api-") {
        match winapi::apiset(&filename) {
            Some(name) => filename = name.to_string(),
            None => return HMODULE::null(),
        }
    }

    // Check if builtin.
    if let Some(builtin) = winapi::DLLS.iter().find(|&dll| dll.file_name == filename) {
        return machine.state.kernel32.load_builtin_dll(builtin);
    }

    let mut file = machine.host.open(&filename, host::FileAccess::READ);
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    // TODO: close file.
    if contents.len() == 0 {
        // HACK: zero-length indicates not found.
        return HMODULE::null();
    }

    let dll = pe::load_dll(machine, &filename, &contents).unwrap();
    machine.state.kernel32.dlls.push(DLL {
        name: filename,
        dll,
        builtin: None,
    });
    HMODULE::from_dll_index(machine.state.kernel32.dlls.len() - 1)
}

#[win32_derive::dllexport]
pub fn LoadLibraryExW(
    machine: &mut Machine,
    lpLibFileName: Option<&Str16>,
    hFile: HFILE,
    dwFlags: u32,
) -> HMODULE {
    let filename = lpLibFileName.map(|f| f.to_string());
    LoadLibraryA(machine, filename.as_deref())
}

#[win32_derive::dllexport]
pub fn FreeLibrary(_machine: &mut Machine, hLibModule: HMODULE) -> bool {
    true // success
}

/// The argument to GetProcAddress is an ImportSymbol stuffed into a u32.
#[derive(Debug)]
pub struct GetProcAddressArg<'a>(pub ImportSymbol<'a>);

impl<'a> winapi::stack_args::FromStack<'a> for GetProcAddressArg<'a> {
    unsafe fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let lpProcName = <u32>::from_stack(mem, sp);
        if lpProcName & 0xFFFF_0000 == 0 {
            GetProcAddressArg(ImportSymbol::Ordinal(lpProcName))
        } else {
            let proc_name = expect_ascii(mem.slicez(lpProcName));
            GetProcAddressArg(ImportSymbol::Name(proc_name))
        }
    }
}

pub fn get_kernel32_builtin(machine: &mut Machine, name: &str) -> u32 {
    let dll = &mut machine.state.kernel32.dlls[0];
    dll.resolve(&ImportSymbol::Name(name), |shim| {
        let addr = machine.emu.register(shim);
        machine.labels.insert(addr, format!("{}", name));
        addr
    })
}

#[win32_derive::dllexport]
pub fn GetProcAddress(
    machine: &mut Machine,
    hModule: HMODULE,
    lpProcName: GetProcAddressArg,
) -> u32 {
    let index = hModule.to_dll_index().unwrap();
    if let Some(dll) = machine.state.kernel32.dlls.get_mut(index) {
        return dll.resolve(&lpProcName.0, |shim| {
            let addr = machine.emu.register(shim);
            machine.labels.insert(addr, format!("{}", lpProcName.0));
            addr
        });
    }
    log::error!("GetProcAddress({:x?}, {:?})", hModule, lpProcName);
    0 // fail
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
    let len = std::cmp::min(info.cb, std::mem::size_of::<STARTUPINFOA>() as u32);
    unsafe { info.clear_memory(len) };
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
