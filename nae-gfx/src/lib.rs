mod batchers;
mod buffers;
mod draw;
mod font;
mod matrix;
mod pipeline;
mod render_target;
mod shapes;
mod texture;
mod uniform;

pub use crate::shader::VertexFormat;
use crate::shader::{BufferKey, InnerShader, Shader};
pub use buffers::*;
pub use draw::*;
pub use font::*;
use glow::{Context, HasContext, DEPTH_TEST};
pub use matrix::*;
pub use pipeline::*;
pub use render_target::*;
pub use texture::*;
pub use uniform::*;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sdl")))]
use glutin::event::{Event, WindowEvent};

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sdl")))]
use glutin::event_loop::ControlFlow;

use nae_core::{
    BaseGfx, BaseIndexBuffer, BasePipeline, BaseVertexBuffer, BlendFactor, BlendMode,
    BlendOperation, ClearOptions, Color, CompareMode, CullMode, DrawUsage, GraphicsAPI,
    PipelineOptions, StencilAction, StencilOptions,
};

use std::cell::Ref;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

type VertexArray = <glow::Context as HasContext>::VertexArray;
type Program = <glow::Context as HasContext>::Program;
type TextureKey = <glow::Context as HasContext>::Texture;

#[derive(Debug)]
struct TextureData {
    width: i32,
    height: i32,
    data: Vec<u8>,
}

macro_rules! load_image {
    ($path:expr) => {{
        let bytes = include_bytes!($path);
        let data = image::load_from_memory(bytes).unwrap().to_rgba();

        TextureData {
            width: data.width() as _,
            height: data.height() as _,
            data: data.to_vec(),
        }
    }};
}

//Sample texture array limit https://stackoverflow.com/questions/20836102/how-many-textures-can-i-use-in-a-webgl-fragment-shader

//PORT OPENGL TUTORIALS TO NAE
//http://www.opengl-tutorial.org/beginners-tutorials/tutorial-4-a-colored-cube/
//https://github.com/bwasty/learn-opengl-rs

impl GlowValue for CullMode {
    type VALUE = Option<u32>;

    fn glow_value(&self) -> Self::VALUE {
        use CullMode::*;
        Some(match self {
            None => return Option::None,
            Front => glow::FRONT,
            Back => glow::BACK,
        })
    }
}

impl GlowValue for CompareMode {
    type VALUE = Option<u32>;

    fn glow_value(&self) -> Option<u32> {
        use CompareMode::*;
        Some(match self {
            None => return Option::None,
            Less => glow::LESS,
            Equal => glow::EQUAL,
            LEqual => glow::LEQUAL,
            Greater => glow::GREATER,
            NotEqual => glow::NOTEQUAL,
            GEqual => glow::GEQUAL,
            Always => glow::ALWAYS,
        })
    }
}

impl GlowValue for BlendFactor {
    type VALUE = u32;

    fn glow_value(&self) -> u32 {
        use BlendFactor::*;
        match self {
            Zero => glow::ZERO,
            One => glow::ONE,
            SourceAlpha => glow::SRC_ALPHA,
            SourceColor => glow::SRC_COLOR,
            InverseSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
            InverseSourceColor => glow::ONE_MINUS_SRC_COLOR,
            DestinationAlpha => glow::DST_ALPHA,
            DestinationColor => glow::SRC_COLOR,
            InverseDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
            InverseDestinationColor => glow::ONE_MINUS_DST_COLOR,
        }
    }
}

impl GlowValue for BlendOperation {
    type VALUE = u32;

    fn glow_value(&self) -> u32 {
        use BlendOperation::*;
        match self {
            Add => glow::FUNC_ADD,
            Subtract => glow::FUNC_SUBTRACT,
            ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            Max => glow::MAX,
            Min => glow::MIN,
        }
    }
}

impl GlowValue for StencilAction {
    type VALUE = u32;

    fn glow_value(&self) -> u32 {
        use StencilAction::*;
        match self {
            Keep => glow::KEEP,
            Zero => glow::ZERO,
            Replace => glow::REPLACE,
            Increment => glow::INCR,
            IncrementWrap => glow::INCR_WRAP,
            Decrement => glow::DECR,
            DecrementWrap => glow::DECR_WRAP,
            Invert => glow::INVERT,
        }
    }
}

