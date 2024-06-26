use gdbstub::{
    common::Signal,
    conn::{Connection, ConnectionExt},
    stub::{run_blocking, DisconnectReason, GdbStub, SingleThreadStopReason},
    target::{
        ext::base::{
            singlethread::{SingleThreadBase, SingleThreadResume, SingleThreadResumeOps},
            BaseOps,
        },
        Target,
    },
};
use win32::Machine;

struct MachineTarget {
    machine: Machine,
}

impl MachineTarget {
    fn new(machine: Machine) -> Self {
        MachineTarget { machine }
    }
}

impl gdbstub::target::Target for MachineTarget {
    type Arch = gdbstub_arch::x86::X86_SSE;
    type Error = std::io::Error;

    fn guard_rail_implicit_sw_breakpoints(&self) -> bool {
        true // xxx
    }

    fn base_ops(&mut self) -> BaseOps<'_, Self::Arch, Self::Error> {
        BaseOps::SingleThread(self)
    }
}

impl SingleThreadBase for MachineTarget {
    fn read_registers(
        &mut self,
        regs: &mut <Self::Arch as gdbstub::arch::Arch>::Registers,
    ) -> gdbstub::target::TargetResult<(), Self> {
        todo!()
    }

    fn write_registers(
        &mut self,
        regs: &<Self::Arch as gdbstub::arch::Arch>::Registers,
    ) -> gdbstub::target::TargetResult<(), Self> {
        todo!()
    }

    fn read_addrs(
        &mut self,
        start_addr: <Self::Arch as gdbstub::arch::Arch>::Usize,
        data: &mut [u8],
    ) -> gdbstub::target::TargetResult<usize, Self> {
        todo!()
    }

    fn write_addrs(
        &mut self,
        start_addr: <Self::Arch as gdbstub::arch::Arch>::Usize,
        data: &[u8],
    ) -> gdbstub::target::TargetResult<(), Self> {
        todo!()
    }
}

enum MachineEventLoop {}

impl run_blocking::BlockingEventLoop for MachineEventLoop {
    type Target = MachineTarget;
    type Connection = std::net::TcpStream;

    // or MultiThreadStopReason on multi threaded targets
    type StopReason = SingleThreadStopReason<u32>;

    // Invoked immediately after the target's `resume` method has been
    // called. The implementation should block until either the target
    // reports a stop reason, or if new data was sent over the connection.
    fn wait_for_stop_reason(
        target: &mut MachineTarget,
        conn: &mut Self::Connection,
    ) -> Result<
        run_blocking::Event<SingleThreadStopReason<u32>>,
        run_blocking::WaitForStopReasonError<
            <Self::Target as Target>::Error,
            <Self::Connection as Connection>::Error,
        >,
    > {
        log::info!("wait");
        while target.machine.run() {
            log::info!("run");
            if conn
                .peek()
                .map_err(run_blocking::WaitForStopReasonError::Connection)?
                .is_some()
            {
                let byte = conn
                    .read()
                    .map_err(run_blocking::WaitForStopReasonError::Connection)?;
                return Ok(run_blocking::Event::IncomingData(byte));
            }
        }

        todo!();
        //return Ok(run_blocking::Event::TargetStopped(()))
    }

    // Invoked when the GDB client sends a Ctrl-C interrupt.
    fn on_interrupt(
        target: &mut MachineTarget,
    ) -> Result<Option<SingleThreadStopReason<u32>>, <MachineTarget as Target>::Error> {
        // target.stop_in_response_to_ctrl_c_interrupt()?;
        todo!();
        // a pretty typical stop reason in response to a Ctrl-C interrupt is to
        // report a "Signal::SIGINT".
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT).into()))
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

struct Logger {}
static LOGGER: Logger = Logger {};

impl log_crate::Log for Logger {
    fn enabled(&self, _metadata: &log_crate::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log_crate::Record) {
        let rec = log::Record {
            level: record.level(),
            file: record.file(),
            line: record.line(),
            args: record.args(),
        };
        log::log_record(&rec);
        // Implement the `log` method here.
        // You can use `println!` or any other logging mechanism.
    }

    fn flush(&self) {
        // Implement the `flush` method here.
        // You can use `std::io::stdout().flush()` or any other flushing mechanism.
    }
}

pub fn main(machine: Machine) -> anyhow::Result<()> {
    log_crate::set_logger(&LOGGER)?;

    let mut target = MachineTarget::new(machine);
    let connection: std::net::TcpStream = wait_for_gdb_connection(9001)?;
    let debugger = GdbStub::new(connection);

    let status = debugger.run_blocking::<MachineEventLoop>(&mut target)?;

    match status {
        DisconnectReason::Disconnect => {
            log::error!("Client disconnected")
        }
        DisconnectReason::TargetExited(code) => {
            log::error!("Target exited with code {}", code)
        }
        DisconnectReason::TargetTerminated(sig) => {
            log::error!("Target terminated with signal {}", sig)
        }
        DisconnectReason::Kill => log::error!("GDB sent a kill command"),
    }

    Ok(())
}
