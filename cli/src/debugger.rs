use gdbstub::arch::Arch;
use gdbstub::common::Pid;
use gdbstub::stub::state_machine::GdbStubStateMachine;
use gdbstub::stub::{BaseStopReason, SingleThreadStopReason};
use gdbstub::target::ext::base::singlethread::{
    SingleThreadRangeSteppingOps, SingleThreadSingleStep, SingleThreadSingleStepOps,
};
use gdbstub::target::ext::breakpoints::{
    Breakpoints, BreakpointsOps, SwBreakpoint, SwBreakpointOps,
};
use gdbstub::target::ext::exec_file::{ExecFile, ExecFileOps};
use gdbstub::target::ext::host_io::{
    HostIo, HostIoClose, HostIoCloseOps, HostIoError, HostIoOpen, HostIoOpenFlags, HostIoOpenMode,
    HostIoOpenOps, HostIoOps, HostIoPread, HostIoPreadOps, HostIoPwriteOps, HostIoResult,
};
use gdbstub::target::{Target, TargetError, TargetResult};
use gdbstub::{
    common::Signal,
    stub::GdbStub,
    target::ext::base::{
        single_register_access::{SingleRegisterAccess, SingleRegisterAccessOps},
        singlethread::{SingleThreadBase, SingleThreadResume, SingleThreadResumeOps},
        BaseOps,
    },
};
use std::io::ErrorKind;
use win32::mem::{Extensions, ExtensionsMut};
use win32::Machine;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MachineTargetAction {
    Stop,
    Resume,
    SingleStep,
}

pub struct MachineTarget {
    pub machine: Machine,
    action: Option<MachineTargetAction>,
}

impl MachineTarget {
    pub fn new(machine: Machine) -> Self {
        MachineTarget {
            machine,
            action: None,
        }
    }
}

impl Target for MachineTarget {
    type Arch = gdbstub_arch::x86::X86_SSE;
    type Error = std::io::Error;

    fn base_ops(&mut self) -> BaseOps<'_, Self::Arch, Self::Error> {
        BaseOps::SingleThread(self)
    }

    fn support_breakpoints(&mut self) -> Option<BreakpointsOps<'_, Self>> {
        Some(self)
    }

    fn support_host_io(&mut self) -> Option<HostIoOps<'_, Self>> {
        Some(self)
    }

    fn support_exec_file(&mut self) -> Option<ExecFileOps<'_, Self>> {
        Some(self)
    }
}

#[cfg(feature = "x86-emu")]
impl SingleThreadBase for MachineTarget {
    fn read_registers(
        &mut self,
        regs: &mut <Self::Arch as Arch>::Registers,
    ) -> TargetResult<(), Self> {
        use gdbstub_arch::x86::reg::F80;
        let cpu = self.machine.emu.x86.cpu();
        regs.eax = cpu.regs.get32(x86::Register::EAX);
        regs.ecx = cpu.regs.get32(x86::Register::ECX);
        regs.edx = cpu.regs.get32(x86::Register::EDX);
        regs.ebx = cpu.regs.get32(x86::Register::EBX);
        regs.esp = cpu.regs.get32(x86::Register::ESP);
        regs.ebp = cpu.regs.get32(x86::Register::EBP);
        regs.esi = cpu.regs.get32(x86::Register::ESI);
        regs.edi = cpu.regs.get32(x86::Register::EDI);
        regs.eip = cpu.regs.eip;
        regs.eflags = cpu.flags.bits();
        regs.segments.cs = 0; // TODO
        regs.segments.ss = 0; // TODO
        regs.segments.ds = 0; // TODO
        regs.segments.es = 0; // TODO
        regs.segments.fs = 0; // TODO
        regs.segments.gs = 0; // TODO
        regs.st.fill(F80::default()); // TODO
        regs.xmm.fill(0u128); // TODO
        regs.mxcsr = 0; // TODO
        Ok(())
    }

