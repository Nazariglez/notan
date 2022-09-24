use crate::texture::create_texture;
use crate::GlowBackend;
use notan_graphics::color::Color;
use notan_graphics::{TextureFormat, TextureInfo};

pub(crate) fn add_empty_texture(
    backend: &mut GlowBackend,
    info: TextureInfo,
) -> Result<(u64, TextureInfo), String> {
    let tex = unsafe { create_texture(&backend.gl, None, &info)? };
    let id = backend.add_inner_texture(tex, &info)?;
    Ok((id, info))
}

pub(crate) fn add_texture_from_image(
    backend: &mut GlowBackend,
    buffer: Vec<u8>,
    mut info: TextureInfo,
) -> Result<(u64, TextureInfo), String> {
    let data = image::load_from_memory(&buffer)
        .map_err(|e| e.to_string())?
        .to_rgba8();

    let pixels = if info.premultiplied_alpha {
        premultiplied_alpha(&data)
    } else {
        data.to_vec()
    };

    info.format = TextureFormat::Rgba32;
    info.width = data.width() as _;
    info.height = data.height() as _;

    let tex = unsafe { create_texture(&backend.gl, Some(&pixels), &info)? };
    let id = backend.add_inner_texture(tex, &info)?;
    Ok((id, info))
}

pub(crate) fn add_texture_from_bytes(
    backend: &mut GlowBackend,
    bytes: Vec<u8>,
    info: TextureInfo,
) -> Result<(u64, TextureInfo), String> {
    #[cfg(debug_assertions)]
    {
        let size = info.width * info.height * 4;
        debug_assert_eq!(
            bytes.len(),
            size as usize,
            "Texture bytes of len {} when it should be {} (width: {} * height: {} * bytes: {})",
            bytes.len(),
            size,
            info.width,
            info.height,
            4
        );
    }

    let pixels = if info.premultiplied_alpha {
        premultiplied_alpha(&bytes)
    } else {
        bytes
    };

    let tex = unsafe { create_texture(&backend.gl, Some(&pixels), &info)? };
    let id = backend.add_inner_texture(tex, &info)?;
    Ok((id, info))
}

fn premultiplied_alpha(pixels: &[u8]) -> Vec<u8> {
    pixels
        .chunks(4)
        .flat_map(|c| {
            Color::from_bytes(c[0], c[1], c[2], c[3])
                .to_premultiplied_alpha()
                .rgba_u8()
        })
        .collect()
}
