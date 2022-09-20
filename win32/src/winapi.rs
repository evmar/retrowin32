use crate::X86;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// However, we shim our implementations without the caller ever pushing EIP,
// so pop() immediately gives the rightmost argument and cleans the stack.

#[allow(non_snake_case)]
mod kernel32 {
    use super::*;

    pub fn GetModuleHandleA(x86: &mut X86) {
        let lpModuleName = x86.pop();
        if lpModuleName != 0 {
            log::error!("unimplemented: GetModuleHandle(non-null)")
        }
        // HMODULE is base address of current module.
        x86.regs.eax = x86.base;
    }
}

#[allow(non_snake_case)]
mod user32 {
    use super::*;
    pub fn RegisterClassA(x86: &mut X86) {
        let _lpWndClass = x86.pop();
        log::warn!("todo: RegisterClassA");
    }
}

pub fn resolve(sym: &str) -> Option<fn(&mut X86)> {
    Some(match sym {
        "kernel32.dll!GetModuleHandleA" => kernel32::GetModuleHandleA,
        "user32.dll!RegisterClassA" => user32::RegisterClassA,
        _ => return None,
    })
}
