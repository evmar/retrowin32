use crate::X86;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// However, we shim our implementations without the caller ever pushing EIP,
// so pop() immediately gives the rightmost argument and cleans the stack.

#[allow(non_snake_case)]
mod kernel32 {
    use super::*;

    pub fn GetModuleHandleA(x86: &mut X86) {
        let module_name = x86.pop();
        if module_name != 0 {
            log::error!("unimplemented: GetModuleHandle(non-null)")
        }
        // HMODULE is base address of current module.
        x86.regs.eax = x86.base;
    }
}

pub fn resolve(sym: &str) -> Option<fn(&mut X86)> {
    Some(match sym {
        "kernel32.dll!GetModuleHandleA" => kernel32::GetModuleHandleA,
        _ => return None,
    })
}
