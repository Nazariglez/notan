use crate::context::Context2d;
use crate::texture::Texture;
use crate::{GlContext, TextureKey};
use glow::HasContext;
use nae_core::graphics::BaseSurface;
use nae_core::resources::BaseTexture;
use nae_core::BaseApp;

#[cfg(target_arch = "wasm32")]
type FramebufferKey = glow::WebFramebufferKey;

#[cfg(not(target_arch = "wasm32"))]
type FramebufferKey = <glow::Context as HasContext>::Framebuffer;

pub struct Surface {
    texture: Texture,
    pub(crate) fbo: FramebufferKey,
}

impl BaseSurface for Surface {
    type Texture = Texture;
    type Context2d = Context2d;

    fn from_size<T: BaseApp<Graphics = Self::Context2d>>(
        app: &mut T,
        width: i32,
        height: i32,
    ) -> Result<Self, String> {
        let texture = Texture::from_size(app, width, height)?;
        let fbo = create_framebuffer(&app.graphics().gl, texture.tex())?;
        Ok(Self { texture, fbo })
    }

    fn width(&self) -> f32 {
        self.texture.width()
    }

    fn height(&self) -> f32 {
        self.texture.height()
    }

    fn texture(&self) -> &Self::Texture {
        &self.texture
    }
}

fn create_framebuffer(gl: &GlContext, tex: Option<TextureKey>) -> Result<FramebufferKey, String> {
    unsafe {
        let fb = gl.create_framebuffer()?;
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fb));
        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            glow::TEXTURE_2D,
            tex,
            0,
        );
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        Ok(fb)
    }
}