    fn write_registers(
        &mut self,
        regs: &<Self::Arch as Arch>::Registers,
    ) -> TargetResult<(), Self> {
        let cpu = self.machine.emu.x86.cpu_mut();
        cpu.regs.set32(x86::Register::EAX, regs.eax);
        cpu.regs.set32(x86::Register::ECX, regs.ecx);
        cpu.regs.set32(x86::Register::EDX, regs.edx);
        cpu.regs.set32(x86::Register::EBX, regs.ebx);
        cpu.regs.set32(x86::Register::ESP, regs.esp);
        cpu.regs.set32(x86::Register::EBP, regs.ebp);
        cpu.regs.set32(x86::Register::ESI, regs.esi);
        cpu.regs.set32(x86::Register::EDI, regs.edi);
        cpu.regs.eip = regs.eip;
        cpu.flags = x86::Flags::from_bits_truncate(regs.eflags);
        // TODO: segments, st, xmm, mxcsr
        Ok(())
    }

    fn support_single_register_access(&mut self) -> Option<SingleRegisterAccessOps<'_, (), Self>> {
        Some(self)
    }

    fn read_addrs(
        &mut self,
        start_addr: <Self::Arch as Arch>::Usize,
        data: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let mem = self.machine.mem();
        let slice = if start_addr > mem.len() {
            return Ok(0);
        } else if start_addr + data.len() as u32 > mem.len() {
            mem.sub32(start_addr, mem.len() - start_addr)
        } else {
            mem.sub32(start_addr, data.len() as u32)
        };
        data[..slice.len()].copy_from_slice(slice);
        Ok(slice.len())
    }

    fn write_addrs(
        &mut self,
        start_addr: <Self::Arch as Arch>::Usize,
        data: &[u8],
    ) -> TargetResult<(), Self> {
        self.machine
            .mem()
            .sub32_mut(start_addr, data.len() as u32)
            .copy_from_slice(data);
        Ok(())
    }

    fn support_resume(&mut self) -> Option<SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }
}

#[cfg(feature = "x86-emu")]
impl SingleRegisterAccess<()> for MachineTarget {
    fn read_register(
        &mut self,
        _tid: (),
        reg_id: <Self::Arch as Arch>::RegId,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        use gdbstub_arch::x86::reg::id::X86CoreRegId;
        let cpu = self.machine.emu.x86.cpu();
        fn reg_read_u32(
            cpu: &x86::CPU,
            buf: &mut [u8],
            reg: x86::Register,
        ) -> TargetResult<usize, MachineTarget> {
            let value = cpu.regs.get32(reg);
            buf.copy_from_slice(&value.to_le_bytes());
            Ok(4)
        }
        match reg_id {
            X86CoreRegId::Eax => reg_read_u32(cpu, buf, x86::Register::EAX),
            X86CoreRegId::Ecx => reg_read_u32(cpu, buf, x86::Register::ECX),
            X86CoreRegId::Edx => reg_read_u32(cpu, buf, x86::Register::EDX),
            X86CoreRegId::Ebx => reg_read_u32(cpu, buf, x86::Register::EBX),
            X86CoreRegId::Esp => reg_read_u32(cpu, buf, x86::Register::ESP),
            X86CoreRegId::Ebp => reg_read_u32(cpu, buf, x86::Register::EBP),
            X86CoreRegId::Esi => reg_read_u32(cpu, buf, x86::Register::ESI),
            X86CoreRegId::Edi => reg_read_u32(cpu, buf, x86::Register::EDI),
            X86CoreRegId::Eip => {
                buf.copy_from_slice(&cpu.regs.eip.to_le_bytes());
                Ok(4)
            }
            X86CoreRegId::Eflags => {
                buf.copy_from_slice(&cpu.flags.bits().to_le_bytes());
                Ok(4)
            }
            X86CoreRegId::Segment(_reg_id) => {
                // TODO
                buf.copy_from_slice(&0u32.to_le_bytes());
                Ok(4)
            }
            X86CoreRegId::St(idx) => {
                let _value = cpu.fpu.st[idx as usize];
                // TODO convert to f80
                buf.copy_from_slice(&[0u8; 10]);
                Ok(10)
            }
            X86CoreRegId::Fpu(_reg_id) => {
                // TODO
                buf.copy_from_slice(&0u32.to_le_bytes());
                Ok(4)
            }
            X86CoreRegId::Xmm(_idx) => {
                // TODO
                buf.copy_from_slice(&0u128.to_le_bytes());
                Ok(16)
            }
            X86CoreRegId::Mxcsr => {
                // TODO
                buf.copy_from_slice(&0u32.to_le_bytes());
                Ok(4)
            }
            _ => {
                log::warn!("read_register: unsupported register {:?}", reg_id);
                Ok(0)
            }
        }
    }

