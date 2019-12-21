mod batchers;
mod context;
mod font;
mod shader;
mod surface;
mod texture;

use glow::{Context, HasContext};
use std::rc::Rc;
pub use surface::*;

pub(crate) type GlContext = Rc<Context>;

#[cfg(target_arch = "wasm32")]
pub(crate) type TextureKey = glow::WebTextureKey;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type TextureKey = <glow::Context as HasContext>::Texture;

#[cfg(target_arch = "wasm32")]
pub(crate) type BufferKey = glow::WebBufferKey;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type BufferKey = <glow::Context as HasContext>::Buffer;

pub(crate) trait GlowValue {
    fn glow_value(&self) -> u32;
}
