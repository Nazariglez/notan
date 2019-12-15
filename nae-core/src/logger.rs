#[cfg(target_arch = "wasm32")]
pub use log::{debug, error, info, trace, warn, Level};

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[cfg(target_arch = "wasm32")]
impl From<LogLevel> for Level {
    fn from(lvl: LogLevel) -> Self {
        use LogLevel::*;
        match lvl {
            Trace => Level::Trace,
            Debug => Level::Debug,
            Info => Level::Info,
            Warn => Level::Warn,
            Error => Level::Error,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn init_logger(level: LogLevel) {
    console_log::init_with_level(level.into());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init_logger(level: LogLevel) {
    unimplemented!()
}