    fn write_register(
        &mut self,
        _tid: (),
        reg_id: <Self::Arch as Arch>::RegId,
        val: &[u8],
    ) -> TargetResult<(), Self> {
        use gdbstub_arch::x86::reg::id::X86CoreRegId;
        let cpu = self.machine.emu.x86.cpu_mut();
        fn reg_write_u32(
            cpu: &mut x86::CPU,
            val: &[u8],
            reg: x86::Register,
        ) -> TargetResult<(), MachineTarget> {
            let value = u32::from_le_bytes(val.try_into().unwrap());
            cpu.regs.set32(reg, value);
            Ok(())
        }
        match reg_id {
            X86CoreRegId::Eax => reg_write_u32(cpu, val, x86::Register::EAX),
            X86CoreRegId::Ecx => reg_write_u32(cpu, val, x86::Register::ECX),
            X86CoreRegId::Edx => reg_write_u32(cpu, val, x86::Register::EDX),
            X86CoreRegId::Ebx => reg_write_u32(cpu, val, x86::Register::EBX),
            X86CoreRegId::Esp => reg_write_u32(cpu, val, x86::Register::ESP),
            X86CoreRegId::Ebp => reg_write_u32(cpu, val, x86::Register::EBP),
            X86CoreRegId::Esi => reg_write_u32(cpu, val, x86::Register::ESI),
            X86CoreRegId::Edi => reg_write_u32(cpu, val, x86::Register::EDI),
            X86CoreRegId::Eip => {
                cpu.regs.eip = u32::from_le_bytes(val.try_into().unwrap());
                Ok(())
            }
            X86CoreRegId::Eflags => {
                cpu.flags =
                    x86::Flags::from_bits_truncate(u32::from_le_bytes(val.try_into().unwrap()));
                Ok(())
            }
            X86CoreRegId::Segment(_reg_id) => {
                // TODO
                Ok(())
            }
            X86CoreRegId::St(_idx) => {
                // TODO
                Ok(())
            }
            X86CoreRegId::Fpu(_reg_id) => {
                // TODO
                Ok(())
            }
            X86CoreRegId::Xmm(_idx) => {
                // TODO
                Ok(())
            }
            X86CoreRegId::Mxcsr => {
                // TODO
                Ok(())
            }
            _ => {
                log::warn!("write_register: unsupported register {:?}", reg_id);
                Ok(())
            }
        }
    }
}

#[cfg(feature = "x86-unicorn")]
const ST_REGS: [unicorn_engine::RegisterX86; 8] = [
    unicorn_engine::RegisterX86::ST0,
    unicorn_engine::RegisterX86::ST1,
    unicorn_engine::RegisterX86::ST2,
    unicorn_engine::RegisterX86::ST3,
    unicorn_engine::RegisterX86::ST4,
    unicorn_engine::RegisterX86::ST5,
    unicorn_engine::RegisterX86::ST6,
    unicorn_engine::RegisterX86::ST7,
];

#[cfg(feature = "x86-unicorn")]
const XMM_REGS: [unicorn_engine::RegisterX86; 8] = [
    unicorn_engine::RegisterX86::XMM0,
    unicorn_engine::RegisterX86::XMM1,
    unicorn_engine::RegisterX86::XMM2,
    unicorn_engine::RegisterX86::XMM3,
    unicorn_engine::RegisterX86::XMM4,
    unicorn_engine::RegisterX86::XMM5,
    unicorn_engine::RegisterX86::XMM6,
    unicorn_engine::RegisterX86::XMM7,
];

