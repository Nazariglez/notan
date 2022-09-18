use crate::texture::create_texture;
use crate::GlowBackend;
use notan_graphics::color::Color;
use notan_graphics::{DeviceBackend, TextureBuilder, TextureFormat, TextureInfo, TextureSource};

#[cfg(target_arch = "wasm32")]
use crate::texture::create_texture_from_html_image;

// - Types of sources

/// An image file to be uploaded to the gpu
#[derive(Clone, Debug)]
pub struct TextureSourceImage(pub Vec<u8>);

impl TextureSource for TextureSourceImage {
    fn upload(
        &self,
        device: &mut dyn DeviceBackend,
        mut info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut()
            .ok_or("Invalid backend type".to_string())?;

        let data = image::load_from_memory(&self.0)
            .map_err(|e| e.to_string())?
            .to_rgba8();

        let pixels = if info.premultiplied_alpha {
            premultiplied_alpha(&data)
        } else {
            data.to_vec()
        };

        info.bytes = Some(pixels);
        info.format = TextureFormat::Rgba32;
        info.width = data.width() as _;
        info.height = data.height() as _;

        let tex = unsafe { create_texture(&backend.gl, &info)? };
        let id = backend.add_inner_texture(tex, &info)?;
        Ok((id, info))
    }
}

/// An image bytes to be uploaded to the gpu
#[derive(Clone, Debug)]
pub struct TextureSourceBytes(pub Vec<u8>);

impl TextureSource for TextureSourceBytes {
    fn upload(
        &self,
        device: &mut dyn DeviceBackend,
        mut info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        #[cfg(debug_assertions)]
        {
            let size = info.width * info.height * 4;
            debug_assert_eq!(
                self.0.len(),
                size as usize,
                "Texture bytes of len {} when it should be {} (width: {} * height: {} * bytes: {})",
                self.0.len(),
                size,
                info.width,
                info.height,
                4
            );
        }

        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut()
            .ok_or("Invalid backend type".to_string())?;

        let pixels = if info.premultiplied_alpha {
            premultiplied_alpha(&self.0)
        } else {
            self.0.clone()
        };

        info.bytes = Some(pixels);

        let tex = unsafe { create_texture(&backend.gl, &info)? };
        let id = backend.add_inner_texture(tex, &info)?;
        Ok((id, info))
    }
}

#[cfg(target_arch = "wasm32")]
/// A html image element to be uploaded to the gpu
#[derive(Clone, Debug)]
pub struct TextureSourceHtmlImage(pub web_sys::HtmlImageElement);

#[cfg(target_arch = "wasm32")]
impl TextureSource for TextureSourceHtmlImage {
    fn upload(
        &self,
        device: &mut dyn DeviceBackend,
        mut info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut()
            .ok_or("Invalid backend type".to_string())?;

        info.width = self.0.width() as _;
        info.height = self.0.height() as _;

        let tex = unsafe { create_texture_from_html_image(&backend.gl, &self.0, &info)? };
        let id = backend.add_inner_texture(tex, &info)?;
        Ok((id, info))
    }
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

// - Trait helpers
//
// pub trait TextureSourceBuilder {
//     /// Creates a Texture from an image
//     fn from_image(self, bytes: &[u8]) -> Self;
//
//     /// Creates a Texture from a buffer of pixels
//     fn from_bytes(self, bytes: &[u8], width: i32, height: i32) -> Self;
//
//     /// Creates a buffer for the size passed in and creates a Texture with it
//     fn from_empty_buffer(self, width: i32, height: i32) -> Self;
// }
//
// impl TextureSourceBuilder for TextureBuilder<'_, '_> {
//     /// Creates a Texture from an image
//     fn from_image(mut self, bytes: &[u8]) -> Self {
//         self.from_source(TextureSourceImage(bytes.to_vec()))
//     }
//
//     /// Creates a Texture from a buffer of pixels
//     fn from_bytes(mut self, bytes: &[u8], width: i32, height: i32) -> Self {
//         self.from_source(TextureSourceBytes(bytes.to_vec()))
//             .with_size(width, height)
//     }
//
//     /// Creates a buffer for the size passed in and creates a Texture with it
//     fn from_empty_buffer(mut self, width: i32, height: i32) -> Self {
//         self.with_size(width, height)
//     }
// }
