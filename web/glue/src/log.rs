//! Hooks up the log crate (log::info!() etc.) to forward to a JS-based host.
//! This doesn't map directly to console.log etc. because we want to (eventually)
//! surface these logs in the UI.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type JsLogger;

    #[wasm_bindgen(method)]
    fn log(this: &JsLogger, level: u8, msg: String);
}

struct HostLogger {
    host: JsLogger,
}
impl log::Log for HostLogger {
    fn log(&self, record: &log::Record) {
        self.host.log(
            record.level as u8,
            format!("{}:{} {}", record.file, record.line, record.args),
        );
    }
}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

pub fn init(host: JsLogger) {
    let logger: &'static mut HostLogger = Box::leak(Box::new(HostLogger { host }));
    log::set_logger(logger);
    std::panic::set_hook(Box::new(panic_hook));
}
