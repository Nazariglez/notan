use crate::texture::{create_texture, TextureKey};
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
    let img = image_load_from_memory(&buffer)?;
    let tex = parse_image(backend, &img, &mut info)?;
    let id = backend.add_inner_texture(tex, &info)?;
    let max_size = backend.limits.max_texture_size;
    if invalid_tex_size(max_size, &info) {
        return Err(format!(
            "Texture '{}' size '{}x{}' is bigger than maximum texture size allowed per side '{}x{}'",
            id,
            info.width,
            info.height,
            max_size,
            max_size
        ));
    }
    Ok((id, info))
}

fn invalid_tex_size(max_size: u32, info: &TextureInfo) -> bool {
    info.width > max_size || info.height > max_size
}

fn image_load_from_memory(buffer: &[u8]) -> Result<image::DynamicImage, String> {
    let format = image::guess_format(buffer).map_err(|e| e.to_string())?;
    let mut reader = image::io::Reader::with_format(std::io::Cursor::new(buffer), format);
    // TODO allow to pass the limit from a config by the user
    reader.no_limits();
    reader.decode().map_err(|e| e.to_string())
}

fn parse_image(
    backend: &mut GlowBackend,
    img: &image::DynamicImage,
    info: &mut TextureInfo,
) -> Result<TextureKey, String> {
    // TODO process the loading of more texture types directly?
    match info.format {
        TextureFormat::Rgba32Float => {
            let mut data = img.to_rgba32f();
            let width = data.width() as _;
            let height = data.height() as _;
            if info.premultiplied_alpha {
                data.pixels_mut().for_each(|rgba| {
                    rgba.0 = Color::from(rgba.0).to_premultiplied_alpha().into();
                });
            }
            info.width = width;
            info.height = height;
            let tex =
                unsafe { create_texture(&backend.gl, Some(bytemuck::cast_slice(&data)), info)? };
            Ok(tex)
        }
        _ => {
            let mut data = img.to_rgba8();
            let width = data.width() as _;
            let height = data.height() as _;
            if info.premultiplied_alpha {
                data.pixels_mut().for_each(|rgba| {
                    rgba.0 = Color::from(rgba.0).to_premultiplied_alpha().into();
                });
            }
            info.width = width;
            info.height = height;
            let tex =
                unsafe { create_texture(&backend.gl, Some(bytemuck::cast_slice(&data)), info)? };
            Ok(tex)
        }
    }
}

pub(crate) fn add_texture_from_bytes(
    backend: &mut GlowBackend,
    bytes: Vec<u8>,
    info: TextureInfo,
) -> Result<(u64, TextureInfo), String> {
    #[cfg(debug_assertions)]
    {
        let size = info.width * info.height * (info.bytes_per_pixel() as u32);
        debug_assert_eq!(
            bytes.len(),
            size as usize,
            "Texture bytes of len {} when it should be {} (width: {} * height: {} * bytes: {})",
            bytes.len(),
            size,
            info.width,
            info.height,
            info.bytes_per_pixel(),
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
