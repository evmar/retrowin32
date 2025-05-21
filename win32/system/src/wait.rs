use crate::host::Host;

const INFINITE: u32 = 0xffff_ffff;

pub enum Wait {
    None,
    Forever,
    // In absolute time as measured by host.ticks().
    Millis(u32),
}

impl Wait {
    pub fn from_millis(host: &dyn Host, ms: u32) -> Self {
        if ms == 0 {
            Wait::None
        } else if ms == INFINITE {
            Wait::Forever
        } else {
            Wait::Millis(host.ticks() + ms)
        }
    }
}

pub enum WaitResult {
    Object(u32),
    Timeout,
    // Abandoned
}

const WAIT_OBJECT_0: u32 = 0;
//const WAIT_ABANDONED_0: u32 = 0x80;
const WAIT_TIMEOUT: u32 = 0x102;

impl WaitResult {
    pub fn to_code(&self) -> u32 {
        match self {
            WaitResult::Object(index) => WAIT_OBJECT_0 + index,
            WaitResult::Timeout => WAIT_TIMEOUT,
        }
    }
}
