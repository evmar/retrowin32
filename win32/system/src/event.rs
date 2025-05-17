use std::sync::{Arc, Mutex};

pub type ArcEvent = Arc<Event>;

/// Event objects, used for synchronization between Windows threads.
//
// In the emulator there are no real threads, but we still use event objects
// for signaling all the way up to the Host layer, which itself may use threads.
// (In particular SDL audio notifications come in on a separate thread.)
// So internally, Events use Mutex/Arc instead of Cell/Rc.
// Under wasm Mutex/Arc are implemented as Cell/Rc anyway:
//   https://github.com/rust-lang/rust/issues/77839
pub struct Event {
    pub name: Option<String>,
    pub manual_reset: bool,
    pub signaled: Mutex<bool>,
}

impl Event {
    pub fn new(name: Option<String>, manual_reset: bool, signaled: bool) -> Arc<Self> {
        Arc::new(Self {
            name,
            manual_reset,
            signaled: Mutex::new(signaled),
        })
    }

    pub fn signal(&self) {
        *self.signaled.lock().unwrap() = true;
        // TODO: wake up waiting host threads somehow.
    }
}
