//! Hooks up the log crate (log::info!() etc.) to forward to a JS-based host.
//! This doesn't map directly to console.log etc. because we want to (eventually)
//! surface these logs in the UI.

use wasm_bindgen::prelude::*;

use crate::JsResult;

#[wasm_bindgen]
extern "C" {
    pub type JsLogger;

    #[wasm_bindgen(method)]
    fn log(this: &JsLogger, level: u8, msg: String);
}

struct HostLogger {
    host: JsLogger,
}
unsafe impl Send for HostLogger {}
unsafe impl Sync for HostLogger {}
impl log::Log for HostLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            log::Level::Error => 5,
            log::Level::Warn => 4,
            log::Level::Info => 3,
            log::Level::Debug => 2,
            log::Level::Trace => 1,
        };
        self.host.log(
            level,
            format!(
                "{}:{}: {}",
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.args()
            ),
        );
    }

    fn flush(&self) {}
}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

pub fn init(host: JsLogger) -> JsResult<()> {
    let logger: &'static mut HostLogger = Box::leak(Box::new(HostLogger { host }));
    log::set_logger(logger).map_err(|err| JsError::new(&err.to_string()))?;
    log::set_max_level(log::LevelFilter::Debug);
    std::panic::set_hook(Box::new(panic_hook));
    Ok(())
}