#[cfg(feature = "x86-unicorn")]
impl SingleThreadBase for MachineTarget {
    fn read_registers(
        &mut self,
        regs: &mut <Self::Arch as Arch>::Registers,
    ) -> TargetResult<(), Self> {
        let cpu = &self.machine.emu.unicorn;
        regs.eax = cpu.reg_read(unicorn_engine::RegisterX86::EAX).unwrap() as u32;
        regs.ecx = cpu.reg_read(unicorn_engine::RegisterX86::ECX).unwrap() as u32;
        regs.edx = cpu.reg_read(unicorn_engine::RegisterX86::EDX).unwrap() as u32;
        regs.ebx = cpu.reg_read(unicorn_engine::RegisterX86::EBX).unwrap() as u32;
        regs.esp = cpu.reg_read(unicorn_engine::RegisterX86::ESP).unwrap() as u32;
        regs.ebp = cpu.reg_read(unicorn_engine::RegisterX86::EBP).unwrap() as u32;
        regs.esi = cpu.reg_read(unicorn_engine::RegisterX86::ESI).unwrap() as u32;
        regs.edi = cpu.reg_read(unicorn_engine::RegisterX86::EDI).unwrap() as u32;
        regs.eip = cpu.reg_read(unicorn_engine::RegisterX86::EIP).unwrap() as u32;
        regs.eflags = cpu.reg_read(unicorn_engine::RegisterX86::EFLAGS).unwrap() as u32;
        regs.segments.cs = cpu.reg_read(unicorn_engine::RegisterX86::CS).unwrap() as u32;
        regs.segments.ss = cpu.reg_read(unicorn_engine::RegisterX86::SS).unwrap() as u32;
        regs.segments.ds = cpu.reg_read(unicorn_engine::RegisterX86::DS).unwrap() as u32;
        regs.segments.es = cpu.reg_read(unicorn_engine::RegisterX86::ES).unwrap() as u32;
        regs.segments.fs = cpu.reg_read(unicorn_engine::RegisterX86::FS).unwrap() as u32;
        regs.segments.gs = cpu.reg_read(unicorn_engine::RegisterX86::GS).unwrap() as u32;
        for (idx, reg) in ST_REGS.iter().cloned().enumerate() {
            regs.st[idx] = cpu.reg_read_long(reg).unwrap().as_ref().try_into().unwrap();
        }
        // TODO regs.fpu
        for (idx, reg) in XMM_REGS.iter().cloned().enumerate() {
            regs.xmm[idx] =
                u128::from_le_bytes(cpu.reg_read_long(reg).unwrap().as_ref().try_into().unwrap());
        }
        regs.mxcsr = cpu.reg_read(unicorn_engine::RegisterX86::MXCSR).unwrap() as u32;
        Ok(())
    }

    fn write_registers(
        &mut self,
        regs: &<Self::Arch as Arch>::Registers,
    ) -> TargetResult<(), Self> {
        let cpu = &mut self.machine.emu.unicorn;
        cpu.reg_write(unicorn_engine::RegisterX86::EAX, regs.eax as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::ECX, regs.ecx as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::EDX, regs.edx as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::EBX, regs.ebx as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::ESP, regs.esp as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::EBP, regs.ebp as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::ESI, regs.esi as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::EDI, regs.edi as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::EIP, regs.eip as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::EFLAGS, regs.eflags as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::CS, regs.segments.cs as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::SS, regs.segments.ss as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::DS, regs.segments.ds as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::ES, regs.segments.es as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::FS, regs.segments.fs as u64)
            .unwrap();
        cpu.reg_write(unicorn_engine::RegisterX86::GS, regs.segments.gs as u64)
            .unwrap();
        for (idx, reg) in ST_REGS.iter().cloned().enumerate() {
            cpu.reg_write_long(reg, &regs.st[idx]).unwrap();
        }
        // TODO regs.fpu
        for (idx, reg) in XMM_REGS.iter().cloned().enumerate() {
            cpu.reg_write_long(reg, &regs.xmm[idx].to_le_bytes())
                .unwrap();
        }
        cpu.reg_write(unicorn_engine::RegisterX86::MXCSR, regs.mxcsr as u64)
            .unwrap();
        Ok(())
    }

