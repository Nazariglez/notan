use crate::texture::{Texture, TextureOptions};
use crate::{matrix4_orthogonal, GlContext, Graphics, Matrix4};
use glow::HasContext;
use nae_core::{BaseApp, BaseSystem};

type FramebufferKey = <glow::Context as HasContext>::Framebuffer;

#[derive(Clone)]
pub struct RenderTarget {
    pub texture: Texture,
    pub(crate) raw: FramebufferKey,
}

impl RenderTarget {
    /// Create a new surface with the default options and custom size
    pub fn from_size<T, S>(app: &mut T, width: i32, height: i32) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        Self::from(app, width, height, Default::default())
    }

    /// Create a new texture with custom options and size
    pub fn from<T, S>(
        app: &mut T,
        width: i32,
        height: i32,
        options: TextureOptions,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        let texture = Texture::from(app, width, height, options)?;
        let raw = create_framebuffer(app.system().gfx(), &texture)?;
        Ok(Self { texture, raw })
    }

    /// Returns the width of the inner texture
    pub fn width(&self) -> f32 {
        self.texture.width()
    }

    /// Returns the height of the inner texture
    pub fn height(&self) -> f32 {
        self.texture.height()
    }
}

fn create_framebuffer(gfx: &Graphics, texture: &Texture) -> Result<FramebufferKey, String> {
    let gl = &gfx.gl;
    unsafe {
        let fb = gl.create_framebuffer()?;
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fb));
        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            glow::TEXTURE_2D,
            texture.raw(),
            0,
        );
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        Ok(fb)
    }
}
