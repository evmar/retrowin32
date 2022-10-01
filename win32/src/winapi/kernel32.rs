#![allow(non_snake_case)]

use super::{x86, x86::X86};
use crate::winapi;
use tsify::Tsify;

// For now, a magic variable that makes it easier to spot.
pub const STDOUT_HFILE: u32 = 0xF11E_0100;
pub const STDERR_HFILE: u32 = 0xF11E_0101;

#[derive(Debug, tsify::Tsify, serde::Serialize)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
}

pub struct State {
    // Address image was loaded at.
    pub image_base: u32,
    // Address of TEB (FS register).
    pub teb: u32,
    pub mappings: Vec<Mapping>,
}
impl State {
    pub fn new() -> Self {
        let mappings = vec![Mapping {
            addr: 0,
            size: x86::NULL_POINTER_REGION_SIZE,
            desc: "avoid null pointers".into(),
        }];
        State {
            image_base: 0,
            teb: 0,
            mappings,
        }
    }

    pub fn add_mapping(&mut self, mapping: Mapping) {
        let pos = self
            .mappings
            .iter()
            .position(|m| m.addr > mapping.addr)
            .unwrap_or(self.mappings.len());
        if pos > 0 {
            let prev = &self.mappings[pos - 1];
            assert!(prev.addr + prev.size <= mapping.addr);
        }
        if pos < self.mappings.len() {
            let next = &self.mappings[pos];
            assert!(mapping.addr + mapping.size <= next.addr);
        }
        self.mappings.insert(pos, mapping);
    }

    pub fn alloc(&mut self, size: u32, desc: String, mem: &mut Vec<u8>) -> &Mapping {
        let mut end = 0;
        let pos = self
            .mappings
            .iter()
            .position(|mapping| {
                let space = mapping.addr - end;
                if space > size {
                    return true;
                }
                end = mapping.addr + mapping.size + (0x1000 - 1) & !(0x1000 - 1);
                false
            })
            .unwrap_or_else(|| {
                let space = mem.len() as u32 - end;
                if space < size {
                    mem.resize((end + size) as usize, 0);
                }
                self.mappings.len()
            });

        self.mappings.insert(
            pos,
            Mapping {
                addr: end,
                size,
                desc,
            },
        );
        return &self.mappings[pos];
    }
}

fn ExitProcess(x86: &mut X86, uExitCode: u32) -> u32 {
    x86.host.exit(uExitCode);
    0
}

fn GetEnvironmentVariableA(_x86: &mut X86, _lpName: u32, _lpBuffer: u32, _nSize: u32) -> u32 {
    // Fail for now.
    0
}

fn GetModuleFileNameA(_x86: &mut X86, hModule: u32, lpFilename: u32, nSize: u32) -> u32 {
    log::warn!("GetModuleFileNameA({hModule:x}, {lpFilename:x}, {nSize:x})");
    0 // fail
}

fn GetModuleHandleA(x86: &mut X86, lpModuleName: u32) -> u32 {
    if lpModuleName != 0 {
        log::error!("unimplemented: GetModuleHandle(non-null)")
    }
    // HMODULE is base address of current module.
    x86.state.kernel32.image_base
}

fn GetStdHandle(_x86: &mut X86, nStdHandle: u32) -> u32 {
    match nStdHandle as i32 {
        -10 => unimplemented!("GetStdHandle(stdin)"),
        -11 => STDOUT_HFILE,
        -12 => STDERR_HFILE,
        _ => (-1i32) as u32,
    }
}

fn GetVersion(_x86: &mut X86) -> u32 {
    // Win95, version 4.0.
    (1 << 31) | 0x4
}

fn GetVersionExA(_x86: &mut X86, _lpVersionInformation: u32) -> u32 {
    // Fail for now.
    0
}

fn HeapCreate(x86: &mut X86, flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32) -> u32 {
    log::warn!("HeapCreate({flOptions:x}, {dwInitialSize:x}, {dwMaximumSize:x})");
    let mapping = x86
        .state
        .kernel32
        .alloc(dwInitialSize, "HeapCreate".into(), &mut x86.mem);
    mapping.addr
}

fn HeapDestroy(_x86: &mut X86, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

fn LoadLibraryA(_x86: &mut X86, lpLibFileName: u32) -> u32 {
    log::warn!("LoadLibrary({lpLibFileName:x})");
    0 // fail
}

fn WriteFile(
    x86: &mut X86,
    hFile: u32,
    lpBuffer: u32,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: u32,
    lpOverlapped: u32,
) -> u32 {
    assert!(hFile == STDOUT_HFILE || hFile == STDERR_HFILE);
    assert!(lpOverlapped == 0);
    let buf = &x86.mem[lpBuffer as usize..(lpBuffer + nNumberOfBytesToWrite) as usize];

    let n = x86.host.write(buf);

    x86.write_u32(lpNumberOfBytesWritten, n as u32);
    1
}

fn VirtualAlloc(
    x86: &mut X86,
    lpAddress: u32,
    dwSize: u32,
    _flAllocationType: u32,
    _flProtec: u32,
) -> u32 {
    if lpAddress != 0 {
        log::warn!("failing VirtualAlloc({lpAddress:x}, {dwSize:x}, ...)");
        return 0;
    }
    // TODO round dwSize to page boundary

    let mapping = x86
        .state
        .kernel32
        .alloc(dwSize, "VirtualAlloc".into(), &mut x86.mem);
    mapping.addr
}

fn VirtualFree(_x86: &mut X86, lpAddress: u32, dwSize: u32, dwFreeType: u32) -> u32 {
    log::warn!("VirtualFree({lpAddress:x}, {dwSize:x}, {dwFreeType:x})");
    1 // success
}

winapi!(
    fn ExitProcess(uExitCode: u32);
    fn GetEnvironmentVariableA(lpName: u32, lpBuffer: u32, nSize: u32);
    fn GetModuleFileNameA(hModule: u32, lpFilename: u32, nSize: u32);
    fn GetModuleHandleA(lpModuleName: u32);
    fn GetStdHandle(nStdHandle: u32);
    fn GetVersion();
    fn GetVersionExA(lpVersionInformation: u32);
    fn HeapCreate(flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32);
    fn HeapDestroy(hHeap: u32);
    fn LoadLibraryA(lpLibFileName: u32);
    fn WriteFile(
        hFile: u32,
        lpBuffer: u32,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: u32,
        lpOverlapped: u32,
    );
    fn VirtualAlloc(lpAddress: u32, dwSize: u32, _flAllocationType: u32, _flProtec: u32);
    fn VirtualFree(lpAddress: u32, dwSize: u32, dwFreeType: u32);
);