    fn support_single_register_access(&mut self) -> Option<SingleRegisterAccessOps<'_, (), Self>> {
        Some(self)
    }

    fn read_addrs(
        &mut self,
        start_addr: <Self::Arch as Arch>::Usize,
        data: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let mem = self.machine.mem();
        let slice = if start_addr > mem.len() {
            return Ok(0);
        } else if start_addr + data.len() as u32 > mem.len() {
            mem.sub32(start_addr, mem.len() - start_addr)
        } else {
            mem.sub32(start_addr, data.len() as u32)
        };
        data[..slice.len()].copy_from_slice(slice);
        Ok(slice.len())
    }

    fn write_addrs(
        &mut self,
        start_addr: <Self::Arch as Arch>::Usize,
        data: &[u8],
    ) -> TargetResult<(), Self> {
        self.machine
            .mem()
            .sub32_mut(start_addr, data.len() as u32)
            .copy_from_slice(data);
        Ok(())
    }

    fn support_resume(&mut self) -> Option<SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }
}

#[cfg(feature = "x86-unicorn")]
impl SingleRegisterAccess<()> for MachineTarget {
    fn read_register(
        &mut self,
        _tid: (),
        reg_id: <Self::Arch as Arch>::RegId,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        use gdbstub_arch::x86::reg::id::{X86CoreRegId, X86SegmentRegId};
        let cpu = &self.machine.emu.unicorn;
        fn reg_read_u32(
            cpu: &unicorn_engine::Unicorn<()>,
            buf: &mut [u8],
            reg: unicorn_engine::RegisterX86,
        ) -> TargetResult<usize, MachineTarget> {
            let value = cpu.reg_read(reg).unwrap() as u32;
            buf.copy_from_slice(&value.to_le_bytes());
            Ok(4)
        }
        match reg_id {
            X86CoreRegId::Eax => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EAX),
            X86CoreRegId::Ecx => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::ECX),
            X86CoreRegId::Edx => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EDX),
            X86CoreRegId::Ebx => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EBX),
            X86CoreRegId::Esp => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::ESP),
            X86CoreRegId::Ebp => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EBP),
            X86CoreRegId::Esi => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::ESI),
            X86CoreRegId::Edi => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EDI),
            X86CoreRegId::Eip => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EIP),
            X86CoreRegId::Eflags => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::EFLAGS),
            X86CoreRegId::Segment(reg_id) => reg_read_u32(
                cpu,
                buf,
                match reg_id {
                    X86SegmentRegId::CS => unicorn_engine::RegisterX86::CS,
                    X86SegmentRegId::SS => unicorn_engine::RegisterX86::SS,
                    X86SegmentRegId::DS => unicorn_engine::RegisterX86::DS,
                    X86SegmentRegId::ES => unicorn_engine::RegisterX86::ES,
                    X86SegmentRegId::FS => unicorn_engine::RegisterX86::FS,
                    X86SegmentRegId::GS => unicorn_engine::RegisterX86::GS,
                },
            ),
            X86CoreRegId::St(idx) => {
                let value = cpu.reg_read_long(ST_REGS[idx as usize]).unwrap();
                buf.copy_from_slice(value.as_ref());
                Ok(10)
            }
            X86CoreRegId::Fpu(_reg_id) => {
                // TODO
                buf.copy_from_slice(&0u32.to_le_bytes());
                Ok(4)
            }
            X86CoreRegId::Xmm(idx) => {
                let value = cpu.reg_read_long(XMM_REGS[idx as usize]).unwrap();
                buf.copy_from_slice(value.as_ref());
                Ok(16)
            }
            X86CoreRegId::Mxcsr => reg_read_u32(cpu, buf, unicorn_engine::RegisterX86::MXCSR),
            _ => {
                log::warn!("read_register: unsupported register {:?}", reg_id);
                Ok(0)
            }
        }
    }

    fn write_register(
        &mut self,
        _tid: (),
        reg_id: <Self::Arch as Arch>::RegId,
        val: &[u8],
    ) -> TargetResult<(), Self> {
        use gdbstub_arch::x86::reg::id::{X86CoreRegId, X86SegmentRegId};
        let cpu = &mut self.machine.emu.unicorn;
        fn reg_write_u32(
            cpu: &mut unicorn_engine::Unicorn<()>,
            reg: unicorn_engine::RegisterX86,
            val: &[u8],
        ) -> TargetResult<(), MachineTarget> {
            let value = u32::from_le_bytes(val.try_into().unwrap());
            cpu.reg_write(reg, value as u64).unwrap();
            Ok(())
        }
        match reg_id {
            X86CoreRegId::Eax => reg_write_u32(cpu, unicorn_engine::RegisterX86::EAX, val),
            X86CoreRegId::Ecx => reg_write_u32(cpu, unicorn_engine::RegisterX86::ECX, val),
            X86CoreRegId::Edx => reg_write_u32(cpu, unicorn_engine::RegisterX86::EDX, val),
            X86CoreRegId::Ebx => reg_write_u32(cpu, unicorn_engine::RegisterX86::EBX, val),
            X86CoreRegId::Esp => reg_write_u32(cpu, unicorn_engine::RegisterX86::ESP, val),
            X86CoreRegId::Ebp => reg_write_u32(cpu, unicorn_engine::RegisterX86::EBP, val),
            X86CoreRegId::Esi => reg_write_u32(cpu, unicorn_engine::RegisterX86::ESI, val),
            X86CoreRegId::Edi => reg_write_u32(cpu, unicorn_engine::RegisterX86::EDI, val),
            X86CoreRegId::Eip => reg_write_u32(cpu, unicorn_engine::RegisterX86::EIP, val),
            X86CoreRegId::Eflags => reg_write_u32(cpu, unicorn_engine::RegisterX86::EFLAGS, val),
            X86CoreRegId::Segment(reg_id) => reg_write_u32(
                cpu,
                match reg_id {
                    X86SegmentRegId::CS => unicorn_engine::RegisterX86::CS,
                    X86SegmentRegId::SS => unicorn_engine::RegisterX86::SS,
                    X86SegmentRegId::DS => unicorn_engine::RegisterX86::DS,
                    X86SegmentRegId::ES => unicorn_engine::RegisterX86::ES,
                    X86SegmentRegId::FS => unicorn_engine::RegisterX86::FS,
                    X86SegmentRegId::GS => unicorn_engine::RegisterX86::GS,
                },
                val,
            ),
            X86CoreRegId::St(idx) => {
                cpu.reg_write_long(ST_REGS[idx as usize], val).unwrap();
                Ok(())
            }
            X86CoreRegId::Fpu(_reg_id) => {
                // TODO
                Ok(())
            }
            X86CoreRegId::Xmm(idx) => {
                cpu.reg_write_long(XMM_REGS[idx as usize], val).unwrap();
                Ok(())
            }
            X86CoreRegId::Mxcsr => reg_write_u32(cpu, unicorn_engine::RegisterX86::MXCSR, val),
            _ => {
                log::warn!("write_register: unsupported register {:?}", reg_id);
                Ok(())
            }
        }
    }
}

