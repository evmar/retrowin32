use win32_system::System;

#[win32_derive::dllexport(cdecl)]
pub fn __setusermatherr(sys: &mut dyn System, pf: u32) {
    todo!();
}

#[win32_derive::dllexport]
pub const _adjust_fdiv: &'static str = "_adjust_fdiv";

// ftol is unique(?) in that it does not follow the calling convention,
// but rather pops a value from the FPU stack and returns it in edx:eax.
// We just use x86 assembly for this.
// TODO: implementations of this online typically set the RC flags of the
// FPU control register to 11, round towards zero.
#[win32_derive::dllexport(raw_asm)]
pub const _ftol: &'static str = "
  sub esp, 8
  fistp qword ptr [esp]
  mov eax, dword ptr [esp]
  mov edx, dword ptr [esp + 4]
  ret 8
";

#[win32_derive::dllexport(cdecl)]
pub fn sqrt(sys: &mut dyn System, x: f64) -> f64 {
    x.sqrt()
}

#[win32_derive::dllexport(cdecl)]
pub fn sin(sys: &mut dyn System, x: f64) -> f64 {
    x.sin()
}

#[win32_derive::dllexport(cdecl)]
pub fn cos(sys: &mut dyn System, x: f64) -> f64 {
    x.cos()
}

#[win32_derive::dllexport(cdecl)]
pub fn floor(sys: &mut dyn System, x: f64) -> f64 {
    x.floor()
}
