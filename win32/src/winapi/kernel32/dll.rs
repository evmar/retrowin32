use crate::{
    machine::Machine,
    shims::Shims,
    winapi::{self, types::*, BuiltinDLL, ImportSymbol},
};
use std::collections::HashMap;

const TRACE: bool = false;

// HMODULE is index+1 into kernel32::State::dlls.
declare_handle!(HMODULE);

impl HMODULE {
    fn from_dll_index(index: usize) -> Self {
        return HMODULE::from_raw((index + 1) as u32);
    }

    pub fn to_dll_index(&self) -> usize {
        self.0 as usize - 1
    }
}

pub struct DLL {
    name: String,

    /// Function name => resolved address.
    names: HashMap<String, u32>,

    /// If present, DLL is one defined in winapi/...
    builtin: Option<&'static BuiltinDLL>,
}

impl DLL {
    pub fn resolve(&mut self, shims: &mut Shims, sym: ImportSymbol) -> u32 {
        match sym {
            ImportSymbol::Name(name) => {
                if let Some(&addr) = self.names.get(name) {
                    return addr;
                }
            }
            _ => todo!(),
        }

        if let Some(builtin) = self.builtin {
            let handler = (builtin.resolve)(&sym);
            let name = format!("{}!{}", self.name, sym.to_string());
            let addr = shims.add(name, handler);
            return addr;
        }

        todo!()
    }
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(machine: &mut Machine, lpModuleName: Option<&str>) -> HMODULE {
    let name = match lpModuleName {
        None => return HMODULE::from_raw(machine.state.kernel32.image_base),
        Some(name) => name,
    };

    let name = name.to_ascii_lowercase();

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
pub fn GetModuleHandleW(machine: &mut Machine, lpModuleName: Option<Str16>) -> HMODULE {
    let ascii = lpModuleName.map(|str| str.to_string());
    GetModuleHandleA(machine, ascii.as_deref())
}

#[win32_derive::dllexport]
pub fn GetModuleHandleExW(
    machine: &mut Machine,
    dwFlags: u32,
    lpModuleName: Option<Str16>,
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
pub fn LoadLibraryA(machine: &mut Machine, filename: Option<&str>) -> HMODULE {
    let filename = filename.unwrap().to_ascii_lowercase();

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

    // Check if builtin.
    if let Some(builtin) = winapi::DLLS.iter().find(|&dll| dll.file_name == filename) {
        machine.state.kernel32.dlls.push(DLL {
            name: filename,
            names: HashMap::new(),
            builtin: Some(builtin),
        });
        return HMODULE::from_dll_index(machine.state.kernel32.dlls.len() - 1);
    }

    // TODO: load dll from disk using pe::load_dll.
    log::error!("LoadLibrary({filename:?}) unimplemented");

    HMODULE::null() // fail
}

#[win32_derive::dllexport]
pub fn LoadLibraryExW(
    machine: &mut Machine,
    lpLibFileName: Option<Str16>,
    hFile: HFILE,
    dwFlags: u32,
) -> HMODULE {
    let filename = lpLibFileName.map(|f| f.to_string());
    LoadLibraryA(machine, filename.as_deref())
}

#[win32_derive::dllexport]
pub fn GetProcAddress(machine: &mut Machine, hModule: HMODULE, lpProcName: Option<&str>) -> u32 {
    let proc_name = lpProcName.unwrap();
    if let Some(dll) = machine.state.kernel32.dlls.get_mut(hModule.to_dll_index()) {
        return dll.resolve(&mut machine.shims, ImportSymbol::Name(proc_name));
    }
    log::error!("GetProcAddress({:x?}, {:?})", hModule, lpProcName);
    0 // fail
}
