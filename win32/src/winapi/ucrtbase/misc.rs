use crate::{Machine, System, calling_convention::VarArgs};
use win32_system::host;

#[win32_derive::dllexport(cdecl)]
pub fn _exit(sys: &mut dyn System, status: u32) {
    sys.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(sys: &mut dyn System, status: u32) {
    sys.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn _onexit(sys: &mut dyn System, func: u32) -> u32 {
    // register onexit handler
    func
}

#[win32_derive::dllexport(cdecl)]
pub fn atexit(sys: &mut dyn System, func: u32) -> u32 {
    // register onexit handler
    0 // success
}

#[win32_derive::dllexport(cdecl)]
pub fn _cexit(sys: &mut dyn System) {
    // call atexit handlers but don't exit
}

#[win32_derive::dllexport(cdecl, symbol = "?terminate@@YAXXZ")]
pub fn terminate(sys: &dyn System) {
    todo!()
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
    builtin_user32::printf::printf(&mut out, fmt.unwrap(), args, machine.mem()).unwrap();
    1 // TODO: compute length written
}

#[win32_derive::dllexport(cdecl)]
pub fn sprintf(sys: &mut dyn System, buf: u32, fmt: Option<&str>, args: VarArgs) -> i32 {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn vfprintf(sys: &mut dyn System, buf: u32, fmt: Option<&str>, args: VarArgs) -> i32 {
    todo!()
}

// TODO: needs to be an array
#[win32_derive::dllexport]
pub const _iob: &'static str = "_iob";

// TODO: needs to be an array
#[win32_derive::dllexport]
pub const _winmajor: &'static str = "_winmajor";

#[win32_derive::dllexport(cdecl)]
pub fn abort(sys: &mut dyn System) {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn fwrite(sys: &mut dyn System, filename: Option<&str>, mode: Option<&str>) -> u32 {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn signal(sys: &mut dyn System, sig: u32, func: u32) {
    todo!()
}
