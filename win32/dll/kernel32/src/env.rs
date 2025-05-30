use win32_system::System;
use win32_winapi::{Str16, calling_convention::ArrayOut, encoding::*};

type State = Vec<(String, String)>;

fn initial_environment() -> State {
    vec![(String::from("WINDIR"), String::from("C:\\Windows"))]
}

fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    type SysState = std::cell::RefCell<State>;
    sys.state(&std::any::TypeId::of::<SysState>(), || {
        Box::new(std::cell::RefCell::new(initial_environment()))
    })
    .downcast_ref::<SysState>()
    .unwrap()
    .borrow_mut()
}

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
pub fn GetEnvironmentStrings(sys: &dyn System) -> u32 {
    // Yes, this function without "A" suffix exists:
    // https://devblogs.microsoft.com/oldnewthing/20130117-00/?p=5533
    let mut measure = EncoderAnsi::new(&mut []);
    let env = get_state(sys);
    encode_env(&mut measure, &env);
    let len = measure.status().unwrap_err();

    let addr = sys.memory().process_heap.alloc(sys.mem(), len as u32);

    let mut encoder = EncoderAnsi::from_mem(sys.mem(), addr, len as u32);
    encode_env(&mut encoder, &env);
    encoder.status().unwrap();

    addr
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(sys: &dyn System, penv: u32) -> bool {
    sys.memory().process_heap.free(sys.mem(), penv);
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(sys: &dyn System) -> u32 {
    // Yuck, this is copy of GetEnvironmentStringsA, because we need to create two Encoder types.
    let mut measure = EncoderWide::new(&mut []);
    let env = get_state(sys);
    encode_env(&mut measure, &env);
    let len = measure.status().unwrap_err();

    let addr = sys.memory().process_heap.alloc(sys.mem(), len as u32);

    let mut encoder = EncoderWide::from_mem(sys.mem(), addr, len as u32);
    encode_env(&mut encoder, &env);
    encoder.status().unwrap();

    addr
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(sys: &dyn System, penv: u32) -> bool {
    sys.memory().process_heap.free(sys.mem(), penv);
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(sys: &dyn System, name: Option<&str>, buf: ArrayOut<u8>) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableW(sys: &dyn System, name: Option<&Str16>, buf: ArrayOut<u16>) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableW(
    sys: &dyn System,
    lpName: Option<&Str16>,
    lpValue: Option<&Str16>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableA(sys: &dyn System, name: Option<&str>, value: Option<&str>) -> bool {
    true
}
