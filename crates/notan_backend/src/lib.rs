pub mod prelude;

#[cfg(target_arch = "wasm32")]
pub use notan_web::{WebBackend as DefaultBackend, *};

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
pub use notan_winit::{WinitBackend as DefaultBackend, *};

#[cfg(target_os = "ios")]
pub use notan_app::empty::{EmptyBackend as DefaultBackend, *};
