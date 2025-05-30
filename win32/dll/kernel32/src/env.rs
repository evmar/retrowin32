use win32_system::{System, generic_get_state};
use win32_winapi::{Str16, calling_convention::ArrayOut, encoding::*};

struct State {
    env: Vec<(String, String)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            env: vec![(String::from("WINDIR"), String::from("C:\\Windows"))],
        }
    }
}

#[inline]
fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    generic_get_state::<State>(sys)
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
    let state = get_state(sys);
    encode_env(&mut measure, &state.env);
    let len = measure.status().unwrap_err();

    let addr = sys.memory().process_heap.alloc(sys.mem(), len as u32);

    let mut encoder = EncoderAnsi::from_mem(sys.mem(), addr, len as u32);
    encode_env(&mut encoder, &state.env);
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
    let state = get_state(sys);
    encode_env(&mut measure, &state.env);
    let len = measure.status().unwrap_err();

    let addr = sys.memory().process_heap.alloc(sys.mem(), len as u32);

    let mut encoder = EncoderWide::from_mem(sys.mem(), addr, len as u32);
    encode_env(&mut encoder, &state.env);
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
