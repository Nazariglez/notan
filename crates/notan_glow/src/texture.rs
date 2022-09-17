use crate::to_glow::*;
use crate::GlowBackend;
use glow::*;
use notan_graphics::prelude::*;

pub type TextureKey = <glow::Context as glow::HasContext>::Texture;

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
            "Unsupported texture slot '{}', You can use up to 6.",
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

#[derive(Clone, Debug)]
pub struct TextureSourceImage(pub Vec<u8>);

impl TextureSource for TextureSourceImage {
    fn upload(&self, device: &mut dyn DeviceBackend, mut info: TextureInfo) -> Result<(), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut()
            .ok_or("Invalid backend type".to_string())?;

        let data = image::load_from_memory(&self.0)
            .map_err(|e| e.to_string())?
            .to_rgba8();

        let pixels = if info.premultiplied_alpha {
            premultiplied_alpha(data.to_vec())
        } else {
            data.to_vec()
        };

        info.bytes = Some(pixels);
        info.format = TextureFormat::Rgba32;
        info.width = data.width() as _;
        info.height = data.height() as _;

        let tex = unsafe { create_texture(&backend.gl, &info)? };

        backend.add_inner_texture(tex, &info)
    }
}

fn premultiplied_alpha(pixels: Vec<u8>) -> Vec<u8> {
    pixels
        .chunks(4)
        .flat_map(|c| {
            Color::from_bytes(c[0], c[1], c[2], c[3])
                .to_premultiplied_alpha()
                .rgba_u8()
        })
        .collect()
}
