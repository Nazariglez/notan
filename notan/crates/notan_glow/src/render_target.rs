use crate::texture::*;
use glow::*;
use notan_graphics::prelude::*;

pub(crate) struct InnerRenderTarget {
    fbo: Framebuffer,
    pub size: (i32, i32),
}

impl InnerRenderTarget {
    pub fn new(gl: &Context, texture: &InnerTexture) -> Result<Self, String> {
        let fbo = unsafe { create_fbo(gl, texture.texture)? };
        let size = texture.size;
        Ok(Self { fbo, size })
    }

    #[inline(always)]
    pub fn clean(&self, gl: &Context) {
        unsafe {
            gl.delete_framebuffer(self.fbo);
        }
    }

    #[inline]
    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo));
        }
    }
}

unsafe fn create_fbo(gl: &Context, texture: TextureKey) -> Result<Framebuffer, String> {
    let fbo = gl.create_framebuffer()?;
    gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
    gl.framebuffer_texture_2d(
        glow::FRAMEBUFFER,
        glow::COLOR_ATTACHMENT0,
        glow::TEXTURE_2D,
        Some(texture),
        0,
    );

    let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
    if status != glow::FRAMEBUFFER_COMPLETE {
        return Err(
            "Cannot create a render target because the frambuffer is incomplete...".to_string(),
        );
    }

    gl.bind_framebuffer(glow::FRAMEBUFFER, None);
    Ok(fbo)
}
