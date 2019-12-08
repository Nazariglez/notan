use crate::app::App;
use crate::graphics::GlContext;
use crate::res::{ResourceConstructor, Texture};
use glow::HasContext;

//https://webgl2fundamentals.org/webgl/lessons/webgl-render-to-texture.html
pub struct Surface {
    texture: Texture,
    pub(crate) fbo: glow::WebFramebufferKey,
    gl: GlContext,
}

impl Surface {
    pub fn from_size(app: &mut App, width: i32, height: i32) -> Result<Self, String> {
        let gl = app.graphics.gl.clone();
        let texture = Texture::from_size(&gl, width, height)?;
        let fbo = create_framebuffer(&gl, texture.tex())?;
        Ok(Self { texture, fbo, gl })
    }

    pub fn width(&self) -> f32 {
        self.texture.width()
    }

    pub fn height(&self) -> f32 {
        self.texture.height()
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

fn create_framebuffer(
    gl: &GlContext,
    tex: Option<glow::WebTextureKey>,
) -> Result<glow::WebFramebufferKey, String> {
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
