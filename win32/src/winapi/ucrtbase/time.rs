use crate::Machine;

fn time64(machine: &mut Machine, destTime: Option<&mut u64>) -> u32 {
    let time = machine.host.system_time().timestamp() as u64;
    if let Some(destTime) = destTime {
        *destTime = time;
    }

    // TODO: 64-bit return values go through edx:eax, which is not yet modeled in the shims
    // machinery, so we only return 32 bits here.
    // Thankfully 32-bit time_t only overflows in 2038 anyway.
    time as u32
}

#[win32_derive::dllexport(cdecl)]
pub fn time(machine: &mut Machine, destTime: Option<&mut u64>) -> u32 {
    time64(machine, destTime)
}

#[win32_derive::dllexport(cdecl)]
pub fn _time64(machine: &mut Machine, destTime: Option<&mut u64>) -> u32 {
    time64(machine, destTime)
}
