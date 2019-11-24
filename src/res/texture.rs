use super::loader::load_file;
use super::resource::*;
use crate::graphics::batchers::GraphicTexture;

use crate::app::App;
use crate::graphics::{create_gl_tex, GlContext};
use crate::log;
use futures::future::Future;
use glow::{HasContext, TEXTURE_ALPHA_TYPE};
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
        let mut inner = InnerTexture::empty(width, height);
        let gl = gl.clone();
        let tex = create_gl_tex(&gl, width, height, &vec![0; (width * height) as usize * 4])?;
        inner.gl = Some(gl);
        inner.tex = Some(tex);
        Ok(Self {
            inner: Rc::new(RefCell::new(inner)),
        })
    }
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

struct InnerTexture {
    width: i32,
    height: i32,
    raw: Vec<u8>,
    gl: Option<GlContext>,
    tex: Option<glow::WebTextureKey>,
}

impl InnerTexture {
    fn empty(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            raw: vec![],
            gl: None,
            tex: None,
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
    let mut padded_data = Vec::with_capacity(data.len() * 4);

    for a in data {
        padded_data.push(255);
        padded_data.push(255);
        padded_data.push(255);
        padded_data.push(*a);
    }

    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, texture.tex());

        gl.tex_sub_image_2d_u8_slice(
            glow::TEXTURE_2D,
            0,
            rect.min.x as _,
            rect.min.y as _,
            rect.width() as _,
            rect.height() as _,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&padded_data),
        );
    }
}

//TODO add rect and rotation to support texturepacker?