mod shader;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sdl")))]
use glutin::{PossiblyCurrent, WindowedContext};

#[cfg(target_arch = "wasm32")]
type Device = web_sys::HtmlCanvasElement;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sdl")))]
type Device = WindowedContext<PossiblyCurrent>;

#[cfg(all(not(target_arch = "wasm32"), feature = "sdl"))]
type Device = sdl2::video::Window;

pub(crate) type GlContext = Rc<Context>;

pub struct Graphics {
    pub(crate) gl: GlContext,
    pub(crate) gfx_api: GraphicsAPI,
    index_type: u32,
    pipeline: Option<Pipeline>,
    indices_in_use: bool,
    width: f32,
    height: f32,
    running: bool,
    draw_calls: u32,
    last_pass_draw_calls: u32,
    render_target: Option<RenderTarget>,

    #[cfg(feature = "sdl")]
    _sdl_gl: Option<sdl2::video::GLContext>,
}

struct DeviceInfo {
    width: i32,
    height: i32,
    ctx: GlContext,
    api: GraphicsAPI,

    #[cfg(feature = "sdl")]
    _sdl_gl: Option<sdl2::video::GLContext>,
}

#[cfg(target_arch = "wasm32")]
fn create_gl_context(win: &web_sys::HtmlCanvasElement) -> Result<(GlContext, GraphicsAPI), String> {
    if let Ok(ctx) = create_webgl2_context(win) {
        return Ok((ctx, GraphicsAPI::WebGl2));
    }

    let ctx = create_webgl_context(win)?;
    Ok((ctx, GraphicsAPI::WebGl))
}

#[cfg(target_arch = "wasm32")]
fn webgl_options() -> web_sys::WebGlContextAttributes {
    let mut opts = web_sys::WebGlContextAttributes::new();
    opts.stencil(true);
    opts.premultiplied_alpha(false);
    opts.alpha(false);
    opts.antialias(true);
    opts
}

#[cfg(target_arch = "wasm32")]
fn create_webgl_context(win: &web_sys::HtmlCanvasElement) -> Result<GlContext, String> {
    //TODO manage errors
    let gl = win
        .get_context_with_context_options("webgl", webgl_options().as_ref())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .unwrap();

    //TODO call extensions here?

    let ctx = Rc::new(glow::Context::from_webgl1_context(gl));
    Ok(ctx)
}

#[cfg(target_arch = "wasm32")]
fn create_webgl2_context(win: &web_sys::HtmlCanvasElement) -> Result<GlContext, String> {
    //TODO manage errors
    let gl = win
        .get_context_with_context_options("webgl2", webgl_options().as_ref())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let ctx = Rc::new(glow::Context::from_webgl2_context(gl));
    Ok(ctx)
}

