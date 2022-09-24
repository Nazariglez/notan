#![cfg(target_arch = "wasm32")]

use crate::texture::{
    post_create_texture, pre_create_texture, texture_format, texture_internal_format, TexInfo,
    TextureKey,
};
use crate::GlowBackend;
use glow::Context;
use notan_graphics::device::DeviceBackend;
use notan_graphics::texture::{
    TextureBuilder, TextureInfo, TextureSource, TextureUpdate, TextureUpdater,
};

/// A html image element to be uploaded to the gpu
struct TextureSourceHtmlImage(web_sys::HtmlImageElement);

impl TextureSource for TextureSourceHtmlImage {
    fn create(
        &self,
        device: &mut dyn DeviceBackend,
        mut info: TextureInfo,
    ) -> Result<(u64, TextureInfo), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut() // TODO use downcast_unchecked once stabilized https://github.com/rust-lang/rust/issues/90850
            .ok_or_else(|| "Invalid backend type".to_string())?;

        info.width = self.0.width() as _;
        info.height = self.0.height() as _;

        let tex = unsafe { create_texture_from_html_image(&backend.gl, &self.0, &info)? };
        let id = backend.add_inner_texture(tex, &info)?;
        Ok((id, info))
    }

    fn update(&self, device: &mut dyn DeviceBackend, opts: TextureUpdate) -> Result<(), String> {
        let backend: &mut GlowBackend = device
            .as_any_mut()
            .downcast_mut() // TODO use downcast_unchecked once stabilized https://github.com/rust-lang/rust/issues/90850
            .ok_or_else(|| "Invalid backend type".to_string())?;

        unsafe { update_texture_from_html_image(&backend.gl, &self.0, &opts) }
    }
}

/// Extend the texture builder with new methods to load easily HtmlImageElement
pub trait HtmlTextureBuilder {
    /// Creates a Texture from an image
    #[allow(clippy::wrong_self_convention)]
    fn from_html_image(self, element: &web_sys::HtmlImageElement) -> Self;

    // TODO, from_html_canvas?
}

impl HtmlTextureBuilder for TextureBuilder<'_, '_> {
    fn from_html_image(self, element: &web_sys::HtmlImageElement) -> Self {
        self.from_source(TextureSourceHtmlImage(element.clone()))
    }
}

/// Extend the texture updater with new methods to update easily textures with HtmlImageElement
pub trait HtmlTextureUpdater {
    /// Creates a Texture from an image
    fn with_html_image(self, element: &web_sys::HtmlImageElement) -> Self;
}

impl HtmlTextureUpdater for TextureUpdater<'_> {
    fn with_html_image(self, element: &web_sys::HtmlImageElement) -> Self {
        self.with_source(TextureSourceHtmlImage(element.clone()))
    }
}

pub(crate) unsafe fn update_texture_from_html_image(
    gl: &Context,
    image: &web_sys::HtmlImageElement,
    opts: &TextureUpdate,
) -> Result<(), String> {
    gl.tex_sub_image_2d_with_html_image(
        glow::TEXTURE_2D,
        0,
        opts.x_offset,
        opts.y_offset,
        texture_format(&opts.format), // 3d texture needs another value?
        glow::UNSIGNED_BYTE,          // todo UNSIGNED SHORT FOR DEPTH (3d) TEXTURES
        image,
    );

    Ok(())
}

pub(crate) unsafe fn create_texture_from_html_image(
    gl: &Context,
    image: &web_sys::HtmlImageElement,
    info: &TextureInfo,
) -> Result<TextureKey, String> {
    let TexInfo {
        texture,
        typ,
        format,
        ..
    } = pre_create_texture(gl, None, info)?;

    gl.tex_image_2d_with_html_image(
        glow::TEXTURE_2D,
        0,
        texture_internal_format(&info.format) as _,
        format,
        typ,
        image,
    );

    post_create_texture(gl);

    Ok(texture)
}
