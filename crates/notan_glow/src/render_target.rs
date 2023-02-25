use crate::clear;
use crate::texture::*;
use glow::*;
use notan_graphics::prelude::*;

pub(crate) struct InnerRenderTexture {
    fbo: Framebuffer,
    depth_texture: Option<TextureKey>,
    pub size: (u32, u32),
    pub use_mipmaps: bool,
}

impl InnerRenderTexture {
    pub fn new(gl: &Context, texture: &InnerTexture, info: &TextureInfo) -> Result<Self, String> {
        let use_mipmaps = texture.use_mipmaps;
        let width = info.width;
        let height = info.height;
        let depth_info = if info.depth {
            Some(DepthInfo { width, height })
        } else {
            None
        };

        let (fbo, depth_texture) = unsafe { create_fbo(gl, texture.texture, depth_info)? };
        let size = texture.size;
        Ok(Self {
            fbo,
            depth_texture,
            size,
            use_mipmaps,
        })
    }

    #[inline(always)]
    pub fn clean(&self, gl: &Context) {
        unsafe {
            gl.delete_framebuffer(self.fbo);
            if let Some(tex) = self.depth_texture {
                gl.delete_texture(tex);
            }
        }
    }

    #[inline]
    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo));
        }
    }
}

unsafe fn create_fbo(
    gl: &Context,
    texture: TextureKey,
    depth_info: Option<DepthInfo>,
) -> Result<(Framebuffer, Option<TextureKey>), String> {
    let fbo = gl.create_framebuffer()?;
    gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
    gl.framebuffer_texture_2d(
        glow::FRAMEBUFFER,
        glow::COLOR_ATTACHMENT0,
        glow::TEXTURE_2D,
        Some(texture),
        0,
    );

    let depth_texture = match depth_info {
        Some(info) => Some(create_texture(
            gl,
            None,
            &TextureInfo {
                width: info.width,
                height: info.height,
                format: TextureFormat::Depth16,
                min_filter: TextureFilter::Linear,
                mag_filter: TextureFilter::Linear,
                ..Default::default()
            },
        )?),
        _ => None,
    };

    let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
    if status != glow::FRAMEBUFFER_COMPLETE {
        return Err(
            "Cannot create a render target because the framebuffer is incomplete...".to_string(),
        );
    }

    // transparent clear to avoid weird visual glitches
    clear(gl, &Some(Color::TRANSPARENT), &None, &None);

    gl.bind_framebuffer(glow::FRAMEBUFFER, None);
    Ok((fbo, depth_texture))
}

struct DepthInfo {
    width: u32,
    height: u32,
}
