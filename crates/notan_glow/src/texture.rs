use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;

pub(crate) type TextureKey = <glow::Context as glow::HasContext>::Texture;

pub(crate) struct InnerTexture {
    pub texture: TextureKey,
    pub size: (i32, i32),
    pub is_srgba: bool,
    pub use_mipmaps: bool,
}

impl InnerTexture {
    pub fn new(texture: TextureKey, info: &TextureInfo) -> Result<Self, String> {
        let size = (info.width, info.height);
        let is_srgba = info.format == TextureFormat::SRgba8;
        let use_mipmaps = info.mipmap_filter.is_some();
        Ok(Self {
            texture,
            size,
            is_srgba,
            use_mipmaps,
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
            "Unsupported texture slot '{slot}', You can use up to 16.",
        ))
    }
}

pub(crate) struct TexInfo<'a> {
    pub texture: TextureKey,
    pub format: u32,
    pub data: Option<&'a [u8]>,
}

const CANNOT_USE_LINEAR_FILTER: [TextureFormat; 1] = [TextureFormat::R32Float];

fn assert_can_use_linear_filter(info: &TextureInfo) -> Result<(), String> {
    let needs_check_filter = CANNOT_USE_LINEAR_FILTER.contains(&info.format);
    if needs_check_filter
        && (matches!(info.min_filter, TextureFilter::Linear)
            || matches!(info.mag_filter, TextureFilter::Linear))
    {
        return Err(format!(
            "Textures with format {:?} can only use TextureFiler::Nearest filter.",
            info.format
        ));
    }

    Ok(())
}

pub(crate) unsafe fn pre_create_texture<'a>(
    gl: &Context,
    bytes: Option<&'a [u8]>,
    info: &TextureInfo,
) -> Result<TexInfo<'a>, String> {
    // Some texture types cannot use linear filtering
    assert_can_use_linear_filter(info)?;

    let texture = gl.create_texture()?;

    let bytes_per_pixel = info.bytes_per_pixel().min(8) as _;
    gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, bytes_per_pixel);

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

    let min_filter = get_min_filter(info);

    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_filter as _);

    let depth = matches!(info.format, TextureFormat::Depth16);
    let mut data = bytes;
    let mut format = texture_format(&info.format);
    if depth {
        format = glow::DEPTH_COMPONENT;
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

    Ok(TexInfo {
        texture,
        format,
        data,
    })
}

fn get_min_filter(info: &TextureInfo) -> u32 {
    match info.mipmap_filter {
        None => info.min_filter.to_glow(),
        Some(mipmap_filter) => match (info.min_filter, mipmap_filter) {
            (TextureFilter::Linear, TextureFilter::Linear) => glow::LINEAR_MIPMAP_LINEAR,
            (TextureFilter::Linear, TextureFilter::Nearest) => glow::LINEAR_MIPMAP_NEAREST,
            (TextureFilter::Nearest, TextureFilter::Linear) => glow::NEAREST_MIPMAP_LINEAR,
            (TextureFilter::Nearest, TextureFilter::Nearest) => glow::NEAREST_MIPMAP_NEAREST,
        },
    }
}

pub(crate) unsafe fn post_create_texture(gl: &Context, info: &TextureInfo) {
    if info.mipmap_filter.is_some() {
        gl.generate_mipmap(glow::TEXTURE_2D);
    }

    gl.bind_texture(glow::TEXTURE_2D, None);
}

pub(crate) unsafe fn create_texture(
    gl: &Context,
    bytes: Option<&[u8]>,
    info: &TextureInfo,
) -> Result<TextureKey, String> {
    let TexInfo {
        texture,
        format,
        data,
    } = pre_create_texture(gl, bytes, info)?;

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        texture_internal_format(&info.format) as _,
        info.width,
        info.height,
        0,
        format,
        texture_type(&info.format),
        data,
    );

    post_create_texture(gl, info);

    Ok(texture)
}

pub(crate) fn texture_type(tf: &TextureFormat) -> u32 {
    match tf {
        TextureFormat::R32Float => glow::FLOAT,
        TextureFormat::Rgba32Float => glow::FLOAT,
        TextureFormat::R32Uint => glow::UNSIGNED_INT,
        TextureFormat::R16Uint => glow::UNSIGNED_SHORT,
        TextureFormat::Depth16 => glow::UNSIGNED_SHORT,
        _ => glow::UNSIGNED_BYTE,
    }
}

pub(crate) fn texture_format(tf: &TextureFormat) -> u32 {
    match tf {
        TextureFormat::Rgba32 => glow::RGBA,
        TextureFormat::Rgba32Float => RGBA,
        TextureFormat::R8 => glow::RED,
        TextureFormat::R8Uint => glow::RED_INTEGER,
        TextureFormat::R16Uint => glow::RED_INTEGER,
        TextureFormat::R32Float => glow::RED,
        TextureFormat::R32Uint => glow::RED_INTEGER,
        TextureFormat::Depth16 => glow::DEPTH_COMPONENT16,
        TextureFormat::SRgba8 => glow::RGBA,
    }
}

pub(crate) fn texture_internal_format(tf: &TextureFormat) -> u32 {
    match tf {
        TextureFormat::R8 => glow::R8,
        TextureFormat::R8Uint => glow::R8UI,
        TextureFormat::R16Uint => R16UI,
        TextureFormat::R32Float => glow::R32F,
        TextureFormat::R32Uint => glow::R32UI,
        TextureFormat::SRgba8 => glow::SRGB8_ALPHA8,
        TextureFormat::Rgba32Float => glow::RGBA32F,
        _ => texture_format(tf),
    }
}
