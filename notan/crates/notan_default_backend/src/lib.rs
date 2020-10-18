#[cfg(target_arch = "wasm32")]
pub use notan_web::{WebBackend as DefaultBackend, *};

#[cfg(not(target_arch = "wasm32"))]
pub use notan_desktop::{DesktopBackend as DefaultBackend, *};
