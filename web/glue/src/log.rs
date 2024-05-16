//! Hooks up the log crate (log::info!() etc.) to forward to a JS-based host.
//! This doesn't map directly to console.log etc. because we want to (eventually)
//! surface these logs in the UI.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS: &'static str = r#"
export interface HostLogger {
    log(level: number, msg: string);
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "HostLogger")]
    pub type HostLogger;

    #[wasm_bindgen(method)]
    fn log(this: &HostLogger, level: u8, msg: &str);
}

struct GlueLogger {
    host: HostLogger,
}
impl log::Log for GlueLogger {
    fn log(&self, record: &log::Record) {
        let msg = format!("{}:{} {}", record.file, record.line, record.args);
        self.host.log(record.level as u8, &msg);
    }
}

fn panic_hook(info: &std::panic::PanicInfo) {
    log::error!("{}", info);
}

pub fn init(host: HostLogger) {
    let logger: &'static mut GlueLogger = Box::leak(Box::new(GlueLogger { host }));
    log::set_logger(logger);
    std::panic::set_hook(Box::new(panic_hook));
}
