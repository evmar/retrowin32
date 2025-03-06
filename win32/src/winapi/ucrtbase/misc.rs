use crate::{calling_convention::VarArgs, winapi::CStr, Machine};

#[win32_derive::dllexport(cdecl)]
pub fn _exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn strlen(_machine: &mut Machine, lpString: Option<&CStr>) -> u32 {
    // The mapping to str already computes the string length.
    lpString.unwrap().count_bytes() as u32
}

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(machine: &mut Machine, dest: u32, src: u32, count: u32) -> u32 {
    machine.mem().copy(src, dest, count);
    dest
}

struct HostStdout<'a>(&'a dyn crate::host::Host);

impl<'a> std::io::Write for HostStdout<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.stdout(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn printf(machine: &mut Machine, fmt: Option<&str>, args: VarArgs) -> i32 {
    let mut out = HostStdout(machine.host.as_ref());
    crate::winapi::printf::printf(&mut out, fmt.unwrap(), args, machine.mem()).unwrap();
    1 // TODO: compute length written
}