#[cfg(target_arch = "wasm32")]
fn get_device_info(win: &web_sys::HtmlCanvasElement) -> Result<DeviceInfo, String> {
    let width = win.width() as _;
    let height = win.height() as _;
    let (gl, api) = create_gl_context(win)?;
    Ok(DeviceInfo {
        width,
        height,
        ctx: gl,
        api,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn default_api() -> GraphicsAPI {
    if cfg!(target_os = "android") || cfg!(target_os = "ios") {
        return GraphicsAPI::OpenGlEs2_0;
    }

    GraphicsAPI::OpenGl3_3
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sdl")))]
fn get_device_info(device: &WindowedContext<PossiblyCurrent>) -> Result<DeviceInfo, String> {
    let win: &glutin::window::Window = device.window();
    let size = win.inner_size();
    let width = size.width as _;
    let height = size.height as _;
    let ctx = Rc::new(glow::Context::from_loader_function(|s| {
        device.get_proc_address(s) as *const _
    }));
    let api = default_api();
    Ok(DeviceInfo {
        width,
        height,
        ctx,
        api,
    })
}

#[cfg(all(not(target_arch = "wasm32"), feature = "sdl"))]
fn get_device_info(device: &sdl2::video::Window) -> Result<DeviceInfo, String> {
    let size = device.drawable_size();
    let width = size.0 as _;
    let height = size.1 as _;

    let gl_attr = device.subsystem().gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    if cfg!(target_os = "ios") || cfg!(target_os = "android") {
        gl_attr.set_context_version(2, 0);
    } else {
        gl_attr.set_context_version(3, 3);
    }

    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(8);

    let sdl_gl = device.gl_create_context()?;
    let ctx = Rc::new(glow::Context::from_loader_function(|s| {
        device.subsystem().gl_get_proc_address(s) as *const _
    }));

    let api = default_api();
    Ok(DeviceInfo {
        width,
        height,
        ctx,
        api,
        _sdl_gl: Some(sdl_gl),
    })
}

impl Graphics {
    pub fn new(device: &Device) -> Result<Self, String> {
        let info = get_device_info(device)?; //TODO return webgl driver
        let gl = info.ctx.clone();

        let index_type = match info.api {
            GraphicsAPI::WebGl => glow::UNSIGNED_SHORT,
            _ => glow::UNSIGNED_INT,
        };

        Ok(Self {
            gl,
            width: info.width as _,
            height: info.height as _,
            running: false,
            gfx_api: info.api,
            indices_in_use: false,
            draw_calls: 0,
            last_pass_draw_calls: 0,
            index_type,
            render_target: None,
            pipeline: None,

            #[cfg(feature = "sdl")]
            _sdl_gl: info._sdl_gl,
        })
    }

    pub fn bind_uniform(
        &mut self,
        location: &<Graphics as BaseGfx>::Location,
        value: &UniformValue<Graphics = Self>,
    ) {
        debug_assert!(
            self.pipeline.is_some(),
            "A pipeline should be set before bind uniforms"
        );
        value.bind_uniform(self, location.clone());
    }

    pub fn draw_calls(&self) -> u32 {
        self.last_pass_draw_calls
    }

    pub fn begin_to(&mut self, target: Option<&RenderTarget>, opts: &ClearOptions) {
        debug_assert!(!self.running, "Graphics pass already running.");

        self.running = true;

        unsafe {
            let (width, height) = match target {
                Some(rt) => {
                    let needs_update = match &self.render_target {
                        Some(current_rt) => current_rt.raw != rt.raw,
                        None => true,
                    };

                    if needs_update {
                        self.render_target = Some(rt.clone());
                    }

                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(rt.raw));
                    self.gl.draw_buffer(glow::COLOR_ATTACHMENT0);
                    (rt.width(), rt.height())
                }
                None => {
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    (self.width, self.height)
                }
            };

            self.viewport(0.0, 0.0, width, height);
            self.clear(opts);
        }
    }

    pub fn clear(&mut self, opts: &ClearOptions) {
        let mut mask = 0;
        unsafe {
            if let Some(color) = &opts.color {
                mask |= glow::COLOR_BUFFER_BIT;
                self.gl.clear_color(color.r, color.g, color.b, color.a);
            }

            if let Some(depth) = opts.depth {
                mask |= glow::DEPTH_BUFFER_BIT;
                self.gl.enable(glow::DEPTH_TEST);
                self.gl.depth_mask(true);
                self.gl.clear_depth_f32(depth);
            }

            if let Some(stencil) = opts.stencil {
                mask |= glow::STENCIL_BUFFER_BIT;
                self.gl.enable(glow::STENCIL_TEST);
                self.gl.stencil_mask(0xff);
                self.gl.clear_stencil(stencil);
            }

            self.gl.clear(mask);
        }
    }
}

impl BaseGfx for Graphics {
    type Location = Uniform;
    type Texture = texture::Texture;
    type Pipeline = Pipeline;

    fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    fn api(&self) -> GraphicsAPI {
        self.gfx_api.clone()
    }

    fn viewport(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            self.gl
                .viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    fn begin(&mut self, opts: &ClearOptions) {
        self.begin_to(None, opts);
    }

    fn bind_texture(&mut self, location: &Self::Location, tex: &texture::Texture) {
        self.bind_texture_slot(0, location, tex);
    }

    fn bind_texture_slot(&mut self, slot: u32, location: &Self::Location, tex: &texture::Texture) {
        debug_assert!(
            self.pipeline.is_some(),
            "A pipeline should be set before bind textures"
        );

        unsafe {
            let gl_slot = match slot {
                0 => glow::TEXTURE0,
                1 => glow::TEXTURE1,
                2 => glow::TEXTURE2,
                3 => glow::TEXTURE3,
                4 => glow::TEXTURE4,
                5 => glow::TEXTURE5,
                6 => glow::TEXTURE6,
                7 => glow::TEXTURE7,
                _ => panic!("invalid texture slot"),
            };

            self.gl.active_texture(gl_slot);
            self.gl.bind_texture(glow::TEXTURE_2D, tex.raw());
            self.bind_uniform(location, &(slot as i32));
        }
    }

    fn end(&mut self) {
        debug_assert!(self.running, "Begin should be called first.");

        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        self.indices_in_use = false;
        self.pipeline = None;
        self.running = false;

        self.last_pass_draw_calls = self.draw_calls;
        self.draw_calls = 0;
    }

    fn set_pipeline(&mut self, pipeline: &Self::Pipeline) {
        self.pipeline = Some(pipeline.clone());
        pipeline.bind(self);
        self.indices_in_use = false;
    }

    fn bind_vertex_buffer(&mut self, buffer: &BaseVertexBuffer<Graphics = Self>, data: &[f32]) {
        debug_assert!(
            self.pipeline.is_some(),
            "A pipeline should be set before bind the vertex buffer"
        );
        buffer.bind(self, data);
    }

    fn bind_index_buffer(&mut self, buffer: &BaseIndexBuffer<Graphics = Self>, data: &[u32]) {
        debug_assert!(
            self.pipeline.is_some(),
            "A pipeline should be set before bind the vertex buffer"
        );
        buffer.bind(self, data);
        self.indices_in_use = true;
    }

    fn draw(&mut self, offset: i32, count: i32) {
        debug_assert!(
            self.pipeline.is_some(),
            "A pipeline should be set before draw"
        );
        // TODO draw instanced?

        unsafe {
            if self.indices_in_use {
                self.gl
                    .draw_elements(glow::TRIANGLES, count, self.index_type, offset * 4);
            } else {
                self.gl.draw_arrays(glow::TRIANGLES, offset, count);
            }
        }

        self.draw_calls += 1;
    }
}

impl GlowValue for DrawUsage {
    type VALUE = u32;

    fn glow_value(&self) -> u32 {
        match self {
            DrawUsage::Static => glow::STATIC_DRAW,
            DrawUsage::Dynamic => glow::DYNAMIC_DRAW,
        }
    }
}

pub struct VertexAttr {
    pub location: u32,
    pub format: VertexFormat,
}

impl VertexAttr {
    pub fn new(location: u32, vertex_data: VertexFormat) -> Self {
        Self {
            location: location,
            format: vertex_data,
        }
    }
}

pub trait AttrLocationId {
    fn location(&self, pipeline: &Pipeline) -> u32;
}

impl AttrLocationId for u32 {
    fn location(&self, pipeline: &Pipeline) -> u32 {
        *self
    }
}

impl AttrLocationId for String {
    fn location(&self, pipeline: &Pipeline) -> u32 {
        unsafe {
            pipeline
                .shader
                .inner
                .gl
                .get_attrib_location(pipeline.shader.inner.raw, &self)
                .expect("Invalid location") as u32
        }
    }
}

fn create_gl_tex_ext(
    gfx: &Graphics,
    image: TextureData,
    internal: i32,
    format: i32,
    min_filter: i32,
    mag_filter: i32,
    bytes_per_pixel: usize,
) -> Result<TextureKey, String> {
    unsafe {
        let gl = &gfx.gl;
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
            image.width,
            image.height,
            0,
            format as _,
            glow::UNSIGNED_BYTE,
            Some(&image.data),
        );

        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(tex)
    }
}

pub trait GlowValue {
    type VALUE;
    fn glow_value(&self) -> Self::VALUE;
}
