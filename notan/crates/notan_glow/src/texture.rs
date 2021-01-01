use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;

type TextureKey = <glow::Context as glow::HasContext>::Texture;

pub(crate) struct InnerTexture {
    texture: TextureKey,
}

impl InnerTexture {
    pub fn new(gl: &Context, info: &TextureInfo) -> Result<Self, String> {
        let texture = unsafe { create_texture(gl, info)? };
        Ok(Self { texture })
    }

    pub fn bind(&self, gl: &Context, slot: u32, location: &UniformLocation) {
        unsafe {
            gl.active_texture(gl_slot(slot).unwrap());
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.uniform_1_i32(Some(location), slot as _);
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

unsafe fn create_texture(gl: &Context, info: &TextureInfo) -> Result<TextureKey, String> {
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

    let depth = TextureFormat::Depth == info.format;
    let mut data = Some(info.bytes.as_slice());
    let mut typ = glow::UNSIGNED_BYTE;
    let mut format = info.format.to_glow();
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
        info.internal_format.to_glow() as _,
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
