#[cfg(target_arch = "wasm32")]
pub use notan_web::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub use notan_winit::prelude::*;