impl Breakpoints for MachineTarget {
    fn support_sw_breakpoint(&mut self) -> Option<SwBreakpointOps<'_, Self>> {
        Some(self)
    }
}

impl SwBreakpoint for MachineTarget {
    fn add_sw_breakpoint(
        &mut self,
        addr: <Self::Arch as Arch>::Usize,
        kind: <Self::Arch as Arch>::BreakpointKind,
    ) -> TargetResult<bool, Self> {
        log::info!("add_sw_breakpoint: {:#x} {}", addr, kind);
        Ok(self.machine.add_breakpoint(addr))
    }

    fn remove_sw_breakpoint(
        &mut self,
        addr: <Self::Arch as Arch>::Usize,
        kind: <Self::Arch as Arch>::BreakpointKind,
    ) -> TargetResult<bool, Self> {
        log::info!("remove_sw_breakpoint: {:#x} {}", addr, kind);
        Ok(self.machine.clear_breakpoint(addr))
    }
}

impl ExecFile for MachineTarget {
    fn get_exec_file(
        &self,
        pid: Option<Pid>,
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        if !matches!(pid, None | Some(Pid::MIN)) {
            return Err(TargetError::Io(std::io::Error::new(
                ErrorKind::Other,
                "get_exec_file: pid is not supported",
            )));
        }
        let path = self.machine.exe_path.as_os_str().as_encoded_bytes();
        if offset >= path.len() as u64 {
            return Ok(0);
        }
        let end = (offset + length as u64).min(path.len() as u64);
        let slice = &path[offset as usize..end as usize];
        buf[..slice.len()].copy_from_slice(slice);
        Ok(slice.len())
    }
}

