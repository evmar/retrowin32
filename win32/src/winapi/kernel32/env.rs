use crate::{
    Machine,
    calling_convention::ArrayOut,
    winapi::{Str16, encoding::*},
};

/// Encode environment variables in the environment block format.
fn encode_env(w: &mut dyn Encoder, env: &[(String, String)]) {
    for (key, val) in env {
        w.write(key);
        w.write("=");
        w.write(val);
        w.write("\0");
    }
    if env.is_empty() {
        w.write("\0");
    }
    w.write("\0");
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings(machine: &mut Machine) -> u32 {
    // Yes, this function without "A" suffix exists:
    // https://devblogs.microsoft.com/oldnewthing/20130117-00/?p=5533
    let mut measure = EncoderAnsi::new(&mut []);
    encode_env(&mut measure, &machine.state.kernel32.env);
    let len = measure.status().unwrap_err();

    let addr = machine
        .state
        .kernel32
        .process_heap
        .alloc(machine.memory.mem(), len as u32);

    let mut env = EncoderAnsi::from_mem(machine.memory.mem(), addr, len as u32);
    encode_env(&mut env, &machine.state.kernel32.env);
    env.status().unwrap();

    addr
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(machine: &mut Machine, penv: u32) -> bool {
    machine
        .state
        .kernel32
        .process_heap
        .free(machine.memory.mem(), penv);
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(machine: &mut Machine) -> u32 {
    // Yuck, this is copy of GetEnvironmentStringsA, because we need to create two Encoder types.
    let mut measure = EncoderWide::new(&mut []);
    encode_env(&mut measure, &machine.state.kernel32.env);
    let len = measure.status().unwrap_err();

    let addr = machine
        .state
        .kernel32
        .process_heap
        .alloc(machine.memory.mem(), len as u32);

    let mut env = EncoderWide::from_mem(machine.memory.mem(), addr, len as u32);
    encode_env(&mut env, &machine.state.kernel32.env);
    env.status().unwrap();

    addr
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(machine: &mut Machine, penv: u32) -> bool {
    machine
        .state
        .kernel32
        .process_heap
        .free(machine.memory.mem(), penv);
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    buf: ArrayOut<u8>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableW(
    _machine: &mut Machine,
    name: Option<&Str16>,
    buf: ArrayOut<u16>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableW(
    _machine: &mut Machine,
    lpName: Option<&Str16>,
    lpValue: Option<&Str16>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    value: Option<&str>,
) -> bool {
    true
}
