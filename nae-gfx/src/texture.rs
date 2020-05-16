use crate::{GlContext, GlowValue, Graphics, TextureKey};
use glow::HasContext;
use nae_core::math::Rect;
use nae_core::{BaseApp, BaseGfx, BaseSystem, TextureFilter, TextureFormat};
use std::cell::RefCell;
use std::rc::Rc;

struct InnerTexture {
    gl: GlContext,
    width: i32,
    height: i32,
    texture: Option<TextureKey>,
    buffer: Vec<u8>,
    format: TextureFormat,
    internal_format: TextureFormat,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
}

impl Drop for InnerTexture {
    fn drop(&mut self) {
        if let Some(texture) = self.texture {
            unsafe {
                self.gl.delete_texture(texture);
            }
        }
    }
}

impl InnerTexture {
    fn frame(&self) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}

/// Represents a texture loaded in memory
#[derive(Clone)]
pub struct Texture {
    inner: Rc<RefCell<InnerTexture>>,
    frame: Option<Rect>,
}

impl Texture {
    /// Returns if the texture is ready to render
    pub fn is_loaded(&self) -> bool {
        self.inner.borrow().texture.is_some()
    }

    /// Returns the current frame
    pub fn frame(&self) -> Rect {
        self.frame.clone().unwrap_or(self.inner.borrow().frame())
    }

    /// Returns a new texture sharing the texture but with a new frame
    pub fn with_frame(&self, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            inner: self.inner.clone(),
            frame: Some(Rect {
                x,
                y,
                width,
                height,
            }),
        }
    }

    /// Create a new texture with the default options and custom size
    pub fn from_size<T, S>(app: &mut T, width: i32, height: i32) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        Self::from(app, width, height, Default::default())
    }

    /// Create a new texture with custom options and size
    pub fn from<T, S>(
        app: &mut T,
        width: i32,
        height: i32,
        options: TextureOptions,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        texture_from_gl_context(&app.system().gfx().gl, width, height, &options)
    }

    /// Create a new texture from bytes
    pub fn from_bytes<T, S>(app: &mut T, data: &[u8]) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        Self::from_bytes_with_options(app, data, Default::default())
    }

    /// Create a new texture from bytes with custom options
    pub fn from_bytes_with_options<T, S>(
        app: &mut T,
        data: &[u8],
        options: TextureOptions,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        let mut texture = Self::from(app, 1, 1, options)?;
        texture.parse_data(app, data.to_vec());
        Ok(texture)
    }

    /// Width of the base texture without the current frame's size
    pub fn base_width(&self) -> f32 {
        self.inner.borrow().width as _
    }

    /// Height of the base texture without the current frame's size
    pub fn base_height(&self) -> f32 {
        self.inner.borrow().height as _
    }

    pub(crate) fn raw(&self) -> Option<TextureKey> {
        self.inner.borrow().texture
    }

    /// Texture's width
    pub fn width(&self) -> f32 {
        self.frame
            .as_ref()
            .map_or(self.inner.borrow().width as _, |f| f.width)
    }

    /// Texture's height
    pub fn height(&self) -> f32 {
        self.frame
            .as_ref()
            .map_or(self.inner.borrow().height as _, |f| f.height)
    }

    pub fn parse_data<T, S>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Graphics = Graphics>,
    {
        let data = image::load_from_memory(&data)
            .map_err(|e| e.to_string())?
            .to_rgba();

        let width = data.width() as _;
        let height = data.height() as _;
        let mut inner = self.inner.borrow_mut();
        let opts = TextureOptions {
            format: inner.format,
            internal_format: inner.internal_format,
            min_filter: inner.min_filter,
            mag_filter: inner.mag_filter,
        };
        let raw_data = data.to_vec();

        let texture = create_texture(
            &app.system().gfx().gl,
            width,
            height,
            &raw_data,
            bytes_per_pixel(&opts.format, &opts.internal_format),
            &opts,
        )?;

        inner.texture = Some(texture);
        inner.buffer = raw_data;
        inner.width = width;
        inner.height = height;

        Ok(())
    }
}

/// Represent the options to create a new texture
pub struct TextureOptions {
    pub format: TextureFormat,
    pub internal_format: TextureFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
}

impl Default for TextureOptions {
    fn default() -> Self {
        Self {
            format: TextureFormat::Rgba,
            internal_format: TextureFormat::Rgba,
            mag_filter: TextureFilter::Nearest,
            min_filter: TextureFilter::Nearest,
        }
    }
}

impl GlowValue for TextureFormat {
    type VALUE = u32;

    fn glow_value(&self) -> Self::VALUE {
        use TextureFormat::*;
        match self {
            Rgba => glow::RGBA,
            Red => glow::RED,
            R8 => glow::R8,
        }
    }
}

impl GlowValue for TextureFilter {
    type VALUE = u32;

    fn glow_value(&self) -> Self::VALUE {
        use TextureFilter::*;
        match self {
            Linear => glow::LINEAR,
            Nearest => glow::NEAREST,
        }
    }
}

fn create_texture(
    gl: &GlContext,
    width: i32,
    height: i32,
    data: &[u8],
    bytes_per_pixel: usize,
    opts: &TextureOptions,
) -> Result<TextureKey, String> {
    unsafe {
        let texture = gl.create_texture()?;
        let format = opts.format.glow_value();
        let internal = opts.internal_format.glow_value();
        let min_filter = opts.min_filter.glow_value();
        let mag_filter = opts.mag_filter.glow_value();

        if bytes_per_pixel != 4 {
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, bytes_per_pixel as _);
        }

        gl.bind_texture(glow::TEXTURE_2D, Some(texture));

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as _,
        );

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as _,
        );

        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, mag_filter as _);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_filter as _);

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            internal as _,
            width,
            height,
            0,
            format as _,
            glow::UNSIGNED_BYTE,
            Some(data),
        );

        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(texture)
    }
}

fn bytes_per_pixel(format: &TextureFormat, internal_format: &TextureFormat) -> usize {
    use TextureFormat::*;
    match (format, internal_format) {
        (Red, R8) => 1,
        _ => 4,
    }
}

pub(crate) fn max_texture_size(gl: &GlContext) -> i32 {
    unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) }
}

pub(crate) fn texture_from_gl_context(
    gl: &GlContext,
    width: i32,
    height: i32,
    opts: &TextureOptions,
) -> Result<Texture, String> {
    let max_size = max_texture_size(gl);
    if width > max_size || height > max_size {
        return Err(format!(
            "Texture size {}x{} is bigger than the max allowed ({}x{})",
            width, height, max_size, max_size
        ));
    }

    let bpp = bytes_per_pixel(&opts.format, &opts.internal_format);
    let pixels = (width * height) as usize * bpp;
    let data = vec![0; pixels];

    let texture = create_texture(gl, width, height, &data, bpp, &opts)?;

    let inner = InnerTexture {
        gl: gl.clone(),
        texture: Some(texture),
        width: width,
        height: height,
        internal_format: opts.internal_format,
        format: opts.format,
        min_filter: opts.min_filter,
        mag_filter: opts.mag_filter,
        buffer: vec![],
    };

    Ok(Texture {
        inner: Rc::new(RefCell::new(inner)),
        frame: None,
    })
}
