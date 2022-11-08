mod backend;
mod clipboard;
mod keyboard;
mod mouse;
mod touch;
mod utils;
mod window;

#[cfg(feature = "drop_files")]
mod files;

#[cfg(feature = "audio")]
mod audio;

#[cfg(all(feature = "clipboard", not(web_sys_unstable_apis)))]
compile_error!("feature \"clipboard\" requires web_sys_unstable_apis to be enabled\nsee https://rustwasm.github.io/wasm-bindgen/web-sys/unstable-apis.html");

pub mod prelude;

pub use backend::*;
pub use notan_glow::texture_source::*;
