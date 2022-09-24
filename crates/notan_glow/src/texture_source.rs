use crate::texture::create_texture;
use crate::GlowBackend;
use notan_graphics::color::Color;
use notan_graphics::{
    DeviceBackend, TextureBuilder, TextureFormat, TextureInfo, TextureSource, TextureUpdate,
    TextureUpdater,
};

// - Types of sources
/// An empty texture to be uploaded to the gpu
pub(crate) struct TextureSourceEmpty;

impl TextureSource for TextureSourceEmpty {
    fn create(
        &self,
        device: &mut dyn DeviceBackend,
        mut info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut() // TODO use downcast_unchecked once stabilized https://github.com/rust-lang/rust/issues/90850
            .ok_or_else(|| "Invalid backend type".to_string())?;

        self.inner_upload(backend, info)
    }

    fn update(&self, device: &mut dyn DeviceBackend, opts: TextureUpdate) -> Result<(), String> {
        todo!()
    }
}

impl TextureSourceEmpty {
    // As this is a private type used by this backend
    // it's better to use an inner impl and call it from the same crate
    // to avoid downcast the type when we know for sure it's GlowBackend
    pub(crate) fn inner_upload(
        &self,
        backend: &mut GlowBackend,
        info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let tex = unsafe { create_texture(&backend.gl, None, &info)? };
        let id = backend.add_inner_texture(tex, &info)?;
        Ok((id, info))
    }
}

/// An image file to be uploaded to the gpu
pub(crate) struct TextureSourceImage(pub Vec<u8>);

impl TextureSource for TextureSourceImage {
    fn create(
        &self,
        device: &mut dyn DeviceBackend,
        info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut() // TODO use downcast_unchecked once stabilized https://github.com/rust-lang/rust/issues/90850
            .ok_or_else(|| "Invalid backend type".to_string())?;

        self.inner_upload(backend, info)
    }

    fn update(&self, device: &mut dyn DeviceBackend, opts: TextureUpdate) -> Result<(), String> {
        todo!()
    }
}

impl TextureSourceImage {
    // As this is a private type used by this backend
    // it's better to use an inner impl and call it from the same crate
    // to avoid downcast the type when we know for sure it's GlowBackend
    pub(crate) fn inner_upload(
        &self,
        backend: &mut GlowBackend,
        mut info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let data = image::load_from_memory(&self.0)
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
}

/// An image bytes to be uploaded to the gpu
pub(crate) struct TextureSourceBytes(pub Vec<u8>);

impl TextureSource for TextureSourceBytes {
    fn create(
        &self,
        device: &mut dyn DeviceBackend,
        info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut() // TODO use downcast_unchecked once stabilized https://github.com/rust-lang/rust/issues/90850
            .ok_or_else(|| "Invalid backend type".to_string())?;

        self.inner_upload(backend, info)
    }

    fn update(&self, device: &mut dyn DeviceBackend, opts: TextureUpdate) -> Result<(), String> {
        todo!()
    }
}

impl TextureSourceBytes {
    // As this is a private type used by this backend
    // it's better to use an inner impl and call it from the same crate
    // to avoid downcast the type when we know for sure it's GlowBackend
    pub(crate) fn inner_upload(
        &self,
        backend: &mut GlowBackend,
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

        let pixels = if info.premultiplied_alpha {
            premultiplied_alpha(&self.0)
        } else {
            self.0.clone()
        };

        let tex = unsafe { create_texture(&backend.gl, Some(&pixels), &info)? };
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
