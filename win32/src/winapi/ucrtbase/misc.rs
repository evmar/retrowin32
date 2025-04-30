use crate::{Machine, System, calling_convention::VarArgs, host, winapi::CStr};

#[win32_derive::dllexport(cdecl)]
pub fn _exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn _onexit(machine: &mut Machine, func: u32) -> u32 {
    // register onexit handler
    func
}

#[win32_derive::dllexport(cdecl)]
pub fn atexit(machine: &mut Machine, func: u32) -> u32 {
    // register onexit handler
    0 // success
}

#[win32_derive::dllexport(cdecl)]
pub fn _cexit(machine: &mut Machine) {
    // call atexit handlers but don't exit
}

#[win32_derive::dllexport(cdecl, symbol = "?terminate@@YAXXZ")]
pub fn terminate(sys: &dyn System) {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn strlen(sys: &dyn System, lpString: Option<&CStr>) -> u32 {
    // The mapping to str already computes the string length.
    lpString.unwrap().count_bytes() as u32
}

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(machine: &mut Machine, dest: u32, src: u32, count: u32) -> u32 {
    machine.mem().copy(src, dest, count);
    dest
}

struct HostStdout<'a>(&'a dyn host::Host);

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

#[win32_derive::dllexport(cdecl)]
pub fn sprintf(machine: &mut Machine, buf: u32, fmt: Option<&str>, args: VarArgs) -> i32 {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn vfprintf(machine: &mut Machine, buf: u32, fmt: Option<&str>, args: VarArgs) -> i32 {
    todo!()
}

// TODO: needs to be an array
#[win32_derive::dllexport]
pub const _iob: &'static str = "_iob";

// TODO: needs to be an array
#[win32_derive::dllexport]
pub const _winmajor: &'static str = "_winmajor";

#[win32_derive::dllexport(cdecl)]
pub fn abort(machine: &mut Machine) {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn fwrite(machine: &mut Machine, filename: Option<&str>, mode: Option<&str>) -> u32 {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn signal(machine: &mut Machine, sig: u32, func: u32) {
    todo!()
}
