use crate::{
    machine::Machine,
    winapi::{self, types::*},
};
use x86::Memory;

const TRACE: bool = false;

// HMODULE is currently an index into the list of preallocated DLLs.
declare_handle!(HMODULE);

impl HMODULE {
    fn find_by_name(filename: &str) -> Option<Self> {
        if let Some(index) = crate::winapi::DLLS
            .iter()
            .position(|dll| dll.file_name == filename)
        {
            return Some(HMODULE::from_raw((index + 1) as u32));
        }
        None
    }

    fn to_dll(&self) -> Option<&winapi::BuiltinDLL> {
        winapi::DLLS.get(self.0 as usize - 1)
    }
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(machine: &mut Machine, lpModuleName: Option<&str>) -> HMODULE {
    if let Some(name) = lpModuleName {
        log::error!("unimplemented: GetModuleHandle({name:?})");
        return HMODULE::null();
    }
    // HMODULE is base address of current module.
    HMODULE::from_raw(machine.state.kernel32.image_base)
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
    let filename = filename.unwrap();
    let filename = filename.to_ascii_lowercase();

    if let Some(hmodule) = HMODULE::find_by_name(&filename) {
        return hmodule;
    }

    log::error!(
        "LoadLibrary({filename:?}) => {:x}",
        machine.x86.mem.read_u32(machine.x86.regs.esp - 4)
    );
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
    if let Some(dll) = hModule.to_dll() {
        // See if the symbol was already imported.
        let full_name = format!("{}!{}", dll.file_name, proc_name);
        if let Some(addr) = machine.shims.lookup(&full_name) {
            return addr;
        }

        let handler = (dll.resolve)(&winapi::ImportSymbol::Name(proc_name));
        let addr = machine.shims.add(full_name, handler);
        return addr;
    }
    log::error!("GetProcAddress({:x?}, {:?})", hModule, lpProcName);
    0 // fail
}