impl HostIo for MachineTarget {
    fn support_open(&mut self) -> Option<HostIoOpenOps<'_, Self>> {
        Some(self)
    }

    fn support_close(&mut self) -> Option<HostIoCloseOps<'_, Self>> {
        Some(self)
    }

    fn support_pread(&mut self) -> Option<HostIoPreadOps<'_, Self>> {
        Some(self)
    }

    fn support_pwrite(&mut self) -> Option<HostIoPwriteOps<'_, Self>> {
        // omitting write support for now
        None
    }
}

impl HostIoOpen for MachineTarget {
    fn open(
        &mut self,
        filename: &[u8],
        flags: HostIoOpenFlags,
        mode: HostIoOpenMode,
    ) -> HostIoResult<u32, Self> {
        let path = String::from_utf8_lossy(filename);
        let mut options = std::fs::OpenOptions::new();
        let rw = flags.contains(HostIoOpenFlags::O_RDWR);
        options
            .read(rw || flags.contains(HostIoOpenFlags::O_RDONLY))
            .write(rw || flags.contains(HostIoOpenFlags::O_WRONLY))
            .create(flags.contains(HostIoOpenFlags::O_CREAT))
            .append(flags.contains(HostIoOpenFlags::O_APPEND))
            .truncate(flags.contains(HostIoOpenFlags::O_TRUNC));
        let _ = mode; // TODO
        let file = match options.open::<&str>(path.as_ref()) {
            Ok(file) => file,
            Err(err) => {
                return Err(HostIoError::from(err));
            }
        };
        // TODO windows
        use std::os::fd::IntoRawFd;
        let fd = file.into_raw_fd() as u32;
        Ok(fd)
    }
}

impl HostIoClose for MachineTarget {
    fn close(&mut self, fd: u32) -> HostIoResult<(), Self> {
        // TODO windows
        use std::os::unix::io::FromRawFd;
        let file = unsafe { std::fs::File::from_raw_fd(fd as i32) };
        file.sync_all().map_err(|e| HostIoError::from(e))
    }
}

impl HostIoPread for MachineTarget {
    fn pread(
        &mut self,
        fd: u32,
        count: usize,
        offset: u64,
        buf: &mut [u8],
    ) -> HostIoResult<usize, Self> {
        // TODO windows
        use std::os::unix::{fs::FileExt, io::FromRawFd};
        let file = unsafe { std::fs::File::from_raw_fd(fd as i32) };
        let end = count.min(buf.len());
        file.read_at(&mut buf[..end], offset)
            .map_err(|e| HostIoError::from(e))
    }
}

impl SingleThreadResume for MachineTarget {
    fn resume(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        let _ = signal;
        self.action = Some(MachineTargetAction::Resume);
        Ok(())
    }

    fn support_single_step(&mut self) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }

    fn support_range_step(&mut self) -> Option<SingleThreadRangeSteppingOps<'_, Self>> {
        None // TODO
    }
}

impl SingleThreadSingleStep for MachineTarget {
    fn step(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        let _ = signal;
        self.action = Some(MachineTargetAction::SingleStep);
        Ok(())
    }
}

fn wait_for_gdb_connection(port: u16) -> std::io::Result<std::net::TcpStream> {
    let sockaddr = format!("localhost:{}", port);
    log::info!("Waiting for a GDB connection on {:?}...", sockaddr);
    let sock = std::net::TcpListener::bind(sockaddr)?;
    let (stream, addr) = sock.accept()?;
    log::info!("Debugger connected from {}", addr);
    Ok(stream)
}

pub type StateMachine<'a> = GdbStubStateMachine<'a, MachineTarget, std::net::TcpStream>;

pub fn run<'a>(target: &mut MachineTarget) -> anyhow::Result<StateMachine<'a>> {
    let connection: std::net::TcpStream = wait_for_gdb_connection(9001)?;
    let debugger = GdbStub::new(connection);
    Ok(debugger.run_state_machine(target)?)
}

