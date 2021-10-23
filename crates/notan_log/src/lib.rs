pub use ::log::{debug, error, info, trace, warn, LevelFilter};

#[cfg(target_arch = "wasm32")]
mod console_error;

mod config;
pub use config::LogConfig;
