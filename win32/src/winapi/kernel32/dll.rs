use crate::{
    machine::Machine,
    pe,
    shims::Shims,
    winapi::{self, types::*, BuiltinDLL, ImportSymbol},
};
use std::collections::HashMap;

const TRACE: bool = true;

// HMODULE is index+1 into kernel32::State::dlls.
declare_handle!(HMODULE);

impl HMODULE {
    fn from_dll_index(index: usize) -> Self {
        return HMODULE::from_raw((index + 1) as u32);
    }

    pub fn to_dll_index(&self) -> Option<usize> {
        if self.is_null() {
            return None;
        }
        Some(self.0 as usize - 1)
    }
}

pub struct DLL {
    name: String,

    /// Function name => resolved address.
    names: HashMap<String, u32>,

    /// Function ordinal => resolved address.
    ordinals: HashMap<u32, u32>,

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
            ImportSymbol::Ordinal(ord) => {
                if let Some(&addr) = self.ordinals.get(&ord) {
                    return addr;
                }
            }
        }

        if let Some(builtin) = self.builtin {
            let handler = (builtin.resolve)(&sym);
            let addr = shims.add(format!("{}!{}", self.name, sym.to_string()), handler);

            match sym {
                ImportSymbol::Name(name) => {
                    self.names.insert(name.to_string(), addr);
                }
                ImportSymbol::Ordinal(ord) => {
                    self.ordinals.insert(ord, addr);
                }
            }

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
            ordinals: HashMap::new(),
            builtin: Some(builtin),
        });
        return HMODULE::from_dll_index(machine.state.kernel32.dlls.len() - 1);
    }

    let mut file = machine.host.open(&filename);
    let mut contents = Vec::new();
    let mut buf: [u8; 16 << 10] = [0; 16 << 10];
    loop {
        let mut len = 0u32;
        assert!(file.read(&mut buf, &mut len));
        if len == 0 {
            break;
        }
        contents.extend_from_slice(&buf[..len as usize]);
    }
    // TODO: close file.
    if contents.len() == 0 {
        // HACK: zero-length indicates not found.
        return HMODULE::null();
    }

    let exports = pe::load_dll(machine, &filename, &contents).unwrap();
    let dll = DLL {
        name: filename,
        names: exports.names,
        ordinals: exports.ordinals,
        builtin: None,
    };
    machine.state.kernel32.dlls.push(dll);
    HMODULE::from_dll_index(machine.state.kernel32.dlls.len() - 1)
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
    let index = hModule.to_dll_index().unwrap();
    if let Some(dll) = machine.state.kernel32.dlls.get_mut(index) {
        return dll.resolve(&mut machine.shims, ImportSymbol::Name(proc_name));
    }
    log::error!("GetProcAddress({:x?}, {:?})", hModule, lpProcName);
    0 // fail
}
