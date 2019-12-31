#[cfg(target_arch = "wasm32")]
pub use nae_web::*;

#[cfg(not(target_arch = "wasm32"))]
pub use nae_native::*;
