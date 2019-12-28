use crate::context::Context2d;
use crate::{GlContext, GlowValue, TextureKey};
use glow::HasContext;
use nae_core::resources::{BaseTexture, Resource, TextureFilter, TextureFormat};
use nae_core::{BaseApp, BaseSystem};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Texture {
    inner: Rc<RefCell<InnerTexture>>,
}

impl Texture {
    pub(crate) fn tex(&self) -> Option<TextureKey> {
        self.inner.borrow().tex
    }
}

impl BaseTexture for Texture {
    fn width(&self) -> f32 {
        self.inner.borrow().width as _
    }

    fn height(&self) -> f32 {
        self.inner.borrow().height as _
    }

    fn from_size<T, S>(app: &mut T, width: i32, height: i32) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Context2d>,
    {
        <Texture as BaseTexture>::from(
            app,
            width,
            height,
            TextureFormat::Rgba,
            TextureFormat::Rgba,
            TextureFilter::Nearest,
            TextureFilter::Nearest,
        )
    }

    fn from<T, S>(
        app: &mut T,
        width: i32,
        height: i32,
        internal_format: TextureFormat,
        format: TextureFormat,
        min_filter: TextureFilter,
        mag_filter: TextureFilter,
    ) -> Result<Self, String>
    where
        T: BaseApp<System = S>,
        S: BaseSystem<Context2d = Self::Context2d>,
    {
        texture_from_gl_context(
            &app.system().ctx2().gl,
            width,
            height,
            internal_format,
            format,
            min_filter,
            mag_filter,
        )
    }

    fn format(&self) -> TextureFormat {
        self.inner.borrow().format
    }
}

pub(crate) fn texture_from_gl_context(
    gl: &GlContext,
    width: i32,
    height: i32,
    internal_format: TextureFormat,
    format: TextureFormat,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
) -> Result<Texture, String> {
    let max_size = max_texture_size(gl);
    if width > max_size || height > max_size {
        return Err(format!(
            "Texture size {}x{} is bigger than the max allowed ({}x{})",
            width, height, max_size, max_size
        ));
    }

    let mut inner = InnerTexture::empty(width, height);
    let bpp = byte_per_pixel(internal_format, format);
    let tex = create_gl_tex_ext(
        gl,
        width,
        height,
        &vec![0; (width * height) as usize * bpp],
        internal_format.glow_value() as _,
        format.glow_value() as _,
        min_filter.glow_value() as _,
        mag_filter.glow_value() as _,
        bpp,
    )?;
    inner.gl = Some(gl.clone());
    inner.tex = Some(tex);
    Ok(Texture {
        inner: Rc::new(RefCell::new(inner)),
    })
}

impl Resource for Texture {
    type Context2d = Context2d;

    fn new(file: &str) -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerTexture::empty(1, 1))),
        }
    }

    fn parse<T: BaseSystem<Context2d = Self::Context2d>>(
        &mut self,
        app: &mut T,
        data: Vec<u8>,
    ) -> Result<(), String> {
        let data = image::load_from_memory(&data)
            .map_err(|e| e.to_string())?
            .to_rgba();

        let width = data.width() as _;
        let height = data.height() as _;
        let raw = data.to_vec();
        let gl = app.ctx2().gl.clone();
        let tex = create_gl_tex_ext(
            &gl,
            width,
            height,
            &raw,
            TextureFormat::Rgba.glow_value() as _,
            TextureFormat::Rgba.glow_value() as _,
            TextureFilter::Nearest.glow_value() as _,
            TextureFilter::Nearest.glow_value() as _,
            byte_per_pixel(TextureFormat::Rgba, TextureFormat::Rgba),
        )?;

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

struct InnerTexture {
    width: i32,
    height: i32,
    raw: Vec<u8>,
    gl: Option<GlContext>,
    tex: Option<TextureKey>,
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

//bytes_per_pixe table https://webgl2fundamentals.org/webgl/lessons/webgl-data-textures.html
fn byte_per_pixel(internal: TextureFormat, format: TextureFormat) -> usize {
    use TextureFormat::*;

    match (internal, format) {
        (R8, Red) => 1,
        _ => 4,
    }
}

fn create_gl_tex_ext(
    gl: &GlContext,
    width: i32,
    height: i32,
    data: &[u8],
    internal: i32,
    format: i32,
    min_filter: i32,
    mag_filter: i32,
    bytes_per_pixel: usize,
) -> Result<TextureKey, String> {
    unsafe {
        let tex = gl.create_texture()?;
        if bytes_per_pixel == 1 {
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
        }

        gl.bind_texture(glow::TEXTURE_2D, Some(tex));

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, mag_filter);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_filter);

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            internal,
            width,
            height,
            0,
            format as _,
            glow::UNSIGNED_BYTE,
            Some(data),
        );

        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(tex)
    }
}

impl GlowValue for TextureFilter {
    fn glow_value(&self) -> u32 {
        use TextureFilter::*;
        match self {
            Linear => glow::LINEAR,
            Nearest => glow::NEAREST,
        }
    }
}

impl GlowValue for TextureFormat {
    fn glow_value(&self) -> u32 {
        use TextureFormat::*;
        match self {
            Rgba => glow::RGBA,
            Red => glow::RED,
            R8 => glow::R8,
        }
    }
}

pub(crate) fn max_texture_size(gl: &GlContext) -> i32 {
    unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) }
}
