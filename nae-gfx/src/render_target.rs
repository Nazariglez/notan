use crate::texture::{Texture, TextureOptions};
use crate::{
    create_texture, matrix4_orthogonal, texture_from_gl_context, GlContext, Graphics, Matrix4,
};
use glow::HasContext;
use nae_core::{BaseApp, BaseSystem, TextureFilter, TextureFormat};
use std::rc::Rc;

type FramebufferKey = <glow::Context as HasContext>::Framebuffer;
type TextureKey = <glow::Context as HasContext>::Texture;

#[derive(Clone)]
struct RenderTargetKey {
    gl: GlContext,
    raw: FramebufferKey,
    depth_tex: Option<TextureKey>,
}

impl Drop for RenderTargetKey {
    fn drop(&mut self) {
        unsafe {
            if let Some(depth_tex) = self.depth_tex.take() {
                self.gl.delete_texture(depth_tex);
            }

            self.gl.delete_framebuffer(self.raw);
        }
    }
}

#[derive(Clone)]
pub struct RenderTarget {
    pub texture: Texture,
    raw: Rc<RenderTargetKey>,
}

impl RenderTarget {
    /// Create a new surface with the default options and custom size
    pub fn from_size<T, S>(
        app: &mut T,
        width: i32,
        height: i32,
        depth: bool,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        Self::from(app, width, height, depth, Default::default())
    }

    /// Create a new texture with custom options and size
    pub fn from<T, S>(
        app: &mut T,
        width: i32,
        height: i32,
        depth: bool,
        options: TextureOptions,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        let texture = Texture::from(app, width, height, options)?;
        let (raw, depth_tex) =
            create_framebuffer(app.system().gfx(), &texture, width, height, depth)?;

        let key = RenderTargetKey {
            gl: app.system().gfx().gl.clone(),
            raw,
            depth_tex,
        };

        Ok(Self {
            texture,
            raw: Rc::new(key),
        })
    }

    /// Returns the width of the inner texture
    pub fn width(&self) -> f32 {
        self.texture.width()
    }

    /// Returns the height of the inner texture
    pub fn height(&self) -> f32 {
        self.texture.height()
    }

    pub(crate) fn raw(&self) -> FramebufferKey {
        self.raw.raw
    }
}

fn create_framebuffer(
    gfx: &Graphics,
    texture: &Texture,
    width: i32,
    height: i32,
    depth: bool,
) -> Result<(FramebufferKey, Option<TextureKey>), String> {
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

        let depth_tex = if depth {
            Some(create_texture(
                gl,
                width,
                height,
                &[],
                4,
                &TextureOptions {
                    format: TextureFormat::Depth,
                    internal_format: TextureFormat::Depth,
                    min_filter: TextureFilter::Linear,
                    mag_filter: TextureFilter::Linear,
                },
            )?)
        } else {
            None
        };

        let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
        if status != glow::FRAMEBUFFER_COMPLETE {
            return Err(String::from("Framebuffer incomplete..."));
        }

        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        Ok((fb, depth_tex))
    }
}
