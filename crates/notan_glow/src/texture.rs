use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;

pub type TextureKey = <glow::Context as glow::HasContext>::Texture;

pub(crate) struct InnerTexture {
    pub texture: TextureKey,
    pub size: (i32, i32),
}

impl InnerTexture {
    pub fn new(gl: &Context, info: &TextureInfo) -> Result<Self, String> {
        let texture = unsafe { create_texture(gl, info)? };
        let size = (info.width, info.height);
        Ok(Self { texture, size })
    }

    pub fn bind(&self, gl: &Context, slot: u32, location: &UniformLocation) {
        unsafe {
            gl.active_texture(gl_slot(slot).unwrap());
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.uniform_1_i32(Some(location), slot as _);
        }
    }

    #[inline(always)]
    pub fn clean(self, gl: &Context) {
        unsafe {
            gl.delete_texture(self.texture);
        }
    }
}

#[inline]
fn gl_slot(slot: u32) -> Result<u32, String> {
    Ok(match slot {
        0 => glow::TEXTURE0,
        1 => glow::TEXTURE1,
        2 => glow::TEXTURE2,
        3 => glow::TEXTURE3,
        4 => glow::TEXTURE4,
        5 => glow::TEXTURE5,
        6 => glow::TEXTURE6,
        7 => glow::TEXTURE7,
        _ => return Err(format!("Unsupported texture slot '{}'", slot)),
    })
}

pub(crate) unsafe fn create_texture(
    gl: &Context,
    info: &TextureInfo,
) -> Result<TextureKey, String> {
    let texture = gl.create_texture()?;

    let bytes_per_pixel = info.bytes_per_pixel();
    if bytes_per_pixel != 4 {
        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, bytes_per_pixel as _);
    }

    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_S,
        glow::CLAMP_TO_EDGE as _,
    );

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        glow::CLAMP_TO_EDGE as _,
    );

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        info.mag_filter.to_glow() as _,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        info.min_filter.to_glow() as _,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_S,
        glow::CLAMP_TO_EDGE as _,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        glow::CLAMP_TO_EDGE as _,
    );

    let depth = TextureFormat::Depth16 == info.format;
    let mut data = info.bytes.as_deref();
    let mut typ = glow::UNSIGNED_BYTE;
    let mut format = texture_format(&info.format);
    if depth {
        format = glow::DEPTH_COMPONENT;
        typ = glow::UNSIGNED_SHORT;
        data = None;

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as _,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as _,
        );

        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::DEPTH_ATTACHMENT,
            glow::TEXTURE_2D,
            Some(texture),
            0,
        );
    }

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        texture_internal_format(&info.format) as _,
        info.width,
        info.height,
        0,
        format,
        typ,
        data,
    );

    //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
    gl.bind_texture(glow::TEXTURE_2D, None);
    Ok(texture)
}

pub(crate) fn texture_format(tf: &TextureFormat) -> u32 {
    match tf {
        TextureFormat::Rgba32 => glow::RGBA,
        TextureFormat::R8 => glow::RED,
        TextureFormat::Depth16 => glow::DEPTH_COMPONENT16,
    }
}

pub(crate) fn texture_internal_format(tf: &TextureFormat) -> u32 {
    match tf {
        TextureFormat::R8 => glow::R8,
        _ => texture_format(tf),
    }
}
