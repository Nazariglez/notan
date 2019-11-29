use super::loader::load_file;
use super::resource::*;
use crate::graphics::batchers::GraphicTexture;

use crate::app::App;
use crate::graphics::{create_gl_tex, create_gl_tex_ext, GlContext};
use crate::log;
use futures::future::Future;
use glow::{HasContext, TEXTURE_ALPHA_TYPE};
use nalgebra_glm::magnitude;
use std::cell::RefCell;
use std::rc::Rc;

/// Represent an image resource
#[derive(Clone)]
pub struct Texture {
    inner: Rc<RefCell<InnerTexture>>,
}

impl Texture {
    /// Returns the texture's width
    pub fn width(&self) -> f32 {
        self.inner.borrow().width as _
    }

    /// Returns the texture's height
    pub fn height(&self) -> f32 {
        self.inner.borrow().height as _
    }

    /// Returns the graphics texture ready to draw on the gpu
    pub fn tex(&self) -> Option<glow::WebTextureKey> {
        self.inner.borrow().tex
    }

    /// Create a new texture with a custom size
    pub fn from_size(gl: &GlContext, width: i32, height: i32) -> Result<Self, String> {
        Texture::from(
            gl,
            width,
            height,
            TextureFormat::Rgba,
            TextureFormat::Rgba,
            TextureFilter::Nearest,
            TextureFilter::Nearest,
        )
    }

    /// Create a new texture using custom size and format
    pub fn from(
        gl: &GlContext,
        width: i32,
        height: i32,
        internal_format: TextureFormat,
        format: TextureFormat,
        min_filter: TextureFilter,
        mag_filter: TextureFilter,
    ) -> Result<Self, String> {
        let mut inner = InnerTexture::empty(width, height);
        let gl = gl.clone();
        let bpp = byte_per_pixel(internal_format, format);
        let tex = create_gl_tex_ext(
            &gl,
            width,
            height,
            &vec![0; (width * height) as usize * bpp],
            internal_format.into(),
            format.into(),
            min_filter.into(),
            mag_filter.into(),
            bpp,
        )?;
        inner.gl = Some(gl);
        inner.tex = Some(tex);
        Ok(Self {
            inner: Rc::new(RefCell::new(inner)),
        })
    }

    /// Returns the texture format
    pub fn format(&self) -> TextureFormat {
        self.inner.borrow().format
    }

    //TODO provide more info, like internal format, and min/mag filters
}

impl Resource for Texture {
    fn parse(&mut self, app: &mut App, data: Vec<u8>) -> Result<(), String> {
        let data = image::load_from_memory(&data)
            .map_err(|e| e.to_string())?
            .to_rgba();

        let width = data.width() as _;
        let height = data.height() as _;
        let raw = data.to_vec();
        let gl = app.graphics.gl.clone();
        let tex = create_gl_tex(&gl, width, height, &raw)?;

        *self.inner.borrow_mut() = InnerTexture {
            width,
            height,
            raw,
            gl: Some(gl),
            tex: Some(tex),
            format: TextureFormat::Rgba,
        };
        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.inner.borrow().tex.is_some()
    }
}

impl ResourceConstructor for Texture {
    fn new(file: &str) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerTexture::empty(1, 1))),
        }
    }
}

//bytes_per_pixe table https://webgl2fundamentals.org/webgl/lessons/webgl-data-textures.html
fn byte_per_pixel(internal: TextureFormat, format: TextureFormat) -> usize {
    use TextureFormat::*;

    match (internal, format) {
        (R8, Red) => 1,
        _ => 4,
    }
}

struct InnerTexture {
    width: i32,
    height: i32,
    raw: Vec<u8>,
    gl: Option<GlContext>,
    tex: Option<glow::WebTextureKey>,
    format: TextureFormat,
}

impl InnerTexture {
    fn empty(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            raw: vec![],
            gl: None,
            tex: None,
            format: TextureFormat::Rgba,
        }
    }
}

impl Drop for InnerTexture {
    fn drop(&mut self) {
        if let (Some(gl), Some(tex)) = (self.gl.as_ref(), self.tex) {
            unsafe {
                gl.delete_texture(tex);
            }
        }
    }
}

pub(crate) fn update_texture(
    gl: &GlContext,
    texture: &Texture,
    rect: glyph_brush::rusttype::Rect<u32>,
    data: &[u8],
) {
    let xx = rect.min.x as i32;
    let yy = rect.min.y as i32;
    let ww = rect.width() as i32;
    let hh = rect.height() as i32;

    //    let rgba_data = data
    //        .iter()
    //        .flat_map(|a| vec![255, 255, 255, *a])
    //        .collect::<Vec<u8>>();
    //
    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, texture.tex());

        gl.tex_sub_image_2d_u8_slice(
            glow::TEXTURE_2D,
            0,
            xx,
            yy,
            ww,
            hh,
            glow::RED, //texture.format().into(),
            glow::UNSIGNED_BYTE,
            //            Some(&rgba_data),
            Some(data),
        );
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba,
    Red,
    R8,
}

impl From<TextureFormat> for u32 {
    fn from(f: TextureFormat) -> Self {
        use TextureFormat::*;
        match f {
            Rgba => glow::RGBA,
            Red => glow::RED,
            R8 => glow::R8,
        }
    }
}

impl From<TextureFormat> for i32 {
    fn from(f: TextureFormat) -> Self {
        let f: u32 = f.into();
        f as _
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

impl From<TextureFilter> for u32 {
    fn from(f: TextureFilter) -> Self {
        use TextureFilter::*;
        match f {
            Linear => glow::LINEAR,
            Nearest => glow::NEAREST,
        }
    }
}

impl From<TextureFilter> for i32 {
    fn from(f: TextureFilter) -> Self {
        let f: u32 = f.into();
        f as _
    }
}

//TODO add rect and rotation to support texturepacker?
