use crate::to_glow::*;
use crate::{create_texture_from_html, GlowBackend};
use glow::*;
use notan_graphics::prelude::*;

pub(crate) type TextureKey = <glow::Context as glow::HasContext>::Texture;

pub(crate) struct InnerTexture {
    pub texture: TextureKey,
    pub size: (i32, i32),
    pub is_srgba: bool,
}

impl InnerTexture {
    pub fn new(gl: &Context, info: &TextureInfo) -> Result<Self, String> {
        let texture = unsafe { create_texture(gl, info)? };
        let size = (info.width, info.height);
        let is_srgba = info.format == TextureFormat::SRgba8;
        Ok(Self {
            texture,
            size,
            is_srgba,
        })
    }

    pub fn new2(gl: &Context, texture: TextureKey, info: &TextureInfo) -> Result<Self, String> {
        let size = (info.width, info.height);
        let is_srgba = info.format == TextureFormat::SRgba8;
        Ok(Self {
            texture,
            size,
            is_srgba,
        })
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
    if slot < 16 {
        Ok(glow::TEXTURE0 + slot)
    } else {
        Err(format!(
            "Unsupported texture slot '{}', You can use up to 16.",
            slot
        ))
    }
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
        info.wrap_x.to_glow() as _,
    );

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        info.wrap_y.to_glow() as _,
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

#[cfg(target_arch = "wasm32")]
pub(crate) unsafe fn create_texture_from_html_image(
    gl: &Context,
    image: &web_sys::HtmlImageElement,
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
        info.wrap_x.to_glow() as _,
    );

    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        info.wrap_y.to_glow() as _,
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

    let depth = TextureFormat::Depth16 == info.format;
    let mut typ = glow::UNSIGNED_BYTE;
    let mut format = texture_format(&info.format);
    if depth {
        format = glow::DEPTH_COMPONENT;
        typ = glow::UNSIGNED_SHORT;

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

    gl.tex_image_2d_with_html_image(
        glow::TEXTURE_2D,
        0,
        texture_internal_format(&info.format) as _,
        format,
        typ,
        image,
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
        TextureFormat::SRgba8 => glow::RGBA,
    }
}

pub(crate) fn texture_internal_format(tf: &TextureFormat) -> u32 {
    match tf {
        TextureFormat::R8 => glow::R8,
        TextureFormat::SRgba8 => glow::SRGB8_ALPHA8,
        _ => texture_format(tf),
    }
}
