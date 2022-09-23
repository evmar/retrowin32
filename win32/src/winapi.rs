use crate::X86;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// The caller of winapi functions is responsible for pushing/popping the
// return address, because some callers actually 'jmp' directly.

// For now, a magic variable that makes it easier to spot.
pub const STDOUT_HFILE: u32 = 0xF11E_0100;

#[allow(non_snake_case)]
mod kernel32 {
    use super::*;

    pub fn ExitProcess(x86: &mut X86) {
        let uExitCode = x86.pop();
        x86.os.exit(uExitCode);
    }

    pub fn GetModuleHandleA(x86: &mut X86) {
        let lpModuleName = x86.pop();
        if lpModuleName != 0 {
            log::error!("unimplemented: GetModuleHandle(non-null)")
        }
        // HMODULE is base address of current module.
        x86.regs.eax = x86.state.image_base;
    }

    pub fn WriteFile(x86: &mut X86) {
        let hFile = x86.pop();
        let lpBuffer = x86.pop();
        let nNumberOfBytesToWrite = x86.pop();
        let lpNumberOfBytesWritten = x86.pop();
        let lpOverlapped = x86.pop();

        assert!(hFile == STDOUT_HFILE);
        assert!(lpOverlapped == 0);
        let buf = &x86.mem[lpBuffer as usize..(lpBuffer + nNumberOfBytesToWrite) as usize];

        let n = x86.os.write(buf);

        x86.write_u32(lpNumberOfBytesWritten, n as u32);
        x86.regs.eax = 1;
    }

    pub fn VirtualAlloc(x86: &mut X86) {
        let lpAddress = x86.pop();
        let dwSize = x86.pop();
        let flAllocationType = x86.pop();
        let _flProtec = x86.pop();

        assert!(lpAddress == 0);
        // TODO round dwSize to page boundary
        assert!(flAllocationType == 0x1000); // MEM_COMMIT

        let mapping = x86.state.alloc(dwSize, "VirtualAlloc".into());
        x86.regs.eax = mapping.addr;
    }
}

#[allow(non_snake_case)]
mod user32 {
    use super::*;
    pub fn RegisterClassA(x86: &mut X86) {
        let lpWndClass = x86.pop();
        log::warn!("todo: RegisterClassA({:x})", lpWndClass);
    }
    pub fn CreateWindowExA(x86: &mut X86) {
        let dwExStyle = x86.pop();
        let lpClassName = x86.pop();
        let lpWindowName = x86.pop();
        let dwStyle = x86.pop();
        let X = x86.pop();
        let Y = x86.pop();
        let nWidth = x86.pop();
        let nHeight = x86.pop();
        let hWndParent = x86.pop();
        let hMenu = x86.pop();
        let hInstance = x86.pop();
        let lpParam = x86.pop();
        log::warn!("todo: CreateWindowExA({dwExStyle:x}, {lpClassName:x}, {lpWindowName:x}, {dwStyle:x}, {X:x}, {Y:x}, {nWidth:x}, {nHeight:x}, {hWndParent:x}, {hMenu:x}, {hInstance:x}, {lpParam:x})");
    }
    pub fn UpdateWindow(x86: &mut X86) {
        let hWnd = x86.pop();
        log::warn!("todo: UpdateWindow({hWnd:x})");
    }
}

pub fn resolve(sym: &str) -> Option<fn(&mut X86)> {
    Some(match sym {
        "kernel32.dll!ExitProcess" => kernel32::ExitProcess,
        "kernel32.dll!GetModuleHandleA" => kernel32::GetModuleHandleA,
        "kernel32.dll!WriteFile" => kernel32::WriteFile,
        "kernel32.dll!VirtualAlloc" => kernel32::VirtualAlloc,
        "user32.dll!RegisterClassA" => user32::RegisterClassA,
        "user32.dll!CreateWindowExA" => user32::CreateWindowExA,
        "user32.dll!UpdateWindow" => user32::UpdateWindow,
        _ => return None,
    })
}
