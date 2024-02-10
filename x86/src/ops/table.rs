//! Efficiently maps an iced_x86::Code (roughly x86 opcode) to a implementation of the op.

use crate::{ops, x86::CPU};
use iced_x86::Instruction;
use memory::Mem;

/// The type of all operations defined in the ops module.
type Op = fn(&mut CPU, Mem, &Instruction);

// This table is constant and ideally would be initialized at compile time,
// but it's too fiddly to do with const fns, so we'd likely need to codegen it.
static mut OP_TAB: [Option<Op>; 2518] = [None; 2518];

pub unsafe fn init_op_tab() {
    OP_TAB[iced_x86::Code::Enterd_imm16_imm8 as usize] = Some(ops::enterd_imm16_imm8);
    OP_TAB[iced_x86::Code::Leaved as usize] = Some(ops::leaved);
    OP_TAB[iced_x86::Code::Call_rel32_32 as usize] = Some(ops::call);
    OP_TAB[iced_x86::Code::Call_rm32 as usize] = Some(ops::call_rm32);
    OP_TAB[iced_x86::Code::Retnd as usize] = Some(ops::retnd);
    OP_TAB[iced_x86::Code::Retnd_imm16 as usize] = Some(ops::retnd_imm16);
    OP_TAB[iced_x86::Code::Jmp_rel32_32 as usize] = Some(ops::jmp);
    OP_TAB[iced_x86::Code::Jmp_rel8_32 as usize] = Some(ops::jmp);
    OP_TAB[iced_x86::Code::Jmp_rm32 as usize] = Some(ops::jmp_rm32);
    OP_TAB[iced_x86::Code::Ja_rel32_32 as usize] = Some(ops::ja);
    OP_TAB[iced_x86::Code::Ja_rel8_32 as usize] = Some(ops::ja);
    OP_TAB[iced_x86::Code::Jae_rel32_32 as usize] = Some(ops::jae);
    OP_TAB[iced_x86::Code::Jae_rel8_32 as usize] = Some(ops::jae);
    OP_TAB[iced_x86::Code::Jb_rel32_32 as usize] = Some(ops::jb);
    OP_TAB[iced_x86::Code::Jb_rel8_32 as usize] = Some(ops::jb);
    OP_TAB[iced_x86::Code::Jbe_rel32_32 as usize] = Some(ops::jbe);
    OP_TAB[iced_x86::Code::Jbe_rel8_32 as usize] = Some(ops::jbe);
    OP_TAB[iced_x86::Code::Je_rel32_32 as usize] = Some(ops::je);
    OP_TAB[iced_x86::Code::Je_rel8_32 as usize] = Some(ops::je);
    OP_TAB[iced_x86::Code::Jecxz_rel8_32 as usize] = Some(ops::jecxz);
    OP_TAB[iced_x86::Code::Jne_rel32_32 as usize] = Some(ops::jne);
    OP_TAB[iced_x86::Code::Jne_rel8_32 as usize] = Some(ops::jne);
    OP_TAB[iced_x86::Code::Jns_rel32_32 as usize] = Some(ops::jns);
    OP_TAB[iced_x86::Code::Jns_rel8_32 as usize] = Some(ops::jns);
    OP_TAB[iced_x86::Code::Jg_rel32_32 as usize] = Some(ops::jg);
    OP_TAB[iced_x86::Code::Jg_rel8_32 as usize] = Some(ops::jg);
    OP_TAB[iced_x86::Code::Jge_rel32_32 as usize] = Some(ops::jge);
    OP_TAB[iced_x86::Code::Jge_rel8_32 as usize] = Some(ops::jge);
    OP_TAB[iced_x86::Code::Jle_rel32_32 as usize] = Some(ops::jle);
    OP_TAB[iced_x86::Code::Jle_rel8_32 as usize] = Some(ops::jle);
    OP_TAB[iced_x86::Code::Jl_rel32_32 as usize] = Some(ops::jl);
    OP_TAB[iced_x86::Code::Jl_rel8_32 as usize] = Some(ops::jl);
    OP_TAB[iced_x86::Code::Js_rel32_32 as usize] = Some(ops::js);
    OP_TAB[iced_x86::Code::Js_rel8_32 as usize] = Some(ops::js);

    OP_TAB[iced_x86::Code::Loop_rel8_32_ECX as usize] = Some(ops::loop_);

    OP_TAB[iced_x86::Code::Pushd_DS as usize] = Some(ops::pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_ES as usize] = Some(ops::pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_FS as usize] = Some(ops::pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_GS as usize] = Some(ops::pushd_r16);
    OP_TAB[iced_x86::Code::Pushd_imm8 as usize] = Some(ops::pushd_imm8);
    OP_TAB[iced_x86::Code::Pushd_imm32 as usize] = Some(ops::pushd_imm32);
    OP_TAB[iced_x86::Code::Push_r32 as usize] = Some(ops::push_r32);
    OP_TAB[iced_x86::Code::Push_rm32 as usize] = Some(ops::push_rm32);
    OP_TAB[iced_x86::Code::Push_rm16 as usize] = Some(ops::push_rm16);
    OP_TAB[iced_x86::Code::Push_r16 as usize] = Some(ops::push_rm16);

    OP_TAB[iced_x86::Code::Popd_DS as usize] = Some(ops::popd_r16);
    OP_TAB[iced_x86::Code::Popd_ES as usize] = Some(ops::popd_r16);
    OP_TAB[iced_x86::Code::Popd_FS as usize] = Some(ops::popd_r16);
    OP_TAB[iced_x86::Code::Popd_GS as usize] = Some(ops::popd_r16);
    OP_TAB[iced_x86::Code::Pop_r32 as usize] = Some(ops::pop_rm32);
    OP_TAB[iced_x86::Code::Pop_rm32 as usize] = Some(ops::pop_rm32);
    OP_TAB[iced_x86::Code::Pop_r16 as usize] = Some(ops::pop_rm16);
    OP_TAB[iced_x86::Code::Pop_rm16 as usize] = Some(ops::pop_rm16);

    OP_TAB[iced_x86::Code::Mov_rm32_imm32 as usize] = Some(ops::mov_rm32_imm32);
    OP_TAB[iced_x86::Code::Mov_r32_imm32 as usize] = Some(ops::mov_rm32_imm32);
    OP_TAB[iced_x86::Code::Mov_moffs32_EAX as usize] = Some(ops::mov_rm32_r32);
    OP_TAB[iced_x86::Code::Mov_rm32_r32 as usize] = Some(ops::mov_rm32_r32);
    OP_TAB[iced_x86::Code::Mov_r32_rm32 as usize] = Some(ops::mov_r32_rm32);
    OP_TAB[iced_x86::Code::Mov_EAX_moffs32 as usize] = Some(ops::mov_r32_rm32);
    OP_TAB[iced_x86::Code::Mov_r16_rm16 as usize] = Some(ops::mov_r16_rm16);
    OP_TAB[iced_x86::Code::Mov_AX_moffs16 as usize] = Some(ops::mov_r16_rm16);
    OP_TAB[iced_x86::Code::Mov_rm16_r16 as usize] = Some(ops::mov_rm16_r16);
    OP_TAB[iced_x86::Code::Mov_moffs16_AX as usize] = Some(ops::mov_rm16_r16);
    OP_TAB[iced_x86::Code::Mov_rm16_imm16 as usize] = Some(ops::mov_rm16_imm16);
    OP_TAB[iced_x86::Code::Mov_r16_imm16 as usize] = Some(ops::mov_rm16_imm16);
    OP_TAB[iced_x86::Code::Mov_r8_rm8 as usize] = Some(ops::mov_r8_rm8);
    OP_TAB[iced_x86::Code::Mov_AL_moffs8 as usize] = Some(ops::mov_r8_rm8);
    OP_TAB[iced_x86::Code::Mov_rm8_r8 as usize] = Some(ops::mov_rm8_r8);
    OP_TAB[iced_x86::Code::Mov_r8_imm8 as usize] = Some(ops::mov_rm8_imm8);
    OP_TAB[iced_x86::Code::Mov_rm8_imm8 as usize] = Some(ops::mov_rm8_imm8);
    OP_TAB[iced_x86::Code::Mov_moffs8_AL as usize] = Some(ops::mov_moffs8_al);
    OP_TAB[iced_x86::Code::Mov_r32m16_Sreg as usize] = Some(ops::mov_r32m16_sreg);
    OP_TAB[iced_x86::Code::Mov_Sreg_r32m16 as usize] = Some(ops::mov_sreg_r32m16);

    OP_TAB[iced_x86::Code::Movsx_r32_rm16 as usize] = Some(ops::movsx_r32_rm16);
    OP_TAB[iced_x86::Code::Movsx_r32_rm8 as usize] = Some(ops::movsx_r32_rm8);
    OP_TAB[iced_x86::Code::Movsx_r16_rm8 as usize] = Some(ops::movsx_r16_rm8);

    OP_TAB[iced_x86::Code::Movzx_r32_rm16 as usize] = Some(ops::movzx_r32_rm16);
    OP_TAB[iced_x86::Code::Movzx_r32_rm8 as usize] = Some(ops::movzx_r32_rm8);
    OP_TAB[iced_x86::Code::Movzx_r16_rm8 as usize] = Some(ops::movzx_r16_rm8);

    OP_TAB[iced_x86::Code::Cmovb_r32_rm32 as usize] = Some(ops::cmovb_r32_rm32);
    OP_TAB[iced_x86::Code::Cmovne_r32_rm32 as usize] = Some(ops::cmovne_r32_rm32);

    OP_TAB[iced_x86::Code::Xchg_rm32_r32 as usize] = Some(ops::xchg_rm32_r32);
    OP_TAB[iced_x86::Code::Xchg_r32_EAX as usize] = Some(ops::xchg_rm32_r32);
    OP_TAB[iced_x86::Code::Xchg_r16_AX as usize] = Some(ops::xchg_rm16_r16);
    OP_TAB[iced_x86::Code::Xchg_rm8_r8 as usize] = Some(ops::xchg_rm8_r8);

    OP_TAB[iced_x86::Code::Cmpxchg_rm32_r32 as usize] = Some(ops::cmpxchg_rm32_r32);

    OP_TAB[iced_x86::Code::Cmpsb_m8_m8 as usize] = Some(ops::cmps);
    OP_TAB[iced_x86::Code::Movsd_m32_m32 as usize] = Some(ops::movsd);
    OP_TAB[iced_x86::Code::Movsw_m16_m16 as usize] = Some(ops::movsw);
    OP_TAB[iced_x86::Code::Movsb_m8_m8 as usize] = Some(ops::movsb);
    OP_TAB[iced_x86::Code::Scasd_EAX_m32 as usize] = Some(ops::scasd);
    OP_TAB[iced_x86::Code::Scasw_AX_m16 as usize] = Some(ops::scasw);
    OP_TAB[iced_x86::Code::Scasb_AL_m8 as usize] = Some(ops::scasb);
    OP_TAB[iced_x86::Code::Stosd_m32_EAX as usize] = Some(ops::stosd);
    OP_TAB[iced_x86::Code::Stosw_m16_AX as usize] = Some(ops::stosw);
    OP_TAB[iced_x86::Code::Stosb_m8_AL as usize] = Some(ops::stosb);
    OP_TAB[iced_x86::Code::Lodsd_EAX_m32 as usize] = Some(ops::lodsd);
    OP_TAB[iced_x86::Code::Lodsw_AX_m16 as usize] = Some(ops::lodsw);
    OP_TAB[iced_x86::Code::Lodsb_AL_m8 as usize] = Some(ops::lodsb);

    OP_TAB[iced_x86::Code::And_rm32_imm32 as usize] = Some(ops::and_rm32_imm32);
    OP_TAB[iced_x86::Code::And_EAX_imm32 as usize] = Some(ops::and_rm32_imm32);
    OP_TAB[iced_x86::Code::And_rm32_imm8 as usize] = Some(ops::and_rm32_imm8);
    OP_TAB[iced_x86::Code::And_rm32_r32 as usize] = Some(ops::and_rm32_r32);
    OP_TAB[iced_x86::Code::And_r32_rm32 as usize] = Some(ops::and_r32_rm32);
    OP_TAB[iced_x86::Code::And_rm16_imm8 as usize] = Some(ops::and_rm16_imm8);
    OP_TAB[iced_x86::Code::And_rm16_imm16 as usize] = Some(ops::and_rm16_imm16);
    OP_TAB[iced_x86::Code::And_AX_imm16 as usize] = Some(ops::and_rm16_imm16);
    OP_TAB[iced_x86::Code::And_rm8_imm8 as usize] = Some(ops::and_rm8_imm8);
    OP_TAB[iced_x86::Code::And_AL_imm8 as usize] = Some(ops::and_rm8_imm8);

    OP_TAB[iced_x86::Code::Or_rm32_r32 as usize] = Some(ops::or_rm32_rm32);
    OP_TAB[iced_x86::Code::Or_r32_rm32 as usize] = Some(ops::or_rm32_rm32);
    OP_TAB[iced_x86::Code::Or_rm32_imm32 as usize] = Some(ops::or_rm32_imm32);
    OP_TAB[iced_x86::Code::Or_EAX_imm32 as usize] = Some(ops::or_rm32_imm32);
    OP_TAB[iced_x86::Code::Or_rm32_imm8 as usize] = Some(ops::or_rm32_imm8);
    OP_TAB[iced_x86::Code::Or_rm16_imm16 as usize] = Some(ops::or_rm16_imm16);
    OP_TAB[iced_x86::Code::Or_rm8_r8 as usize] = Some(ops::or_rm8_r8);
    OP_TAB[iced_x86::Code::Or_rm8_imm8 as usize] = Some(ops::or_rm8_imm8);
    OP_TAB[iced_x86::Code::Or_r8_rm8 as usize] = Some(ops::or_r8_rm8);
    OP_TAB[iced_x86::Code::Or_AL_imm8 as usize] = Some(ops::or_rm8_imm8);

    OP_TAB[iced_x86::Code::Shl_rm32_imm8 as usize] = Some(ops::shl_rm32_imm8);
    OP_TAB[iced_x86::Code::Shl_rm32_1 as usize] = Some(ops::shl_rm32_imm8);
    OP_TAB[iced_x86::Code::Shl_rm32_CL as usize] = Some(ops::shl_rm32_cl);
    OP_TAB[iced_x86::Code::Shl_rm16_imm8 as usize] = Some(ops::shl_rm16_imm8);
    OP_TAB[iced_x86::Code::Shl_rm16_1 as usize] = Some(ops::shl_rm16_imm8);
    OP_TAB[iced_x86::Code::Shl_rm8_CL as usize] = Some(ops::shl_rm8_cl);
    OP_TAB[iced_x86::Code::Shl_rm8_imm8 as usize] = Some(ops::shl_rm8_imm8);

    OP_TAB[iced_x86::Code::Shr_rm32_CL as usize] = Some(ops::shr_rm32_cl);
    OP_TAB[iced_x86::Code::Shr_rm32_1 as usize] = Some(ops::shr_rm32_1);
    OP_TAB[iced_x86::Code::Shr_rm32_imm8 as usize] = Some(ops::shr_rm32_imm8);
    OP_TAB[iced_x86::Code::Shr_rm16_imm8 as usize] = Some(ops::shr_rm16_imm8);
    OP_TAB[iced_x86::Code::Shr_rm16_1 as usize] = Some(ops::shr_rm16_1);
    OP_TAB[iced_x86::Code::Shr_rm8_imm8 as usize] = Some(ops::shr_rm8_imm8);
    OP_TAB[iced_x86::Code::Shr_rm8_1 as usize] = Some(ops::shr_rm8_imm8);
    OP_TAB[iced_x86::Code::Shrd_rm32_r32_imm8 as usize] = Some(ops::shrd_rm32_r32_imm8);
    OP_TAB[iced_x86::Code::Sar_rm32_imm8 as usize] = Some(ops::sar_rm32_imm8);
    OP_TAB[iced_x86::Code::Sar_rm32_1 as usize] = Some(ops::sar_rm32_imm8);
    OP_TAB[iced_x86::Code::Sar_rm32_CL as usize] = Some(ops::sar_rm32_cl);
    OP_TAB[iced_x86::Code::Sar_rm8_1 as usize] = Some(ops::sar_rm8_imm8);

    OP_TAB[iced_x86::Code::Rol_rm32_imm8 as usize] = Some(ops::rol_rm32_imm8);
    OP_TAB[iced_x86::Code::Rol_rm8_imm8 as usize] = Some(ops::rol_rm8_imm8);
    OP_TAB[iced_x86::Code::Rol_rm8_1 as usize] = Some(ops::rol_rm8_imm8);
    OP_TAB[iced_x86::Code::Ror_rm32_CL as usize] = Some(ops::ror_rm32_cl);
    OP_TAB[iced_x86::Code::Ror_rm32_imm8 as usize] = Some(ops::ror_rm32_imm8);
    OP_TAB[iced_x86::Code::Ror_rm8_imm8 as usize] = Some(ops::ror_rm8_imm8);
    OP_TAB[iced_x86::Code::Ror_rm8_1 as usize] = Some(ops::ror_rm8_imm8);

    OP_TAB[iced_x86::Code::Xor_rm32_r32 as usize] = Some(ops::xor_rm32_rm32);
    OP_TAB[iced_x86::Code::Xor_r32_rm32 as usize] = Some(ops::xor_rm32_rm32);
    OP_TAB[iced_x86::Code::Xor_rm32_imm32 as usize] = Some(ops::xor_rm32_imm32);
    OP_TAB[iced_x86::Code::Xor_EAX_imm32 as usize] = Some(ops::xor_rm32_imm32);
    OP_TAB[iced_x86::Code::Xor_rm32_imm8 as usize] = Some(ops::xor_rm32_imm8);
    OP_TAB[iced_x86::Code::Xor_r16_rm16 as usize] = Some(ops::xor_r16_rm16);
    OP_TAB[iced_x86::Code::Xor_rm8_imm8 as usize] = Some(ops::xor_rm8_imm8);
    OP_TAB[iced_x86::Code::Xor_AL_imm8 as usize] = Some(ops::xor_rm8_imm8);
    OP_TAB[iced_x86::Code::Xor_r8_rm8 as usize] = Some(ops::xor_r8_rm8);
    OP_TAB[iced_x86::Code::Xor_rm8_r8 as usize] = Some(ops::xor_rm8_r8);
    OP_TAB[iced_x86::Code::Add_r32_rm32 as usize] = Some(ops::add_r32_rm32);
    OP_TAB[iced_x86::Code::Add_rm32_r32 as usize] = Some(ops::add_rm32_r32);
    OP_TAB[iced_x86::Code::Add_rm32_imm32 as usize] = Some(ops::add_rm32_imm32);
    OP_TAB[iced_x86::Code::Add_EAX_imm32 as usize] = Some(ops::add_rm32_imm32);
    OP_TAB[iced_x86::Code::Add_rm32_imm8 as usize] = Some(ops::add_rm32_imm8);
    OP_TAB[iced_x86::Code::Add_rm16_imm16 as usize] = Some(ops::add_rm16_imm16);
    OP_TAB[iced_x86::Code::Add_AX_imm16 as usize] = Some(ops::add_rm16_imm16);
    OP_TAB[iced_x86::Code::Add_rm16_imm8 as usize] = Some(ops::add_rm16_imm8);
    OP_TAB[iced_x86::Code::Add_r16_rm16 as usize] = Some(ops::add_r16_rm16);
    OP_TAB[iced_x86::Code::Add_rm8_r8 as usize] = Some(ops::add_rm8_r8);
    OP_TAB[iced_x86::Code::Add_rm8_imm8 as usize] = Some(ops::add_rm8_imm8);
    OP_TAB[iced_x86::Code::Add_AL_imm8 as usize] = Some(ops::add_rm8_imm8);
    OP_TAB[iced_x86::Code::Add_r8_rm8 as usize] = Some(ops::add_r8_rm8);
    OP_TAB[iced_x86::Code::Adc_rm32_r32 as usize] = Some(ops::adc_rm32_rm32);
    OP_TAB[iced_x86::Code::Adc_r32_rm32 as usize] = Some(ops::adc_rm32_rm32);
    OP_TAB[iced_x86::Code::Adc_rm32_imm8 as usize] = Some(ops::adc_rm32_imm8);
    OP_TAB[iced_x86::Code::Adc_rm8_r8 as usize] = Some(ops::adc_rm8_rm8);
    OP_TAB[iced_x86::Code::Adc_r8_rm8 as usize] = Some(ops::adc_rm8_rm8);
    OP_TAB[iced_x86::Code::Adc_rm8_imm8 as usize] = Some(ops::adc_rm8_imm8);
    OP_TAB[iced_x86::Code::Adc_AL_imm8 as usize] = Some(ops::adc_rm8_imm8);
    OP_TAB[iced_x86::Code::Sub_rm32_imm8 as usize] = Some(ops::sub_rm32_imm8);
    OP_TAB[iced_x86::Code::Sub_EAX_imm32 as usize] = Some(ops::sub_rm32_imm32);
    OP_TAB[iced_x86::Code::Sub_rm32_imm32 as usize] = Some(ops::sub_rm32_imm32);
    OP_TAB[iced_x86::Code::Sub_rm32_r32 as usize] = Some(ops::sub_rm32_r32);
    OP_TAB[iced_x86::Code::Sub_r32_rm32 as usize] = Some(ops::sub_r32_rm32);
    OP_TAB[iced_x86::Code::Sub_r16_rm16 as usize] = Some(ops::sub_r16_rm16);
    OP_TAB[iced_x86::Code::Sub_rm16_imm16 as usize] = Some(ops::sub_rm16_imm16);
    OP_TAB[iced_x86::Code::Sub_AX_imm16 as usize] = Some(ops::sub_rm16_imm16);
    OP_TAB[iced_x86::Code::Sub_r8_rm8 as usize] = Some(ops::sub_rm8_rm8);
    OP_TAB[iced_x86::Code::Sub_rm8_r8 as usize] = Some(ops::sub_rm8_rm8);
    OP_TAB[iced_x86::Code::Sub_rm8_imm8 as usize] = Some(ops::sub_rm8_imm8);
    OP_TAB[iced_x86::Code::Sub_AL_imm8 as usize] = Some(ops::sub_rm8_imm8);
    OP_TAB[iced_x86::Code::Sbb_r32_rm32 as usize] = Some(ops::sbb_r32_rm32);
    OP_TAB[iced_x86::Code::Sbb_rm32_r32 as usize] = Some(ops::sbb_rm32_r32);
    OP_TAB[iced_x86::Code::Sbb_rm32_imm8 as usize] = Some(ops::sbb_rm32_imm8);
    OP_TAB[iced_x86::Code::Sbb_r8_rm8 as usize] = Some(ops::sbb_r8_rm8);
    OP_TAB[iced_x86::Code::Sbb_AL_imm8 as usize] = Some(ops::sbb_r8_imm8);
    OP_TAB[iced_x86::Code::Mul_rm32 as usize] = Some(ops::mul_rm32);
    OP_TAB[iced_x86::Code::Imul_rm32 as usize] = Some(ops::imul_rm32);
    OP_TAB[iced_x86::Code::Imul_r32_rm32 as usize] = Some(ops::imul_r32_rm32);
    OP_TAB[iced_x86::Code::Imul_r32_rm32_imm32 as usize] = Some(ops::imul_r32_rm32_imm32);
    OP_TAB[iced_x86::Code::Imul_r32_rm32_imm8 as usize] = Some(ops::imul_r32_rm32_imm8);
    OP_TAB[iced_x86::Code::Imul_rm16 as usize] = Some(ops::imul_rm16);
    OP_TAB[iced_x86::Code::Idiv_rm32 as usize] = Some(ops::idiv_rm32);
    OP_TAB[iced_x86::Code::Div_rm32 as usize] = Some(ops::div_rm32);
    OP_TAB[iced_x86::Code::Dec_r32 as usize] = Some(ops::dec_rm32);
    OP_TAB[iced_x86::Code::Dec_rm32 as usize] = Some(ops::dec_rm32);
    OP_TAB[iced_x86::Code::Dec_r16 as usize] = Some(ops::dec_rm16);
    OP_TAB[iced_x86::Code::Dec_rm8 as usize] = Some(ops::dec_rm8);
    OP_TAB[iced_x86::Code::Inc_r32 as usize] = Some(ops::inc_rm32);
    OP_TAB[iced_x86::Code::Inc_rm32 as usize] = Some(ops::inc_rm32);
    OP_TAB[iced_x86::Code::Inc_rm16 as usize] = Some(ops::inc_rm16);
    OP_TAB[iced_x86::Code::Inc_r16 as usize] = Some(ops::inc_rm16);
    OP_TAB[iced_x86::Code::Inc_rm8 as usize] = Some(ops::inc_rm8);
    OP_TAB[iced_x86::Code::Neg_rm32 as usize] = Some(ops::neg_rm32);
    OP_TAB[iced_x86::Code::Neg_rm16 as usize] = Some(ops::neg_rm16);
    OP_TAB[iced_x86::Code::Neg_rm8 as usize] = Some(ops::neg_rm8);
    OP_TAB[iced_x86::Code::Not_rm32 as usize] = Some(ops::not_rm32);

    OP_TAB[iced_x86::Code::Lea_r32_m as usize] = Some(ops::lea_r32_m);

    OP_TAB[iced_x86::Code::Cmp_rm32_r32 as usize] = Some(ops::cmp_rm32_r32);
    OP_TAB[iced_x86::Code::Cmp_r32_rm32 as usize] = Some(ops::cmp_r32_rm32);
    OP_TAB[iced_x86::Code::Cmp_EAX_imm32 as usize] = Some(ops::cmp_rm32_imm32);
    OP_TAB[iced_x86::Code::Cmp_rm32_imm32 as usize] = Some(ops::cmp_rm32_imm32);

    OP_TAB[iced_x86::Code::Cmp_rm32_imm8 as usize] = Some(ops::cmp_rm32_imm8);
    OP_TAB[iced_x86::Code::Cmp_rm16_r16 as usize] = Some(ops::cmp_rm16_rm16);
    OP_TAB[iced_x86::Code::Cmp_r16_rm16 as usize] = Some(ops::cmp_rm16_rm16);

    OP_TAB[iced_x86::Code::Cmp_rm16_imm16 as usize] = Some(ops::cmp_rm16_imm16);
    OP_TAB[iced_x86::Code::Cmp_AX_imm16 as usize] = Some(ops::cmp_rm16_imm16);
    OP_TAB[iced_x86::Code::Cmp_rm16_imm8 as usize] = Some(ops::cmp_rm16_imm8);
    OP_TAB[iced_x86::Code::Cmp_rm8_imm8 as usize] = Some(ops::cmp_rm8_imm8);
    OP_TAB[iced_x86::Code::Cmp_AL_imm8 as usize] = Some(ops::cmp_rm8_imm8);

    OP_TAB[iced_x86::Code::Cmp_rm8_r8 as usize] = Some(ops::cmp_rm8_r8);
    OP_TAB[iced_x86::Code::Cmp_r8_rm8 as usize] = Some(ops::cmp_r8_rm8);
    OP_TAB[iced_x86::Code::Test_rm32_r32 as usize] = Some(ops::test_rm32_r32);
    OP_TAB[iced_x86::Code::Test_rm32_imm32 as usize] = Some(ops::test_rm32_imm32);
    OP_TAB[iced_x86::Code::Test_EAX_imm32 as usize] = Some(ops::test_rm32_imm32);

    OP_TAB[iced_x86::Code::Test_rm16_r16 as usize] = Some(ops::test_rm16_r16);
    OP_TAB[iced_x86::Code::Test_rm8_r8 as usize] = Some(ops::test_rm8_r8);
    OP_TAB[iced_x86::Code::Test_rm8_imm8 as usize] = Some(ops::test_rm8_imm8);
    OP_TAB[iced_x86::Code::Test_AL_imm8 as usize] = Some(ops::test_rm8_imm8);

    OP_TAB[iced_x86::Code::Bt_rm32_r32 as usize] = Some(ops::bt_rm32_r32);
    OP_TAB[iced_x86::Code::Bt_rm32_imm8 as usize] = Some(ops::bt_rm32_imm8);
    OP_TAB[iced_x86::Code::Btr_rm32_imm8 as usize] = Some(ops::btr_rm32_imm8);
    OP_TAB[iced_x86::Code::Bsr_r32_rm32 as usize] = Some(ops::bsr_r32_rm32);

    OP_TAB[iced_x86::Code::Seta_rm8 as usize] = Some(ops::seta_rm8);
    OP_TAB[iced_x86::Code::Setb_rm8 as usize] = Some(ops::setb_rm8);
    OP_TAB[iced_x86::Code::Sete_rm8 as usize] = Some(ops::sete_rm8);
    OP_TAB[iced_x86::Code::Setl_rm8 as usize] = Some(ops::setl_rm8);
    OP_TAB[iced_x86::Code::Setne_rm8 as usize] = Some(ops::setne_rm8);
    OP_TAB[iced_x86::Code::Setge_rm8 as usize] = Some(ops::setge_rm8);

    OP_TAB[iced_x86::Code::Finit as usize] = Some(ops::finit);
    OP_TAB[iced_x86::Code::Fninit as usize] = Some(ops::finit);
    OP_TAB[iced_x86::Code::Fld1 as usize] = Some(ops::fld1);
    OP_TAB[iced_x86::Code::Fldz as usize] = Some(ops::fldz);
    OP_TAB[iced_x86::Code::Fldpi as usize] = Some(ops::fldpi);
    OP_TAB[iced_x86::Code::Fld_sti as usize] = Some(ops::fld_sti);
    OP_TAB[iced_x86::Code::Fld_m64fp as usize] = Some(ops::fld_m64fp);
    OP_TAB[iced_x86::Code::Fld_m32fp as usize] = Some(ops::fld_m32fp);
    OP_TAB[iced_x86::Code::Fild_m64int as usize] = Some(ops::fild_m64int);
    OP_TAB[iced_x86::Code::Fild_m32int as usize] = Some(ops::fild_m32int);
    OP_TAB[iced_x86::Code::Fild_m16int as usize] = Some(ops::fild_m16int);
    OP_TAB[iced_x86::Code::Fst_m64fp as usize] = Some(ops::fst_m64fp);
    OP_TAB[iced_x86::Code::Fst_m32fp as usize] = Some(ops::fst_m32fp);
    OP_TAB[iced_x86::Code::Fstp_m64fp as usize] = Some(ops::fstp_m64fp);
    OP_TAB[iced_x86::Code::Fstp_m32fp as usize] = Some(ops::fstp_m32fp);
    OP_TAB[iced_x86::Code::Fstp_sti as usize] = Some(ops::fstp_sti);
    OP_TAB[iced_x86::Code::Fistp_m64int as usize] = Some(ops::fistp_m64int);
    OP_TAB[iced_x86::Code::Fistp_m32int as usize] = Some(ops::fistp_m32int);
    OP_TAB[iced_x86::Code::Fistp_m16int as usize] = Some(ops::fistp_m16int);
    OP_TAB[iced_x86::Code::Fchs as usize] = Some(ops::fchs);
    OP_TAB[iced_x86::Code::Fcos as usize] = Some(ops::fcos);
    OP_TAB[iced_x86::Code::Fsin as usize] = Some(ops::fsin);
    OP_TAB[iced_x86::Code::Fsincos as usize] = Some(ops::fsincos);
    OP_TAB[iced_x86::Code::Fpatan as usize] = Some(ops::fpatan);
    OP_TAB[iced_x86::Code::Fsqrt as usize] = Some(ops::fsqrt);

    OP_TAB[iced_x86::Code::Fadd_m64fp as usize] = Some(ops::fadd_m64fp);
    OP_TAB[iced_x86::Code::Fadd_m32fp as usize] = Some(ops::fadd_m32fp);
    OP_TAB[iced_x86::Code::Faddp_sti_st0 as usize] = Some(ops::faddp_sti_st0);
    OP_TAB[iced_x86::Code::Fiadd_m16int as usize] = Some(ops::fiadd_m16int);

    OP_TAB[iced_x86::Code::Fsub_m32fp as usize] = Some(ops::fsub_m32fp);
    OP_TAB[iced_x86::Code::Fsub_st0_sti as usize] = Some(ops::fsub_st0_sti);
    OP_TAB[iced_x86::Code::Fsubr_m64fp as usize] = Some(ops::fsubr_m64fp);
    OP_TAB[iced_x86::Code::Fsubr_m32fp as usize] = Some(ops::fsubr_m32fp);
    OP_TAB[iced_x86::Code::Fsubr_st0_sti as usize] = Some(ops::fsubr_st0_sti);

    OP_TAB[iced_x86::Code::Fmul_m64fp as usize] = Some(ops::fmul_m64fp);
    OP_TAB[iced_x86::Code::Fmul_m32fp as usize] = Some(ops::fmul_m32fp);
    OP_TAB[iced_x86::Code::Fimul_m32int as usize] = Some(ops::fimul_m32int);
    OP_TAB[iced_x86::Code::Fimul_m16int as usize] = Some(ops::fimul_m16int);
    OP_TAB[iced_x86::Code::Fmul_st0_sti as usize] = Some(ops::fmul_sti_sti);
    OP_TAB[iced_x86::Code::Fmul_sti_st0 as usize] = Some(ops::fmul_sti_sti);
    OP_TAB[iced_x86::Code::Fmulp_sti_st0 as usize] = Some(ops::fmulp_sti_st0);

    OP_TAB[iced_x86::Code::Fdiv_m64fp as usize] = Some(ops::fdiv_m64fp);
    OP_TAB[iced_x86::Code::Fdiv_m32fp as usize] = Some(ops::fdiv_m32fp);
    OP_TAB[iced_x86::Code::Fdiv_sti_st0 as usize] = Some(ops::fdiv_sti_st0);
    OP_TAB[iced_x86::Code::Fdivp_sti_st0 as usize] = Some(ops::fdivp_sti_st0);
    OP_TAB[iced_x86::Code::Fdiv_st0_sti as usize] = Some(ops::fdiv_st0_sti);
    OP_TAB[iced_x86::Code::Fidiv_m32int as usize] = Some(ops::fidiv_m32int);

    OP_TAB[iced_x86::Code::Fdivr_m64fp as usize] = Some(ops::fdivr_m64fp);
    OP_TAB[iced_x86::Code::Fdivr_m32fp as usize] = Some(ops::fdivr_m32fp);
    OP_TAB[iced_x86::Code::Fdivr_st0_sti as usize] = Some(ops::fdivr_st0_sti);
    OP_TAB[iced_x86::Code::Fdivrp_sti_st0 as usize] = Some(ops::fdivrp_sti_st0);
    OP_TAB[iced_x86::Code::Fidivr_m32int as usize] = Some(ops::fidivr_m32int);

    OP_TAB[iced_x86::Code::Fxch_st0_sti as usize] = Some(ops::fxch_st0_sti);
    OP_TAB[iced_x86::Code::Fcomp_m32fp as usize] = Some(ops::fcomp_m32fp);
    OP_TAB[iced_x86::Code::Fcomp_m64fp as usize] = Some(ops::fcomp_m64fp);
    OP_TAB[iced_x86::Code::Fucomp_st0_sti as usize] = Some(ops::fucomp_st0_sti);
    OP_TAB[iced_x86::Code::Frndint as usize] = Some(ops::frndint);
    OP_TAB[iced_x86::Code::Fnstsw_AX as usize] = Some(ops::fnstsw_ax);
    OP_TAB[iced_x86::Code::Fnstcw_m2byte as usize] = Some(ops::fnstcw_m2byte);
    OP_TAB[iced_x86::Code::Fldcw_m2byte as usize] = Some(ops::fldcw_m2byte);
    OP_TAB[iced_x86::Code::Fclex as usize] = Some(ops::nop);
    OP_TAB[iced_x86::Code::Fnclex as usize] = Some(ops::nop);
    OP_TAB[iced_x86::Code::Wait as usize] = Some(ops::nop);

    OP_TAB[iced_x86::Code::Pushad as usize] = Some(ops::pushad);
    OP_TAB[iced_x86::Code::Popad as usize] = Some(ops::popad);
    OP_TAB[iced_x86::Code::Pushfd as usize] = Some(ops::pushfd);
    OP_TAB[iced_x86::Code::Pushfw as usize] = Some(ops::pushfw);
    OP_TAB[iced_x86::Code::Popfd as usize] = Some(ops::popfd);
    OP_TAB[iced_x86::Code::Popfw as usize] = Some(ops::popfw);
    OP_TAB[iced_x86::Code::Sahf as usize] = Some(ops::sahf);

    OP_TAB[iced_x86::Code::Salc as usize] = Some(ops::salc);
    OP_TAB[iced_x86::Code::Std as usize] = Some(ops::std);
    OP_TAB[iced_x86::Code::Cld as usize] = Some(ops::cld);
    OP_TAB[iced_x86::Code::Stc as usize] = Some(ops::stc);
    OP_TAB[iced_x86::Code::Cmc as usize] = Some(ops::cmc);
    OP_TAB[iced_x86::Code::Cwde as usize] = Some(ops::cwde);
    OP_TAB[iced_x86::Code::Cdq as usize] = Some(ops::cdq);

    OP_TAB[iced_x86::Code::Pxor_mm_mmm64 as usize] = Some(ops::pxor_mm_mmm64);
    OP_TAB[iced_x86::Code::Movq_mm_mmm64 as usize] = Some(ops::movq_mm_mmm64);
    OP_TAB[iced_x86::Code::Movd_mm_rm32 as usize] = Some(ops::movd_mm_rm32);
    OP_TAB[iced_x86::Code::Movd_rm32_mm as usize] = Some(ops::movd_rm32_mm);
    OP_TAB[iced_x86::Code::Punpcklwd_mm_mmm32 as usize] = Some(ops::punpcklwd_mm_mmm32);
    OP_TAB[iced_x86::Code::Punpcklbw_mm_mmm32 as usize] = Some(ops::punpcklbw_mm_mmm32);
    OP_TAB[iced_x86::Code::Pmullw_mm_mmm64 as usize] = Some(ops::pmullw_mm_mmm64);
    OP_TAB[iced_x86::Code::Psrlw_mm_imm8 as usize] = Some(ops::psrlw_mm_imm8);
    OP_TAB[iced_x86::Code::Packuswb_mm_mmm64 as usize] = Some(ops::packuswb_mm_mmm64);
    OP_TAB[iced_x86::Code::Emms as usize] = Some(ops::emms);
    OP_TAB[iced_x86::Code::Psubusb_mm_mmm64 as usize] = Some(ops::psubusb_mm_mmm64);
    OP_TAB[iced_x86::Code::Paddusb_mm_mmm64 as usize] = Some(ops::paddusb_mm_mmm64);

    OP_TAB[iced_x86::Code::Nopd as usize] = Some(ops::nop);
    OP_TAB[iced_x86::Code::Nop_rm16 as usize] = Some(ops::nop);
    OP_TAB[iced_x86::Code::Nop_rm32 as usize] = Some(ops::nop);

    OP_TAB[iced_x86::Code::Int3 as usize] = Some(ops::int3);

    OP_TAB[iced_x86::Code::Bswap_r32 as usize] = Some(ops::bswap_r32);
    OP_TAB[iced_x86::Code::Xlat_m8 as usize] = Some(ops::xlat_m8);

    // Code to print the necessary size of the table:
    // let last = OP_TAB.iter().rposition(|op| op.is_some());
    // log::info!("highest op at {}", last.unwrap());
}

pub fn execute(cpu: &mut CPU, mem: Mem, instr: &Instruction) {
    match unsafe { OP_TAB[instr.code() as usize] } {
        Some(f) => f(cpu, mem, instr),
        None => cpu.err(format!("no dispatch for: {:?}", instr.code())),
    }
}
