//! Hook up the log crate to print to stdout.

static LOGGER: Logger = Logger {};
struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        eprintln!("{} {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

pub fn init() -> anyhow::Result<()> {
    log::set_logger(&LOGGER).map_err(|err| anyhow::anyhow!(err))?;
    log::set_max_level(log::LevelFilter::Debug);
    Ok(())
}
