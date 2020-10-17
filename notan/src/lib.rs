pub mod prelude;

pub use notan_app as app;
pub use notan_derive::main;

#[cfg(target_arch = "wasm32")]
pub use notan_web as window;

#[cfg(not(target_arch = "wasm32"))]
pub use notan_winit as window;

