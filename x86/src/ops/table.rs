//! Efficiently maps an iced_x86::Code (roughly x86 opcode) to a implementation of the op.

use crate::x86::CPU;
use iced_x86::Instruction;
use memory::Mem;

/// The type of all operations defined in the ops module.
pub type Op = fn(&mut CPU, Mem, &Instruction);

// This table is constant and ideally would be initialized at compile time,
// but it's too fiddly to do with const fns, so we'd likely need to codegen it.
static mut OP_TAB: [Option<Op>; 2546] = [None; 2546];

pub unsafe fn init_op_tab() {
    use super::basic::*;
    use super::control::*;
    use super::cpuid::*;
    use super::fpu::*;
    use super::math::*;
    use super::mmx::*;
    use super::string::*;
    use super::test::*;

    OP_TAB[iced_x86::Code::Enterd_imm16_imm8 as usize] = Some(enterd_imm16_imm8);
    OP_TAB[iced_x86::Code::Leaved as usize] = Some(leaved);
    OP_TAB[iced_x86::Code::Call_rel32_32 as usize] = Some(call);
    OP_TAB[iced_x86::Code::Call_rm32 as usize] = Some(call_rm32);
    OP_TAB[iced_x86::Code::Retnd as usize] = Some(retnd);
    OP_TAB[iced_x86::Code::Retnd_imm16 as usize] = Some(retnd_imm16);
    OP_TAB[iced_x86::Code::Jmp_rel32_32 as usize] = Some(jmp);
    OP_TAB[iced_x86::Code::Jmp_rel8_32 as usize] = Some(jmp);
    OP_TAB[iced_x86::Code::Jmp_rm32 as usize] = Some(jmp_rm32);
    OP_TAB[iced_x86::Code::Ja_rel32_32 as usize] = Some(ja);
    OP_TAB[iced_x86::Code::Ja_rel8_32 as usize] = Some(ja);
    OP_TAB[iced_x86::Code::Jae_rel32_32 as usize] = Some(jae);
    OP_TAB[iced_x86::Code::Jae_rel8_32 as usize] = Some(jae);
    OP_TAB[iced_x86::Code::Jb_rel32_32 as usize] = Some(jb);
    OP_TAB[iced_x86::Code::Jb_rel8_32 as usize] = Some(jb);
    OP_TAB[iced_x86::Code::Jbe_rel32_32 as usize] = Some(jbe);
    OP_TAB[iced_x86::Code::Jbe_rel8_32 as usize] = Some(jbe);
    OP_TAB[iced_x86::Code::Je_rel32_32 as usize] = Some(je);
    OP_TAB[iced_x86::Code::Je_rel8_32 as usize] = Some(je);
    OP_TAB[iced_x86::Code::Jecxz_rel8_32 as usize] = Some(jecxz);
    OP_TAB[iced_x86::Code::Jne_rel32_32 as usize] = Some(jne);
    OP_TAB[iced_x86::Code::Jne_rel8_32 as usize] = Some(jne);
    OP_TAB[iced_x86::Code::Jns_rel32_32 as usize] = Some(jns);
    OP_TAB[iced_x86::Code::Jns_rel8_32 as usize] = Some(jns);
    OP_TAB[iced_x86::Code::Jg_rel32_32 as usize] = Some(jg);
    OP_TAB[iced_x86::Code::Jg_rel8_32 as usize] = Some(jg);
    OP_TAB[iced_x86::Code::Jge_rel32_32 as usize] = Some(jge);
    OP_TAB[iced_x86::Code::Jge_rel8_32 as usize] = Some(jge);
    OP_TAB[iced_x86::Code::Jle_rel32_32 as usize] = Some(jle);
    OP_TAB[iced_x86::Code::Jle_rel8_32 as usize] = Some(jle);
    OP_TAB[iced_x86::Code::Jl_rel32_32 as usize] = Some(jl);
    OP_TAB[iced_x86::Code::Jl_rel8_32 as usize] = Some(jl);
    OP_TAB[iced_x86::Code::Js_rel32_32 as usize] = Some(js);
    OP_TAB[iced_x86::Code::Js_rel8_32 as usize] = Some(js);

    OP_TAB[iced_x86::Code::Loop_rel8_32_ECX as usize] = Some(loop_);

    OP_TAB[iced_x86::Code::Pushd_DS as usize] = Some(pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_ES as usize] = Some(pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_FS as usize] = Some(pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_GS as usize] = Some(pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_imm8 as usize] = Some(pushd_imm8);
    OP_TAB[iced_x86::Code::Pushd_imm32 as usize] = Some(pushd_imm32);
    OP_TAB[iced_x86::Code::Push_r32 as usize] = Some(push_r32);
    OP_TAB[iced_x86::Code::Push_rm32 as usize] = Some(push_rm32);
    OP_TAB[iced_x86::Code::Push_rm16 as usize] = Some(push_rm16);
    OP_TAB[iced_x86::Code::Push_r16 as usize] = Some(push_rm16);

    OP_TAB[iced_x86::Code::Popd_DS as usize] = Some(popd_r16);
    OP_TAB[iced_x86::Code::Popd_ES as usize] = Some(popd_r16);
    OP_TAB[iced_x86::Code::Popd_FS as usize] = Some(popd_r16);
    OP_TAB[iced_x86::Code::Popd_GS as usize] = Some(popd_r16);
    OP_TAB[iced_x86::Code::Pop_r32 as usize] = Some(pop_rm32);
    OP_TAB[iced_x86::Code::Pop_rm32 as usize] = Some(pop_rm32);
    OP_TAB[iced_x86::Code::Pop_r16 as usize] = Some(pop_rm16);
    OP_TAB[iced_x86::Code::Pop_rm16 as usize] = Some(pop_rm16);

    OP_TAB[iced_x86::Code::Mov_rm32_imm32 as usize] = Some(mov_rm32_imm32);
    OP_TAB[iced_x86::Code::Mov_r32_imm32 as usize] = Some(mov_rm32_imm32);
    OP_TAB[iced_x86::Code::Mov_moffs32_EAX as usize] = Some(mov_rm32_r32);
    OP_TAB[iced_x86::Code::Mov_rm32_r32 as usize] = Some(mov_rm32_r32);
    OP_TAB[iced_x86::Code::Mov_r32_rm32 as usize] = Some(mov_r32_rm32);
    OP_TAB[iced_x86::Code::Mov_EAX_moffs32 as usize] = Some(mov_r32_rm32);
    OP_TAB[iced_x86::Code::Mov_r16_rm16 as usize] = Some(mov_r16_rm16);
    OP_TAB[iced_x86::Code::Mov_AX_moffs16 as usize] = Some(mov_r16_rm16);
    OP_TAB[iced_x86::Code::Mov_rm16_r16 as usize] = Some(mov_rm16_r16);
    OP_TAB[iced_x86::Code::Mov_moffs16_AX as usize] = Some(mov_rm16_r16);
    OP_TAB[iced_x86::Code::Mov_rm16_imm16 as usize] = Some(mov_rm16_imm16);
    OP_TAB[iced_x86::Code::Mov_r16_imm16 as usize] = Some(mov_rm16_imm16);
    OP_TAB[iced_x86::Code::Mov_r8_rm8 as usize] = Some(mov_r8_rm8);
    OP_TAB[iced_x86::Code::Mov_AL_moffs8 as usize] = Some(mov_r8_rm8);
    OP_TAB[iced_x86::Code::Mov_rm8_r8 as usize] = Some(mov_rm8_r8);
    OP_TAB[iced_x86::Code::Mov_r8_imm8 as usize] = Some(mov_rm8_imm8);
    OP_TAB[iced_x86::Code::Mov_rm8_imm8 as usize] = Some(mov_rm8_imm8);
    OP_TAB[iced_x86::Code::Mov_moffs8_AL as usize] = Some(mov_moffs8_al);
    OP_TAB[iced_x86::Code::Mov_r32m16_Sreg as usize] = Some(mov_r32m16_sreg);
    OP_TAB[iced_x86::Code::Mov_Sreg_r32m16 as usize] = Some(mov_sreg_r32m16);

    OP_TAB[iced_x86::Code::Movsx_r32_rm16 as usize] = Some(movsx_r32_rm16);
    OP_TAB[iced_x86::Code::Movsx_r32_rm8 as usize] = Some(movsx_r32_rm8);
    OP_TAB[iced_x86::Code::Movsx_r16_rm8 as usize] = Some(movsx_r16_rm8);

    OP_TAB[iced_x86::Code::Movzx_r32_rm16 as usize] = Some(movzx_r32_rm16);
    OP_TAB[iced_x86::Code::Movzx_r32_rm8 as usize] = Some(movzx_r32_rm8);
    OP_TAB[iced_x86::Code::Movzx_r16_rm8 as usize] = Some(movzx_r16_rm8);

    OP_TAB[iced_x86::Code::Cmovb_r32_rm32 as usize] = Some(cmovb_r32_rm32);
    OP_TAB[iced_x86::Code::Cmovne_r32_rm32 as usize] = Some(cmovne_r32_rm32);

    OP_TAB[iced_x86::Code::Xchg_rm32_r32 as usize] = Some(xchg_rm32_r32);
    OP_TAB[iced_x86::Code::Xchg_r32_EAX as usize] = Some(xchg_rm32_r32);
    OP_TAB[iced_x86::Code::Xchg_r16_AX as usize] = Some(xchg_rm16_r16);
    OP_TAB[iced_x86::Code::Xchg_rm8_r8 as usize] = Some(xchg_rm8_r8);

    OP_TAB[iced_x86::Code::Cmpxchg_rm32_r32 as usize] = Some(cmpxchg_rm32_r32);
    OP_TAB[iced_x86::Code::Cmpxchg8b_m64 as usize] = Some(cmpxchg8b_m64);

    OP_TAB[iced_x86::Code::Cmpsd_m32_m32 as usize] = Some(cmpsd);
    OP_TAB[iced_x86::Code::Cmpsw_m16_m16 as usize] = Some(cmpsw);
    OP_TAB[iced_x86::Code::Cmpsb_m8_m8 as usize] = Some(cmpsb);
    OP_TAB[iced_x86::Code::Movsd_m32_m32 as usize] = Some(movsd);
    OP_TAB[iced_x86::Code::Movsw_m16_m16 as usize] = Some(movsw);
    OP_TAB[iced_x86::Code::Movsb_m8_m8 as usize] = Some(movsb);
    OP_TAB[iced_x86::Code::Scasd_EAX_m32 as usize] = Some(scasd);
    OP_TAB[iced_x86::Code::Scasw_AX_m16 as usize] = Some(scasw);
    OP_TAB[iced_x86::Code::Scasb_AL_m8 as usize] = Some(scasb);
    OP_TAB[iced_x86::Code::Stosd_m32_EAX as usize] = Some(stosd);
    OP_TAB[iced_x86::Code::Stosw_m16_AX as usize] = Some(stosw);
    OP_TAB[iced_x86::Code::Stosb_m8_AL as usize] = Some(stosb);
    OP_TAB[iced_x86::Code::Lodsd_EAX_m32 as usize] = Some(lodsd);
    OP_TAB[iced_x86::Code::Lodsw_AX_m16 as usize] = Some(lodsw);
    OP_TAB[iced_x86::Code::Lodsb_AL_m8 as usize] = Some(lodsb);

    OP_TAB[iced_x86::Code::And_rm32_imm32 as usize] = Some(and_rm32_imm32);
    OP_TAB[iced_x86::Code::And_EAX_imm32 as usize] = Some(and_rm32_imm32);
    OP_TAB[iced_x86::Code::And_rm32_imm8 as usize] = Some(and_rm32_imm8);
    OP_TAB[iced_x86::Code::And_rm32_r32 as usize] = Some(and_rm32_r32);
    OP_TAB[iced_x86::Code::And_r32_rm32 as usize] = Some(and_r32_rm32);
    OP_TAB[iced_x86::Code::And_rm16_imm8 as usize] = Some(and_rm16_imm8);
    OP_TAB[iced_x86::Code::And_rm16_imm16 as usize] = Some(and_rm16_imm16);
    OP_TAB[iced_x86::Code::And_AX_imm16 as usize] = Some(and_rm16_imm16);
    OP_TAB[iced_x86::Code::And_rm16_r16 as usize] = Some(and_rm16_r16);
    OP_TAB[iced_x86::Code::And_rm8_r8 as usize] = Some(and_rm8_r8);
    OP_TAB[iced_x86::Code::And_rm8_imm8 as usize] = Some(and_rm8_imm8);
    OP_TAB[iced_x86::Code::And_AL_imm8 as usize] = Some(and_rm8_imm8);

    OP_TAB[iced_x86::Code::Or_rm32_r32 as usize] = Some(or_rm32_rm32);
    OP_TAB[iced_x86::Code::Or_r32_rm32 as usize] = Some(or_rm32_rm32);
    OP_TAB[iced_x86::Code::Or_rm32_imm32 as usize] = Some(or_rm32_imm32);
    OP_TAB[iced_x86::Code::Or_EAX_imm32 as usize] = Some(or_rm32_imm32);
    OP_TAB[iced_x86::Code::Or_rm32_imm8 as usize] = Some(or_rm32_imm8);
    OP_TAB[iced_x86::Code::Or_rm16_imm16 as usize] = Some(or_rm16_imm16);
    OP_TAB[iced_x86::Code::Or_rm16_r16 as usize] = Some(or_rm16_r16);
    OP_TAB[iced_x86::Code::Or_rm8_r8 as usize] = Some(or_rm8_r8);
    OP_TAB[iced_x86::Code::Or_rm8_imm8 as usize] = Some(or_rm8_imm8);
    OP_TAB[iced_x86::Code::Or_r8_rm8 as usize] = Some(or_r8_rm8);
    OP_TAB[iced_x86::Code::Or_AL_imm8 as usize] = Some(or_rm8_imm8);

    OP_TAB[iced_x86::Code::Shl_rm32_imm8 as usize] = Some(shl_rm32_imm8);
    OP_TAB[iced_x86::Code::Shl_rm32_1 as usize] = Some(shl_rm32_imm8);
    OP_TAB[iced_x86::Code::Shl_rm32_CL as usize] = Some(shl_rm32_cl);
    OP_TAB[iced_x86::Code::Shl_rm16_imm8 as usize] = Some(shl_rm16_imm8);
    OP_TAB[iced_x86::Code::Shl_rm16_1 as usize] = Some(shl_rm16_imm8);
    OP_TAB[iced_x86::Code::Shl_rm8_CL as usize] = Some(shl_rm8_cl);
    OP_TAB[iced_x86::Code::Shl_rm8_imm8 as usize] = Some(shl_rm8_imm8);

    OP_TAB[iced_x86::Code::Shld_rm32_r32_imm8 as usize] = Some(shld_rm32_r32_imm8);
    OP_TAB[iced_x86::Code::Shld_rm32_r32_CL as usize] = Some(shld_rm32_r32_cl);

    OP_TAB[iced_x86::Code::Shr_rm32_CL as usize] = Some(shr_rm32_cl);
    OP_TAB[iced_x86::Code::Shr_rm32_1 as usize] = Some(shr_rm32_1);
    OP_TAB[iced_x86::Code::Shr_rm32_imm8 as usize] = Some(shr_rm32_imm8);
    OP_TAB[iced_x86::Code::Shr_rm16_imm8 as usize] = Some(shr_rm16_imm8);
    OP_TAB[iced_x86::Code::Shr_rm16_1 as usize] = Some(shr_rm16_1);
    OP_TAB[iced_x86::Code::Shr_rm8_imm8 as usize] = Some(shr_rm8_imm8);
    OP_TAB[iced_x86::Code::Shr_rm8_1 as usize] = Some(shr_rm8_imm8);
    OP_TAB[iced_x86::Code::Shr_rm8_CL as usize] = Some(shr_rm8_cl);

    OP_TAB[iced_x86::Code::Shrd_rm32_r32_imm8 as usize] = Some(shrd_rm32_r32_imm8);
    OP_TAB[iced_x86::Code::Shrd_rm32_r32_CL as usize] = Some(shrd_rm32_r32_cl);

    OP_TAB[iced_x86::Code::Sar_rm32_imm8 as usize] = Some(sar_rm32_imm8);
    OP_TAB[iced_x86::Code::Sar_rm32_1 as usize] = Some(sar_rm32_imm8);
    OP_TAB[iced_x86::Code::Sar_rm32_CL as usize] = Some(sar_rm32_cl);
    OP_TAB[iced_x86::Code::Sar_rm8_1 as usize] = Some(sar_rm8_imm8);
    OP_TAB[iced_x86::Code::Sar_rm8_CL as usize] = Some(sar_rm8_cl);

    OP_TAB[iced_x86::Code::Rol_rm32_CL as usize] = Some(rol_rm32_cl);
    OP_TAB[iced_x86::Code::Rol_rm32_imm8 as usize] = Some(rol_rm32_imm8);
    OP_TAB[iced_x86::Code::Rol_rm8_imm8 as usize] = Some(rol_rm8_imm8);
    OP_TAB[iced_x86::Code::Rol_rm8_1 as usize] = Some(rol_rm8_imm8);
    OP_TAB[iced_x86::Code::Rol_rm8_CL as usize] = Some(rol_rm8_cl);
    OP_TAB[iced_x86::Code::Ror_rm32_CL as usize] = Some(ror_rm32_cl);
    OP_TAB[iced_x86::Code::Ror_rm32_imm8 as usize] = Some(ror_rm32_imm8);
    OP_TAB[iced_x86::Code::Ror_rm8_imm8 as usize] = Some(ror_rm8_imm8);
    OP_TAB[iced_x86::Code::Ror_rm8_1 as usize] = Some(ror_rm8_imm8);
    OP_TAB[iced_x86::Code::Ror_rm8_CL as usize] = Some(ror_rm8_cl);

    OP_TAB[iced_x86::Code::Xor_rm32_r32 as usize] = Some(xor_rm32_rm32);
    OP_TAB[iced_x86::Code::Xor_r32_rm32 as usize] = Some(xor_rm32_rm32);
    OP_TAB[iced_x86::Code::Xor_rm32_imm32 as usize] = Some(xor_rm32_imm32);
    OP_TAB[iced_x86::Code::Xor_EAX_imm32 as usize] = Some(xor_rm32_imm32);
    OP_TAB[iced_x86::Code::Xor_rm32_imm8 as usize] = Some(xor_rm32_imm8);
    OP_TAB[iced_x86::Code::Xor_r16_rm16 as usize] = Some(xor_rm16_rm16);
    OP_TAB[iced_x86::Code::Xor_rm16_r16 as usize] = Some(xor_rm16_rm16);
    OP_TAB[iced_x86::Code::Xor_rm8_imm8 as usize] = Some(xor_rm8_imm8);
    OP_TAB[iced_x86::Code::Xor_AL_imm8 as usize] = Some(xor_rm8_imm8);
    OP_TAB[iced_x86::Code::Xor_r8_rm8 as usize] = Some(xor_r8_rm8);
    OP_TAB[iced_x86::Code::Xor_rm8_r8 as usize] = Some(xor_rm8_r8);
    OP_TAB[iced_x86::Code::Add_r32_rm32 as usize] = Some(add_r32_rm32);
    OP_TAB[iced_x86::Code::Add_rm32_r32 as usize] = Some(add_rm32_r32);
    OP_TAB[iced_x86::Code::Add_rm32_imm32 as usize] = Some(add_rm32_imm32);
    OP_TAB[iced_x86::Code::Add_EAX_imm32 as usize] = Some(add_rm32_imm32);
    OP_TAB[iced_x86::Code::Add_rm32_imm8 as usize] = Some(add_rm32_imm8);
    OP_TAB[iced_x86::Code::Add_rm16_imm16 as usize] = Some(add_rm16_imm16);
    OP_TAB[iced_x86::Code::Add_AX_imm16 as usize] = Some(add_rm16_imm16);
    OP_TAB[iced_x86::Code::Add_rm16_imm8 as usize] = Some(add_rm16_imm8);
    OP_TAB[iced_x86::Code::Add_rm16_r16 as usize] = Some(add_rm16_rm16);
    OP_TAB[iced_x86::Code::Add_r16_rm16 as usize] = Some(add_rm16_rm16);
    OP_TAB[iced_x86::Code::Add_rm8_r8 as usize] = Some(add_rm8_r8);
    OP_TAB[iced_x86::Code::Add_rm8_imm8 as usize] = Some(add_rm8_imm8);
    OP_TAB[iced_x86::Code::Add_AL_imm8 as usize] = Some(add_rm8_imm8);
    OP_TAB[iced_x86::Code::Add_r8_rm8 as usize] = Some(add_r8_rm8);
    OP_TAB[iced_x86::Code::Adc_rm32_r32 as usize] = Some(adc_rm32_rm32);
    OP_TAB[iced_x86::Code::Adc_r32_rm32 as usize] = Some(adc_rm32_rm32);
    OP_TAB[iced_x86::Code::Adc_rm32_imm8 as usize] = Some(adc_rm32_imm8);
    OP_TAB[iced_x86::Code::Adc_rm16_imm8 as usize] = Some(adc_rm16_imm8);
    OP_TAB[iced_x86::Code::Adc_rm8_r8 as usize] = Some(adc_rm8_rm8);
    OP_TAB[iced_x86::Code::Adc_r8_rm8 as usize] = Some(adc_rm8_rm8);
    OP_TAB[iced_x86::Code::Adc_rm8_imm8 as usize] = Some(adc_rm8_imm8);
    OP_TAB[iced_x86::Code::Adc_AL_imm8 as usize] = Some(adc_rm8_imm8);
    OP_TAB[iced_x86::Code::Sub_rm32_imm8 as usize] = Some(sub_rm32_imm8);
    OP_TAB[iced_x86::Code::Sub_EAX_imm32 as usize] = Some(sub_rm32_imm32);
    OP_TAB[iced_x86::Code::Sub_rm32_imm32 as usize] = Some(sub_rm32_imm32);
    OP_TAB[iced_x86::Code::Sub_rm32_r32 as usize] = Some(sub_rm32_r32);
    OP_TAB[iced_x86::Code::Sub_r32_rm32 as usize] = Some(sub_r32_rm32);
    OP_TAB[iced_x86::Code::Sub_r16_rm16 as usize] = Some(sub_rm16_rm16);
    OP_TAB[iced_x86::Code::Sub_rm16_r16 as usize] = Some(sub_rm16_rm16);
    OP_TAB[iced_x86::Code::Sub_rm16_imm16 as usize] = Some(sub_rm16_imm16);
    OP_TAB[iced_x86::Code::Sub_AX_imm16 as usize] = Some(sub_rm16_imm16);
    OP_TAB[iced_x86::Code::Sub_r8_rm8 as usize] = Some(sub_rm8_rm8);
    OP_TAB[iced_x86::Code::Sub_rm8_r8 as usize] = Some(sub_rm8_rm8);
    OP_TAB[iced_x86::Code::Sub_rm8_imm8 as usize] = Some(sub_rm8_imm8);
    OP_TAB[iced_x86::Code::Sub_AL_imm8 as usize] = Some(sub_rm8_imm8);
    OP_TAB[iced_x86::Code::Sbb_r32_rm32 as usize] = Some(sbb_r32_rm32);
    OP_TAB[iced_x86::Code::Sbb_rm32_r32 as usize] = Some(sbb_rm32_r32);
    OP_TAB[iced_x86::Code::Sbb_rm32_imm8 as usize] = Some(sbb_rm32_imm8);
    OP_TAB[iced_x86::Code::Sbb_r8_rm8 as usize] = Some(sbb_r8_rm8);
    OP_TAB[iced_x86::Code::Sbb_AL_imm8 as usize] = Some(sbb_r8_imm8);
    OP_TAB[iced_x86::Code::Mul_rm32 as usize] = Some(mul_rm32);
    OP_TAB[iced_x86::Code::Mul_rm16 as usize] = Some(mul_rm16);
    OP_TAB[iced_x86::Code::Mul_rm8 as usize] = Some(mul_rm8);
    OP_TAB[iced_x86::Code::Imul_rm32 as usize] = Some(imul_rm32);
    OP_TAB[iced_x86::Code::Imul_rm16 as usize] = Some(imul_rm16);
    OP_TAB[iced_x86::Code::Imul_rm8 as usize] = Some(imul_rm8);
    OP_TAB[iced_x86::Code::Imul_r32_rm32 as usize] = Some(imul_r32_rm32);
    OP_TAB[iced_x86::Code::Imul_r32_rm32_imm32 as usize] = Some(imul_r32_rm32_imm32);
    OP_TAB[iced_x86::Code::Imul_r32_rm32_imm8 as usize] = Some(imul_r32_rm32_imm8);
    OP_TAB[iced_x86::Code::Idiv_rm32 as usize] = Some(idiv_rm32);
    OP_TAB[iced_x86::Code::Idiv_rm16 as usize] = Some(idiv_rm16);
    OP_TAB[iced_x86::Code::Idiv_rm8 as usize] = Some(idiv_rm8);
    OP_TAB[iced_x86::Code::Div_rm32 as usize] = Some(div_rm32);
    OP_TAB[iced_x86::Code::Div_rm16 as usize] = Some(div_rm16);
    OP_TAB[iced_x86::Code::Div_rm8 as usize] = Some(div_rm8);
    OP_TAB[iced_x86::Code::Dec_r32 as usize] = Some(dec_rm32);
    OP_TAB[iced_x86::Code::Dec_rm32 as usize] = Some(dec_rm32);
    OP_TAB[iced_x86::Code::Dec_r16 as usize] = Some(dec_rm16);
    OP_TAB[iced_x86::Code::Dec_rm8 as usize] = Some(dec_rm8);
    OP_TAB[iced_x86::Code::Inc_r32 as usize] = Some(inc_rm32);
    OP_TAB[iced_x86::Code::Inc_rm32 as usize] = Some(inc_rm32);
    OP_TAB[iced_x86::Code::Inc_rm16 as usize] = Some(inc_rm16);
    OP_TAB[iced_x86::Code::Inc_r16 as usize] = Some(inc_rm16);
    OP_TAB[iced_x86::Code::Inc_rm8 as usize] = Some(inc_rm8);
    OP_TAB[iced_x86::Code::Neg_rm32 as usize] = Some(neg_rm32);
    OP_TAB[iced_x86::Code::Neg_rm16 as usize] = Some(neg_rm16);
    OP_TAB[iced_x86::Code::Neg_rm8 as usize] = Some(neg_rm8);
    OP_TAB[iced_x86::Code::Not_rm32 as usize] = Some(not_rm32);
    OP_TAB[iced_x86::Code::Not_rm16 as usize] = Some(not_rm16);
    OP_TAB[iced_x86::Code::Not_rm8 as usize] = Some(not_rm8);

    OP_TAB[iced_x86::Code::Lea_r32_m as usize] = Some(lea_r32_m);

    OP_TAB[iced_x86::Code::Cmp_rm32_r32 as usize] = Some(cmp_rm32_r32);
    OP_TAB[iced_x86::Code::Cmp_r32_rm32 as usize] = Some(cmp_r32_rm32);
    OP_TAB[iced_x86::Code::Cmp_EAX_imm32 as usize] = Some(cmp_rm32_imm32);
    OP_TAB[iced_x86::Code::Cmp_rm32_imm32 as usize] = Some(cmp_rm32_imm32);

    OP_TAB[iced_x86::Code::Cmp_rm32_imm8 as usize] = Some(cmp_rm32_imm8);
    OP_TAB[iced_x86::Code::Cmp_rm16_r16 as usize] = Some(cmp_rm16_rm16);
    OP_TAB[iced_x86::Code::Cmp_r16_rm16 as usize] = Some(cmp_rm16_rm16);

    OP_TAB[iced_x86::Code::Cmp_rm16_imm16 as usize] = Some(cmp_rm16_imm16);
    OP_TAB[iced_x86::Code::Cmp_AX_imm16 as usize] = Some(cmp_rm16_imm16);
    OP_TAB[iced_x86::Code::Cmp_rm16_imm8 as usize] = Some(cmp_rm16_imm8);
    OP_TAB[iced_x86::Code::Cmp_rm8_imm8 as usize] = Some(cmp_rm8_imm8);
    OP_TAB[iced_x86::Code::Cmp_AL_imm8 as usize] = Some(cmp_rm8_imm8);

    OP_TAB[iced_x86::Code::Cmp_rm8_r8 as usize] = Some(cmp_rm8_r8);
    OP_TAB[iced_x86::Code::Cmp_r8_rm8 as usize] = Some(cmp_r8_rm8);
    OP_TAB[iced_x86::Code::Test_rm32_r32 as usize] = Some(test_rm32_r32);
    OP_TAB[iced_x86::Code::Test_rm32_imm32 as usize] = Some(test_rm32_imm32);
    OP_TAB[iced_x86::Code::Test_EAX_imm32 as usize] = Some(test_rm32_imm32);

    OP_TAB[iced_x86::Code::Test_rm16_r16 as usize] = Some(test_rm16_r16);
    OP_TAB[iced_x86::Code::Test_rm16_imm16 as usize] = Some(test_rm16_imm16);
    OP_TAB[iced_x86::Code::Test_rm8_r8 as usize] = Some(test_rm8_r8);
    OP_TAB[iced_x86::Code::Test_rm8_imm8 as usize] = Some(test_rm8_imm8);
    OP_TAB[iced_x86::Code::Test_AL_imm8 as usize] = Some(test_rm8_imm8);

    OP_TAB[iced_x86::Code::Bt_rm32_r32 as usize] = Some(bt_rm32_r32);
    OP_TAB[iced_x86::Code::Bt_rm32_imm8 as usize] = Some(bt_rm32_imm8);
    OP_TAB[iced_x86::Code::Btr_rm32_imm8 as usize] = Some(btr_rm32_imm8);
    OP_TAB[iced_x86::Code::Bsr_r32_rm32 as usize] = Some(bsr_r32_rm32);

    OP_TAB[iced_x86::Code::Cmove_r32_rm32 as usize] = Some(cmove_r32_rm32);

    OP_TAB[iced_x86::Code::Seta_rm8 as usize] = Some(seta_rm8);
    OP_TAB[iced_x86::Code::Setae_rm8 as usize] = Some(setae_rm8);
    OP_TAB[iced_x86::Code::Setb_rm8 as usize] = Some(setb_rm8);
    OP_TAB[iced_x86::Code::Setbe_rm8 as usize] = Some(setbe_rm8);
    OP_TAB[iced_x86::Code::Sete_rm8 as usize] = Some(sete_rm8);
    OP_TAB[iced_x86::Code::Setg_rm8 as usize] = Some(setg_rm8);
    OP_TAB[iced_x86::Code::Setl_rm8 as usize] = Some(setl_rm8);
    OP_TAB[iced_x86::Code::Setle_rm8 as usize] = Some(setle_rm8);
    OP_TAB[iced_x86::Code::Setne_rm8 as usize] = Some(setne_rm8);
    OP_TAB[iced_x86::Code::Setge_rm8 as usize] = Some(setge_rm8);

    OP_TAB[iced_x86::Code::Finit as usize] = Some(finit);
    OP_TAB[iced_x86::Code::Fninit as usize] = Some(finit);

    OP_TAB[iced_x86::Code::Fld1 as usize] = Some(fld1);
    OP_TAB[iced_x86::Code::Fldz as usize] = Some(fldz);
    OP_TAB[iced_x86::Code::Fldpi as usize] = Some(fldpi);
    OP_TAB[iced_x86::Code::Fldl2e as usize] = Some(fldl2e);

    OP_TAB[iced_x86::Code::Fld_sti as usize] = Some(fld_sti);
    OP_TAB[iced_x86::Code::Fld_m64fp as usize] = Some(fld_m64fp);
    OP_TAB[iced_x86::Code::Fld_m32fp as usize] = Some(fld_m32fp);
    OP_TAB[iced_x86::Code::Fild_m64int as usize] = Some(fild_m64int);
    OP_TAB[iced_x86::Code::Fild_m32int as usize] = Some(fild_m32int);
    OP_TAB[iced_x86::Code::Fild_m16int as usize] = Some(fild_m16int);
    OP_TAB[iced_x86::Code::Fst_m64fp as usize] = Some(fst_m64fp);
    OP_TAB[iced_x86::Code::Fst_m32fp as usize] = Some(fst_m32fp);
    OP_TAB[iced_x86::Code::Fstp_m64fp as usize] = Some(fstp_m64fp);
    OP_TAB[iced_x86::Code::Fstp_m32fp as usize] = Some(fstp_m32fp);
    OP_TAB[iced_x86::Code::Fstp_sti as usize] = Some(fstp_sti);
    OP_TAB[iced_x86::Code::Fistp_m64int as usize] = Some(fistp_m64int);
    OP_TAB[iced_x86::Code::Fistp_m32int as usize] = Some(fistp_m32int);
    OP_TAB[iced_x86::Code::Fistp_m16int as usize] = Some(fistp_m16int);
    // OP_TAB[iced_x86::Code::Fisp_m64int as usize] = Some(fist_m64int);
    OP_TAB[iced_x86::Code::Fist_m32int as usize] = Some(fist_m32int);
    // OP_TAB[iced_x86::Code::Fist_m16int as usize] = Some(fist_m16int);

    OP_TAB[iced_x86::Code::Fchs as usize] = Some(fchs);
    OP_TAB[iced_x86::Code::Fabs as usize] = Some(fabs);
    OP_TAB[iced_x86::Code::Fcos as usize] = Some(fcos);
    OP_TAB[iced_x86::Code::Fsin as usize] = Some(fsin);
    OP_TAB[iced_x86::Code::Fsincos as usize] = Some(fsincos);
    OP_TAB[iced_x86::Code::Fpatan as usize] = Some(fpatan);
    OP_TAB[iced_x86::Code::Fsqrt as usize] = Some(fsqrt);

    OP_TAB[iced_x86::Code::Fadd_st0_sti as usize] = Some(fadd_sti_sti);
    OP_TAB[iced_x86::Code::Fadd_sti_st0 as usize] = Some(fadd_sti_sti);
    OP_TAB[iced_x86::Code::Fadd_m64fp as usize] = Some(fadd_m64fp);
    OP_TAB[iced_x86::Code::Fadd_m32fp as usize] = Some(fadd_m32fp);
    OP_TAB[iced_x86::Code::Faddp_sti_st0 as usize] = Some(faddp_sti_sti);
    OP_TAB[iced_x86::Code::Fiadd_m32int as usize] = Some(fiadd_m32int);
    OP_TAB[iced_x86::Code::Fiadd_m16int as usize] = Some(fiadd_m16int);

    OP_TAB[iced_x86::Code::Fsub_m64fp as usize] = Some(fsub_m64fp);
    OP_TAB[iced_x86::Code::Fsub_m32fp as usize] = Some(fsub_m32fp);
    OP_TAB[iced_x86::Code::Fsub_st0_sti as usize] = Some(fsub_sti_sti);
    OP_TAB[iced_x86::Code::Fsubp_sti_st0 as usize] = Some(fsubp_sti_sti);
    OP_TAB[iced_x86::Code::Fisub_m32int as usize] = Some(fisub_m32int);

    OP_TAB[iced_x86::Code::Fsubr_m64fp as usize] = Some(fsubr_m64fp);
    OP_TAB[iced_x86::Code::Fsubr_m32fp as usize] = Some(fsubr_m32fp);
    OP_TAB[iced_x86::Code::Fsubr_st0_sti as usize] = Some(fsubr_st0_sti);

    OP_TAB[iced_x86::Code::Fmul_m64fp as usize] = Some(fmul_m64fp);
    OP_TAB[iced_x86::Code::Fmul_m32fp as usize] = Some(fmul_m32fp);
    OP_TAB[iced_x86::Code::Fimul_m32int as usize] = Some(fimul_m32int);
    OP_TAB[iced_x86::Code::Fimul_m16int as usize] = Some(fimul_m16int);
    OP_TAB[iced_x86::Code::Fmul_st0_sti as usize] = Some(fmul_sti_sti);
    OP_TAB[iced_x86::Code::Fmul_sti_st0 as usize] = Some(fmul_sti_sti);
    OP_TAB[iced_x86::Code::Fmulp_sti_st0 as usize] = Some(fmulp_sti_st0);

    OP_TAB[iced_x86::Code::F2xm1 as usize] = Some(f2xm1);
    OP_TAB[iced_x86::Code::Fscale as usize] = Some(fscale);

    OP_TAB[iced_x86::Code::Fdiv_m64fp as usize] = Some(fdiv_m64fp);
    OP_TAB[iced_x86::Code::Fdiv_m32fp as usize] = Some(fdiv_m32fp);
    OP_TAB[iced_x86::Code::Fdiv_sti_st0 as usize] = Some(fdiv_sti_sti);
    OP_TAB[iced_x86::Code::Fdiv_st0_sti as usize] = Some(fdiv_sti_sti);
    OP_TAB[iced_x86::Code::Fdivp_sti_st0 as usize] = Some(fdivp_sti_sti);
    OP_TAB[iced_x86::Code::Fidiv_m32int as usize] = Some(fidiv_m32int);
    OP_TAB[iced_x86::Code::Fidiv_m16int as usize] = Some(fidiv_m16int);

    OP_TAB[iced_x86::Code::Fdivr_m64fp as usize] = Some(fdivr_m64fp);
    OP_TAB[iced_x86::Code::Fdivr_m32fp as usize] = Some(fdivr_m32fp);
    OP_TAB[iced_x86::Code::Fdivr_st0_sti as usize] = Some(fdivr_sti_sti);
    OP_TAB[iced_x86::Code::Fdivrp_sti_st0 as usize] = Some(fdivrp_sti_sti);
    OP_TAB[iced_x86::Code::Fidivr_m32int as usize] = Some(fidivr_m32int);

    OP_TAB[iced_x86::Code::Fprem as usize] = Some(fprem);

    OP_TAB[iced_x86::Code::Fxch_st0_sti as usize] = Some(fxch_st0_sti);
    OP_TAB[iced_x86::Code::Fcom_m32fp as usize] = Some(fcom_m32fp);
    OP_TAB[iced_x86::Code::Fcomp_m32fp as usize] = Some(fcomp_m32fp);
    OP_TAB[iced_x86::Code::Fcomp_m64fp as usize] = Some(fcomp_m64fp);
    OP_TAB[iced_x86::Code::Fcomp_st0_sti as usize] = Some(fcomp_st0_sti);
    OP_TAB[iced_x86::Code::Fucomp_st0_sti as usize] = Some(fucomp_st0_sti);
    OP_TAB[iced_x86::Code::Fcomi_st0_sti as usize] = Some(fcomi_st0_sti);
    OP_TAB[iced_x86::Code::Fucomi_st0_sti as usize] = Some(fucomi_st0_sti);
    OP_TAB[iced_x86::Code::Fucomip_st0_sti as usize] = Some(fucomip_st0_sti);

    OP_TAB[iced_x86::Code::Frndint as usize] = Some(frndint);
    OP_TAB[iced_x86::Code::Fnstsw_AX as usize] = Some(fnstsw_ax);
    OP_TAB[iced_x86::Code::Fnstcw_m2byte as usize] = Some(fnstcw_m2byte);
    OP_TAB[iced_x86::Code::Fldcw_m2byte as usize] = Some(fldcw_m2byte);
    OP_TAB[iced_x86::Code::Fclex as usize] = Some(nop);
    OP_TAB[iced_x86::Code::Fnclex as usize] = Some(nop);

    OP_TAB[iced_x86::Code::Fcmovnbe_st0_sti as usize] = Some(fcmovnbe_st0_sti);

    OP_TAB[iced_x86::Code::Wait as usize] = Some(nop);

    OP_TAB[iced_x86::Code::Pushad as usize] = Some(pushad);
    OP_TAB[iced_x86::Code::Popad as usize] = Some(popad);
    OP_TAB[iced_x86::Code::Pushfd as usize] = Some(pushfd);
    OP_TAB[iced_x86::Code::Pushfw as usize] = Some(pushfw);
    OP_TAB[iced_x86::Code::Popfd as usize] = Some(popfd);
    OP_TAB[iced_x86::Code::Popfw as usize] = Some(popfw);
    OP_TAB[iced_x86::Code::Sahf as usize] = Some(sahf);

    OP_TAB[iced_x86::Code::Salc as usize] = Some(salc);
    OP_TAB[iced_x86::Code::Std as usize] = Some(std);
    OP_TAB[iced_x86::Code::Cld as usize] = Some(cld);
    OP_TAB[iced_x86::Code::Stc as usize] = Some(stc);
    OP_TAB[iced_x86::Code::Clc as usize] = Some(clc);
    OP_TAB[iced_x86::Code::Cmc as usize] = Some(cmc);
    OP_TAB[iced_x86::Code::Cwde as usize] = Some(cwde);
    OP_TAB[iced_x86::Code::Cdq as usize] = Some(cdq);

    OP_TAB[iced_x86::Code::Pxor_mm_mmm64 as usize] = Some(pxor_mm_mmm64);
    OP_TAB[iced_x86::Code::Movq_mm_mmm64 as usize] = Some(movq_mmm64_mmm64);
    OP_TAB[iced_x86::Code::Movq_mmm64_mm as usize] = Some(movq_mmm64_mmm64);
    OP_TAB[iced_x86::Code::Movd_mm_rm32 as usize] = Some(movd_mm_rm32);
    OP_TAB[iced_x86::Code::Movd_rm32_mm as usize] = Some(movd_rm32_mm);
    OP_TAB[iced_x86::Code::Punpcklwd_mm_mmm32 as usize] = Some(punpcklwd_mm_mmm32);
    OP_TAB[iced_x86::Code::Punpcklbw_mm_mmm32 as usize] = Some(punpcklbw_mm_mmm32);
    OP_TAB[iced_x86::Code::Pmullw_mm_mmm64 as usize] = Some(pmullw_mm_mmm64);
    OP_TAB[iced_x86::Code::Pmulhw_mm_mmm64 as usize] = Some(pmulhw_mm_mmm64);
    OP_TAB[iced_x86::Code::Psraw_mm_imm8 as usize] = Some(psraw_mm_imm8);
    OP_TAB[iced_x86::Code::Psrlw_mm_imm8 as usize] = Some(psrlw_mm_imm8);
    OP_TAB[iced_x86::Code::Packuswb_mm_mmm64 as usize] = Some(packuswb_mm_mmm64);
    OP_TAB[iced_x86::Code::Emms as usize] = Some(emms);
    OP_TAB[iced_x86::Code::Psubusb_mm_mmm64 as usize] = Some(psubusb_mm_mmm64);
    OP_TAB[iced_x86::Code::Paddusb_mm_mmm64 as usize] = Some(paddusb_mm_mmm64);
    OP_TAB[iced_x86::Code::Psllw_mm_imm8 as usize] = Some(psllw_mm_imm8);
    OP_TAB[iced_x86::Code::Paddsb_mm_mmm64 as usize] = Some(paddsb_mm_mmm64);
    OP_TAB[iced_x86::Code::Paddw_mm_mmm64 as usize] = Some(paddw_mm_mmm64);
    OP_TAB[iced_x86::Code::Paddsw_mm_mmm64 as usize] = Some(paddsw_mm_mmm64);
    OP_TAB[iced_x86::Code::Psubw_mm_mmm64 as usize] = Some(psubw_mm_mmm64);

    OP_TAB[iced_x86::Code::Nopd as usize] = Some(nop);
    OP_TAB[iced_x86::Code::Nopw as usize] = Some(nop);
    OP_TAB[iced_x86::Code::Nop_rm16 as usize] = Some(nop);
    OP_TAB[iced_x86::Code::Nop_rm32 as usize] = Some(nop);

    OP_TAB[iced_x86::Code::Int3 as usize] = Some(int3);

    OP_TAB[iced_x86::Code::Bswap_r32 as usize] = Some(bswap_r32);
    OP_TAB[iced_x86::Code::Xlat_m8 as usize] = Some(xlat_m8);
    OP_TAB[iced_x86::Code::Bts_rm32_r32 as usize] = Some(bts_rm32_r32);
    OP_TAB[iced_x86::Code::Tzcnt_r32_rm32 as usize] = Some(tzcnt_r32_rm32);

    OP_TAB[iced_x86::Code::Cpuid as usize] = Some(cpuid);

    // Code to print the necessary size of the table:
    // let last = OP_TAB.iter().rposition(|op| op.is_some());
    // log::info!("highest op at {}", last.unwrap());
}

pub fn decode(instr: &Instruction) -> Option<Op> {
    unsafe { OP_TAB[instr.code() as usize] }
}