pub trait DebuggerExt {
    /// Poll the socket for incoming data and process it, optionally returning an action.
    fn poll(&mut self, target: &mut MachineTarget) -> anyhow::Result<Option<MachineTargetAction>>;
    /// Notify the debugger that the machine has completed a single or range step.
    fn done_step(&mut self, target: &mut MachineTarget) -> anyhow::Result<()>;
    /// Notify the debugger that the machine has encountered an error.
    fn machine_error(&mut self, target: &mut MachineTarget, signal: u8) -> anyhow::Result<()>;
    /// Notify the debugger that the machine has hit a breakpoint.
    fn breakpoint(&mut self, target: &mut MachineTarget) -> anyhow::Result<()>;
    /// Notify the debugger that the machine is currently stopped.
    /// Will block until new data is available to avoid busy-waiting.
    fn stopped(&mut self, target: &mut MachineTarget) -> anyhow::Result<()>;
}

impl DebuggerExt for Option<StateMachine<'_>> {
    fn poll(&mut self, target: &mut MachineTarget) -> anyhow::Result<Option<MachineTargetAction>> {
        if let Some(state) = self.take() {
            *self = match state {
                GdbStubStateMachine::Idle(mut inner) => {
                    use gdbstub::conn::ConnectionExt;
                    let conn = inner.borrow_conn();
                    Some(if conn.peek()?.is_some() {
                        let byte = conn.read()?;
                        inner.incoming_data(target, byte)?
                    } else {
                        GdbStubStateMachine::Idle(inner)
                    })
                }
                GdbStubStateMachine::Running(mut inner) => {
                    use gdbstub::conn::ConnectionExt;
                    let conn = inner.borrow_conn();
                    Some(if conn.peek()?.is_some() {
                        let byte = conn.read()?;
                        inner.incoming_data(target, byte)?
                    } else {
                        GdbStubStateMachine::Running(inner)
                    })
                }
                GdbStubStateMachine::CtrlCInterrupt(inner) => {
                    target.action = Some(MachineTargetAction::Stop);
                    Some(
                        inner
                            .interrupt_handled(target, Some(SingleThreadStopReason::SwBreak(())))?,
                    )
                }
                GdbStubStateMachine::Disconnected(_inner) => {
                    log::info!("debugger disconnected");
                    None
                }
            };
        }
        Ok(target.action.take())
    }

    fn done_step(&mut self, target: &mut MachineTarget) -> anyhow::Result<()> {
        if let Some(state) = self.take() {
            *self = match state {
                GdbStubStateMachine::Running(inner) => {
                    Some(inner.report_stop(target, BaseStopReason::<(), u32>::DoneStep)?)
                }
                state => Some(state),
            }
        }
        Ok(())
    }

    fn machine_error(&mut self, target: &mut MachineTarget, signal: u8) -> anyhow::Result<()> {
        if let Some(state) = self.take() {
            *self = match state {
                GdbStubStateMachine::Running(inner) => Some(
                    inner.report_stop(target, BaseStopReason::<(), u32>::Signal(Signal(signal)))?,
                ),
                state => Some(state),
            }
        }
        Ok(())
    }

    fn breakpoint(&mut self, target: &mut MachineTarget) -> anyhow::Result<()> {
        if let Some(state) = self.take() {
            *self = match state {
                GdbStubStateMachine::Running(inner) => {
                    Some(inner.report_stop(target, BaseStopReason::SwBreak(()))?)
                }
                state => Some(state),
            }
        }
        Ok(())
    }

    fn stopped(&mut self, target: &mut MachineTarget) -> anyhow::Result<()> {
        if let Some(state) = self.take() {
            *self = match state {
                GdbStubStateMachine::Running(inner) => {
                    Some(inner.report_stop(target, BaseStopReason::<(), u32>::DoneStep)?)
                }
                GdbStubStateMachine::Idle(mut inner) => {
                    // Both program and debugger are stopped. Read a byte synchronously
                    // from the connection to avoid busy-waiting.
                    use gdbstub::conn::ConnectionExt;
                    let byte = inner.borrow_conn().read()?;
                    Some(inner.incoming_data(target, byte)?)
                }
                state => Some(state),
            }
        }
        Ok(())
    }
}
