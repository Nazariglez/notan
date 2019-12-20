mod surface;
mod context;
mod texture;
mod shader;
mod font;

pub use surface::*;
use glow::{Context, HasContext};
use std::rc::Rc;

pub(crate) type GlContext = Rc<Context>;

#[cfg(target_arch = "wasm32")]
pub(crate) type TextureKey = glow::WebTextureKey;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type TextureKey = glow::Texture;