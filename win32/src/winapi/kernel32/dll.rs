use crate::{
    loader::{self, Module},
    winapi::kernel32::set_last_error,
};
use memory::{Extensions, Pod};
use pe::ImportSymbol;

use crate::{
    host,
    machine::Machine,
    winapi::{self, builtin, calling_convention::ArrayWithSizeMut, *},
};
use std::io::Write;
use typed_path::WindowsPath;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HMODULET;
/// HMODULE is the address of the loaded DLL image.
// (BASS.dll calls LoadLibrary and reads the PE header found at the returned address.)
pub type HMODULE = HANDLE<HMODULET>;

pub struct DLL {
    pub name: String,

    pub module: Module,
}

impl DLL {
    pub fn resolve(&mut self, sym: &ImportSymbol) -> Option<u32> {
        match *sym {
            ImportSymbol::Name(name) => self.module.exports.names.get(name).copied(),
            ImportSymbol::Ordinal(ord) => self
                .module
                .exports
                .fns
                .get((ord - self.module.exports.ordinal_base) as usize)
                .copied(),
        }
    }
}

pub fn normalize_module_name(name: &str) -> String {
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

    if let Some((hmodule, _)) = machine
        .state
        .kernel32
        .dlls
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

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(
    machine: &mut Machine,
    hModule: HMODULE,
    filename: ArrayWithSizeMut<u8>,
) -> u32 {
    let mut filename = filename.unwrap();
    if hModule.is_null() || hModule.to_raw() == machine.state.kernel32.image_base {
        let mut exe = machine.state.kernel32.cmdline.exe_name();
        exe.push(0 as char);
        let n = filename.write(exe.as_bytes()).unwrap();
        n as u32 - 1 // exclude nul
    } else {
        todo!();
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

pub fn load_library(machine: &mut Machine, filename: &str) -> HMODULE {
    let mut filename = normalize_module_name(filename);

    // See if already loaded.
    if let Some((hmodule, _)) = machine
        .state
        .kernel32
        .dlls
        .iter()
        .find(|(_, dll)| dll.name == filename)
    {
        return *hmodule;
    }

    if filename.starts_with("api-") {
        match builtin::apiset(&filename) {
            Some(name) => filename = name.to_string(),
            None => return HMODULE::null(),
        }
    }

    let use_external = machine.external_dlls.contains(&filename);
    // Builtin DLLs are special in that we load the DLL from an in-binary buffer,
    // and the symbols in the DLL are mapped to shims.
    let mut builtin = None;
    if !use_external {
        if let Some(alias) = builtin::dll_alias(&filename) {
            filename = alias.to_string();
        }
        builtin = builtin::DLLS.iter().find(|&dll| dll.file_name == filename)
    };

    let mut buf = Vec::new();
    let contents = {
        if let Some(builtin) = builtin {
            builtin.raw
        } else {
            let exe = machine.state.kernel32.cmdline.exe_name();
            let exe_dir = exe.rsplitn(2, '\\').last().unwrap();
            let dll_paths = [format!("{exe_dir}\\{filename}"), filename.to_string()];
            for path in &dll_paths {
                let path = WindowsPath::new(path);
                let mut file = match machine.host.open(path, host::FileOptions::read()) {
                    Ok(file) => file,
                    Err(_) => continue,
                };
                file.read_to_end(&mut buf).unwrap();
                // TODO: close file.
                break;
            }
            &buf
        }
    };

    if contents.is_empty() {
        log::warn!("load_library({filename:?}): not found");
        return HMODULE::null();
    }

    let module = loader::load_dll(machine, &filename, contents).unwrap();

    // For builtins, register all the exports as known symbols.
    // It is critical that the DLL's exports match up to the shims array;
    // this is ensured by both being generated by the same generator.
    if let Some(builtin) = builtin {
        for (&addr, shim) in module.exports.fns.iter().zip(builtin.shims) {
            machine.emu.shims.register(addr, Ok(shim));
        }
    }

    let hmodule = HMODULE::from_raw(module.image_base);
    machine.state.kernel32.dlls.insert(
        hmodule,
        DLL {
            name: filename,
            module,
        },
    );
    hmodule
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
    LoadLibraryA(machine, filename.as_deref())
}

#[win32_derive::dllexport]
pub fn FreeLibrary(_machine: &mut Machine, hLibModule: HMODULE) -> bool {
    true // success
}

/// The argument to GetProcAddress is an ImportSymbol stuffed into a u32.
#[derive(Debug)]
pub struct GetProcAddressArg<'a>(pub ImportSymbol<'a>);

impl<'a> winapi::calling_convention::FromStack<'a> for GetProcAddressArg<'a> {
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
    let dll = machine.state.kernel32.dlls.get_mut(&hmodule).unwrap();
    dll.resolve(&ImportSymbol::Name(name.as_bytes())).unwrap()
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
    let dll = machine.state.kernel32.dlls.get_mut(&hModule).unwrap();
    let Some(addr) = dll.resolve(&lpProcName.0) else {
        log::warn!("GetProcAddress({:?}, {:?}) failed", dll.name, lpProcName);
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
