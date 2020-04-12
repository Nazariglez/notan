mod common;
pub use common::*;
pub use nae_core::*;
pub use nae_gfx::*;
pub use nae_glow::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "winit"))]
mod winit_backend;

#[cfg(all(not(target_arch = "wasm32"), feature = "winit"))]
pub use winit_backend::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "sdl"))]
mod sdl_backend;

#[cfg(all(not(target_arch = "wasm32"), feature = "sdl"))]
pub use sdl_backend::*;

#[cfg(target_arch = "wasm32")]
mod web_backend;

#[cfg(target_arch = "wasm32")]
pub use web_backend::*;
