#[cfg(target_arch = "wasm32")]
pub use notan_web::prelude::*;

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
pub use notan_winit::prelude::*;

#[cfg(target_os = "ios")]
pub use notan_app::prelude::*;
