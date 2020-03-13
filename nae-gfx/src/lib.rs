use crate::shader::{BufferKey, InnerShader};
pub use crate::shader::{Shader, VertexFormat};
use glow::{Context, HasContext, DEPTH_TEST};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use nae_core::{
    BaseGfx, BaseIndexBuffer, BasePipeline, BaseVertexBuffer, BlendFactor, BlendMode,
    BlendOperation, ClearOptions, Color, CullMode, DepthStencil, DrawUsage, GraphicsAPI,
    PipelineOptions,
};
use std::cell::Ref;
use std::rc::Rc;
use ultraviolet::mat::Mat4;
use ultraviolet::projection::perspective_gl as perspective;
use ultraviolet::vec::Vec3;

type VertexArray = <glow::Context as HasContext>::VertexArray;
type Program = <glow::Context as HasContext>::Program;

// TODO delete on drop opengl allocations
// Shader should got app or gfx as first parameter?

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

impl GlowValue for DepthStencil {
    type VALUE = Option<u32>;

    fn glow_value(&self) -> Option<u32> {
        use DepthStencil::*;
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

mod shader;
fn mat4_to_slice(m: &ultraviolet::mat::Mat4) -> *const [f32; 16] {
    m.as_slice().as_ptr() as *const [f32; 16]
}

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
    pipeline_in_use: bool,
    indices_in_use: bool,
    width: f32,
    height: f32,
    running: bool,

    #[cfg(feature = "sdl")]
    _sdl_gl: Option<sdl2::video::GLContext>,
}

struct DeviceInfo {
    width: i32,
    height: i32,
    ctx: glow::Context,

    #[cfg(feature = "sdl")]
    _sdl_gl: Option<sdl2::video::GLContext>,
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sdl")))]
fn get_device_info(device: &WindowedContext<PossiblyCurrent>) -> DeviceInfo {
    let win: &glutin::window::Window = device.window();
    let size = win.inner_size();
    let width = size.width as _;
    let height = size.height as _;
    let ctx = glow::Context::from_loader_function(|s| device.get_proc_address(s) as *const _);
    DeviceInfo { width, height, ctx }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "sdl"))]
fn get_device_info(device: &sdl2::video::Window) -> DeviceInfo {
    let size = device.size();
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
    let ctx = glow::Context::from_loader_function(|s| {
        device.subsystem().gl_get_proc_address(s) as *const _
    });

    DeviceInfo {
        width,
        height,
        ctx,
        _sdl_gl: Some(sdl_gl),
    }
}

impl Graphics {
    pub fn new(device: &Device) -> Result<Self, String> {
        let info = get_device_info(device);
        let gl = Rc::new(info.ctx);
        Ok(Self {
            gl,
            width: info.width as _,
            height: info.height as _,
            running: false,
            gfx_api: GraphicsAPI::OpenGl3_3,
            pipeline_in_use: false,
            indices_in_use: false,

            #[cfg(feature = "sdl")]
            _sdl_gl: Some(info._sdl_gl),
        })
    }

    fn bind_uniform(&mut self, location: u32, value: &UniformValue<Graphics = Self>) {
        debug_assert!(
            self.pipeline_in_use,
            "A pipeline should be set before bind uniforms"
        );
        value.bind_uniform(self, location);
    }
}

impl BaseGfx for Graphics {
    type Location = u32;
    type Texture = u32;

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
        debug_assert!(!self.running, "Graphics pass already running.");

        self.running = true;
        self.viewport(0.0, 0.0, self.width, self.height);

        let mut mask = 0;
        unsafe {
            if let Some(color) = &opts.color {
                mask |= glow::COLOR_BUFFER_BIT;
                self.gl
                    .clear_color(color.red(), color.green(), color.blue(), color.alpha());
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

    fn bind_texture(&mut self, location: u32, tex: u32) {
        self.bind_texture_slot(0, location, tex);
    }

    fn bind_texture_slot(&mut self, slot: u32, location: u32, tex: u32) {
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
            self.gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            self.bind_uniform(location, &(slot as i32));
        }
    }

    fn end(&mut self) {
        debug_assert!(self.running, "Begin should be called first.");

        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);
        }
        self.indices_in_use = false;
        self.pipeline_in_use = false;
        self.running = false;
    }

    fn set_pipeline(&mut self, pipeline: &BasePipeline<Graphics = Self>) {
        pipeline.bind(self);
        self.pipeline_in_use = true;
        self.indices_in_use = false;
    }

    fn bind_vertex_buffer(&mut self, buffer: &BaseVertexBuffer<Graphics = Self>, data: &[f32]) {
        debug_assert!(
            self.pipeline_in_use,
            "A pipeline should be set before bind the vertex buffer"
        );
        buffer.bind(self, data);
    }

    fn bind_index_buffer(&mut self, buffer: &BaseIndexBuffer<Graphics = Self>, data: &[u32]) {
        debug_assert!(
            self.pipeline_in_use,
            "A pipeline should be set before bind the vertex buffer"
        );
        buffer.bind(self, data);
        self.indices_in_use = true;
    }

    fn draw(&mut self, offset: i32, count: i32) {
        debug_assert!(self.pipeline_in_use, "A pipeline should be set before draw");
        // TODO draw instanced?
        unsafe {
            if self.indices_in_use {
                self.gl
                    .draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, offset * 4);
            } else {
                self.gl.draw_arrays(glow::TRIANGLES, offset, count);
            }
        }
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

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

fn vfi_to_u8(v: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub struct IndexBuffer {
    buffer: BufferKey,
    usage: DrawUsage,
}

impl IndexBuffer {
    pub fn new(graphics: &Graphics, usage: DrawUsage) -> Result<Self, String> {
        unsafe {
            let gl = &graphics.gl;
            let buffer = gl.create_buffer()?;
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer));
            Ok(Self { buffer, usage })
        }
    }
}

impl BaseIndexBuffer for IndexBuffer {
    type Graphics = Graphics;

    fn bind(&self, gfx: &mut Graphics, indices: &[u32]) {
        let gl = &gfx.gl;

        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                vfi_to_u8(&indices),
                self.usage.glow_value(),
            );
        }
    }
}

pub struct VertexBuffer {
    buffer: BufferKey,
    usage: DrawUsage,
}

impl VertexBuffer {
    pub fn new(
        graphics: &Graphics,
        attributes: &[VertexAttr],
        usage: DrawUsage,
    ) -> Result<Self, String> {
        unsafe {
            let gl = &graphics.gl;
            let buffer = gl.create_buffer()?;
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));

            let stride = attributes
                .iter()
                .fold(0, |acc, data| acc + data.format.bytes());

            let mut offset = 0;
            for attr in attributes {
                let location = attr.location;
                let size = attr.format.size();
                let data_type = attr.format.glow_value();
                let normalized = attr.format.normalized();

                gl.enable_vertex_attrib_array(location);
                gl.vertex_attrib_pointer_f32(location, size, data_type, normalized, stride, offset);

                offset += attr.format.bytes();
            }

            Ok(VertexBuffer { buffer, usage })
        }
    }
}

impl BaseVertexBuffer for VertexBuffer {
    type Graphics = Graphics;

    fn bind(&self, gfx: &mut Graphics, data: &[f32]) {
        let gl = &gfx.gl;
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vf_to_u8(data), self.usage.glow_value());
        }
    }
}

pub struct Draw2d<'gfx> {
    gfx: Ref<'gfx, Graphics>,
}

pub trait AttrLocationId {
    fn location(&self, shader: &Shader) -> u32;
}

impl AttrLocationId for u32 {
    fn location(&self, shader: &Shader) -> u32 {
        *self
    }
}

impl AttrLocationId for String {
    fn location(&self, shader: &Shader) -> u32 {
        unsafe {
            shader
                .inner
                .gl
                .get_attrib_location(shader.inner.raw, &self)
                .expect("Invalid location") as u32
        }
    }
}

pub trait UniformValue {
    type Graphics: BaseGfx;
    fn bind_uniform(&self, gfx: &Self::Graphics, location: <Self::Graphics as BaseGfx>::Location);
}

impl UniformValue for i32 {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics.gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformValue for f32 {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics.gl.uniform_1_f32(Some(location), *self);
        }
    }
}

impl UniformValue for [f32; 2] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics.gl.uniform_2_f32(Some(location), self[0], self[1]);
        }
    }
}

impl UniformValue for [f32; 3] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics
                .gl
                .uniform_3_f32(Some(location), self[0], self[1], self[2]);
        }
    }
}

impl UniformValue for [f32; 4] {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics
                .gl
                .uniform_4_f32(Some(location), self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformValue for ultraviolet::mat::Mat4 {
    type Graphics = Graphics;

    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        let matrix = self.as_slice().as_ptr() as *const [f32; 16];
        unsafe {
            graphics
                .gl
                .uniform_matrix_4_f32_slice(Some(location), false, &*matrix);
        }
    }
}

pub struct Texture {}

pub struct RenderTarget {}

// TODO uniform value for matrix values

// fn main2() {
//     let (gl, event_loop, windowed_context, shader_version) = unsafe {
//         let el = glutin::event_loop::EventLoop::new();
//         let wb = glutin::window::WindowBuilder::new()
//             .with_title("Hello triangle!")
//             .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
//         let windowed_context = glutin::ContextBuilder::new()
//             .with_vsync(true)
//             .with_gl(glutin::GlRequest::GlThenGles {
//                 opengl_version: (3, 3),
//                 opengles_version: (2, 0),
//             })
//             .with_depth_buffer(24)
//             .with_stencil_buffer(8)
//             .with_gl_profile(glutin::GlProfile::Core)
//             .build_windowed(wb, &el)
//             .unwrap();
//         let windowed_context = windowed_context.make_current().unwrap();
//         let context = glow::Context::from_loader_function(|s| {
//             windowed_context.get_proc_address(s) as *const _
//         });
//         (context, el, windowed_context, "#version 410")
//     };
//
//     let gl = Rc::new(gl);
//
//     let mut gfx = Graphics::new(gl, 1024.0, 768.0);
//     // let mut cube = Cube::new(&mut gfx);
//     let mut triangle = Triangle::new(&mut gfx);
//     let mut textured_cube = TexturedCube::new(&mut gfx);
//
//     let clear_options = ClearOptions::new(Color::new(0.1, 0.2, 0.3, 1.0));
//
//     unsafe {
//         event_loop.run(move |event, _, control_flow| {
//             *control_flow = ControlFlow::Poll;
//             match event {
//                 Event::LoopDestroyed => {
//                     return;
//                 }
//                 Event::MainEventsCleared => {
//                     windowed_context.window().request_redraw();
//                 }
//                 Event::RedrawRequested(_) => {
//                     gfx.begin(&clear_options);
//                     textured_cube.draw(&mut gfx);
//                     // cube.draw(&mut gfx);
//                     triangle.draw(&mut gfx);
//                     gfx.end();
//
//                     windowed_context.swap_buffers().unwrap();
//                 }
//                 Event::WindowEvent { ref event, .. } => match event {
//                     WindowEvent::Resized(physical_size) => {
//                         windowed_context.resize(*physical_size);
//                     }
//                     WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
//                     _ => (),
//                 },
//                 _ => (),
//             }
//         });
//     }
// }

struct Triangle {
    pipeline: Pipeline,
    vertices: [f32; 21],
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    mvp: Mat4,
    mvp_loc: u32,
}

impl Triangle {
    fn new(gfx: &mut Graphics) -> Self {
        let shader = Shader::new(
            gfx,
            include_bytes!("../resources/shaders/color.vert.spv"),
            include_bytes!("../resources/shaders/color.frag.spv"),
        )
        .unwrap();

        let pipeline = Pipeline::new(
            gfx,
            &shader,
            PipelineOptions {
                ..Default::default()
            },
        );
        let mvp_loc = pipeline.uniform_location("u_matrix");

        let vertex_buffer = VertexBuffer::new(
            gfx,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            DrawUsage::Dynamic,
        )
        .unwrap();

        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic).unwrap();

        #[rustfmt::skip]
        let vertices = [
            -0.1, -0.1, 0.0,    1.0, 0.2, 0.3, 1.0,
            0.1, -0.1, 0.0,     0.1, 1.0, 0.3, 1.0,
            0.0, 0.1, 0.0,      0.1, 0.2, 1.0, 1.0,
        ];

        let mvp = Mat4::identity();

        Self {
            pipeline,
            vertices,
            vertex_buffer,
            index_buffer,
            mvp,
            mvp_loc,
        }
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        gfx.set_pipeline(&self.pipeline);
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &[0, 1, 2]);
        gfx.bind_uniform(self.mvp_loc, &self.mvp);
        gfx.draw(0, 3);
    }
}

struct Cube {
    shader: Shader,
    vertices: [f32; 168],
    indices: [u32; 36],
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    rotation: (f32, f32),
    mvp: Mat4,
}

impl Cube {
    fn new(gfx: &mut Graphics) -> Self {
        let shader = Shader::new(
            gfx,
            include_bytes!("../resources/shaders/color.vert.spv"),
            include_bytes!("../resources/shaders/color.frag.spv"),
        )
        .unwrap();

        let vertex_buffer = VertexBuffer::new(
            gfx,
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            DrawUsage::Dynamic,
        )
        .unwrap();

        let index_buffer = IndexBuffer::new(gfx, DrawUsage::Dynamic).unwrap();

        #[rustfmt::skip]
        let vertices= [
            -1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
            1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
            1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
            -1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,

            -1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
            1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
            1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
            -1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,

            -1.0, -1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
            -1.0,  1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
            -1.0,  1.0,  1.0,   0.0, 0.0, 1.0, 1.0,
            -1.0, -1.0,  1.0,   0.0, 0.0, 1.0, 1.0,

            1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
            1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
            1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,
            1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,

            -1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,
            -1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
            1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
            1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,

            -1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
            -1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
            1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
            1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
        ];

        #[rustfmt::skip]
        let indices = [
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];

        let projection: Mat4 = perspective(45.0, 4.0 / 3.0, 0.1, 100.0);
        let view = Mat4::look_at(
            Vec3::new(4.0, 3.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let mut mvp: Mat4 = Mat4::identity();
        mvp = mvp * projection;
        mvp = mvp * view;

        Self {
            shader,
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            rotation: (0.0, 0.0),
            mvp,
        }
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        let (ref mut rx, ref mut ry) = self.rotation;

        *rx += 0.01;
        *ry += 0.01;

        let rxm = Mat4::from_rotation_x(*rx);
        let rym = Mat4::from_rotation_y(*ry);
        let model = rxm * rym;
        let mvp = self.mvp * model;

        // gfx.use_shader(&self.shader);
        gfx.bind_uniform(0, &mvp);
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &self.indices);
        gfx.draw(0, self.indices.len() as i32);
    }
}

struct TexturedCube {
    vertices: [f32; 108],
    uvs: [f32; 72],
    vertex_buffer: VertexBuffer,
    uvs_buffer: VertexBuffer,
    rotation: (f32, f32),
    mvp: Mat4,
    tex: u32,
    pipeline: Pipeline,
    mvp_loc: u32,
    tex_loc: u32,
}

impl TexturedCube {
    fn new(gfx: &mut Graphics) -> Self {
        let shader = Shader::new(
            gfx,
            include_bytes!("../resources/shaders/textured.vert.spv"),
            include_bytes!("../resources/shaders/textured.frag.spv"),
        )
        .unwrap();

        let pipeline = Pipeline::new(
            &gfx,
            &shader,
            PipelineOptions {
                // color_blend: Some(BlendMode::ERASE),
                cull_mode: CullMode::Back,
                ..Default::default()
            },
        );
        let mvp_loc = pipeline.uniform_location("u_matrix");
        let tex_loc = pipeline.uniform_location("u_texture");

        let vertex_buffer = VertexBuffer::new(
            gfx,
            &[VertexAttr::new(0, VertexFormat::Float3)],
            DrawUsage::Dynamic,
        )
        .unwrap();

        let uvs_buffer = VertexBuffer::new(
            gfx,
            &[VertexAttr::new(1, VertexFormat::Float2)],
            DrawUsage::Dynamic,
        )
        .unwrap();

        #[rustfmt::skip]
        let uvs = [
            0.000059, 0.000004,
            0.000103, 0.336048,
            0.335973, 0.335903,
            1.000023, 0.000013,
            0.667979, 0.335851,
            0.999958, 0.336064,
            0.667979, 0.335851,
            0.336024, 0.671877,
            0.667969, 0.671889,
            1.000023, 0.000013,
            0.668104, 0.000013,
            0.667979, 0.335851,
            0.000059, 0.000004,
            0.335973, 0.335903,
            0.336098, 0.000071,
            0.667979, 0.335851,
            0.335973, 0.335903,
            0.336024, 0.671877,
            1.000004, 0.671847,
            0.999958, 0.336064,
            0.667979, 0.335851,
            0.668104, 0.000013,
            0.335973, 0.335903,
            0.667979, 0.335851,
            0.335973, 0.335903,
            0.668104, 0.000013,
            0.336098, 0.000071,
            0.000103, 0.336048,
            0.000004, 0.671870,
            0.336024, 0.671877,
            0.000103, 0.336048,
            0.336024, 0.671877,
            0.335973, 0.335903,
            0.667969, 0.671889,
            1.000004, 0.671847,
            0.667979, 0.335851
        ];

        #[rustfmt::skip]
        let vertices= [
            -1.0,-1.0,-1.0,
            -1.0,-1.0, 1.0,
            -1.0, 1.0, 1.0,
            1.0, 1.0,-1.0,
            -1.0,-1.0,-1.0,
            -1.0, 1.0,-1.0,
            1.0,-1.0, 1.0,
            -1.0,-1.0,-1.0,
            1.0,-1.0,-1.0,
            1.0, 1.0,-1.0,
            1.0,-1.0,-1.0,
            -1.0,-1.0,-1.0,
            -1.0,-1.0,-1.0,
            -1.0, 1.0, 1.0,
            -1.0, 1.0,-1.0,
            1.0,-1.0, 1.0,
            -1.0,-1.0, 1.0,
            -1.0,-1.0,-1.0,
            -1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0,
            1.0,-1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0,-1.0,-1.0,
            1.0, 1.0,-1.0,
            1.0,-1.0,-1.0,
            1.0, 1.0, 1.0,
            1.0,-1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0,-1.0,
            -1.0, 1.0,-1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0,-1.0,
            -1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0,
            1.0,-1.0, 1.0
        ];

        let projection: Mat4 = perspective(45.0, 4.0 / 3.0, 0.1, 100.0);
        let view = Mat4::look_at(
            Vec3::new(4.0, 3.0, -3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let mut mvp: Mat4 = Mat4::identity();
        mvp = mvp * projection;
        mvp = mvp * view;

        let image = load_image!("../resources/cube.png");
        let tex = create_gl_tex_ext(
            &gfx,
            image,
            glow::RGBA as _,
            glow::RGBA as _,
            glow::NEAREST as _,
            glow::NEAREST as _,
            4,
        )
        .unwrap();

        Self {
            pipeline,
            vertices,
            vertex_buffer,
            uvs_buffer,
            rotation: (0.0, 0.0),
            mvp,
            uvs,
            tex,
            mvp_loc,
            tex_loc,
        }
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        let (ref mut rx, ref mut ry) = self.rotation;

        *rx += 0.01;
        *ry += 0.01;

        let rxm = Mat4::from_rotation_x(-*rx);
        let rym = Mat4::from_rotation_y(-*ry);
        let model = rxm * rym;
        let mvp: Mat4 = self.mvp * model;

        gfx.set_pipeline(&self.pipeline);
        gfx.bind_uniform(self.mvp_loc, &mvp);
        gfx.bind_texture(self.tex_loc, self.tex);
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_vertex_buffer(&self.uvs_buffer, &self.uvs);
        gfx.draw(0, (self.vertices.len() / 3) as i32);
    }
}

#[derive(Clone)]
pub struct Pipeline {
    gl: GlContext,
    vao: <glow::Context as HasContext>::VertexArray,
    shader: Shader,
    pub options: PipelineOptions,
}

impl Pipeline {
    pub fn new(graphics: &Graphics, shader: &Shader, opts: PipelineOptions) -> Self {
        let gl = graphics.gl.clone();
        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            vao
        };
        Self {
            gl,
            vao,
            options: opts,
            shader: shader.clone(),
        }
    }
}

impl BasePipeline for Pipeline {
    type Graphics = Graphics;

    fn bind(&self, gfx: &mut Self::Graphics) {
        unsafe {
            gfx.gl.stencil_mask(0x00); //TODO

            if let Some(d) = self.options.depth_stencil.glow_value() {
                gfx.gl.enable(glow::DEPTH_TEST);
                gfx.gl.depth_func(d);
            } else {
                gfx.gl.disable(glow::DEPTH_TEST);
            }

            if let Some(mode) = self.options.cull_mode.glow_value() {
                gfx.gl.enable(glow::CULL_FACE);
                gfx.gl.cull_face(mode);
            } else {
                gfx.gl.disable(glow::CULL_FACE);
            }

            match (self.options.color_blend, self.options.alpha_blend) {
                (Some(cbm), None) => {
                    gfx.gl.enable(glow::BLEND);
                    gfx.gl
                        .blend_func(cbm.src.glow_value(), cbm.dst.glow_value());
                    gfx.gl.blend_equation(cbm.op.glow_value());
                }
                (Some(cbm), Some(abm)) => {
                    gfx.gl.enable(glow::BLEND);
                    gfx.gl.blend_func_separate(
                        cbm.src.glow_value(),
                        cbm.dst.glow_value(),
                        abm.src.glow_value(),
                        abm.dst.glow_value(),
                    );
                    gfx.gl
                        .blend_equation_separate(cbm.op.glow_value(), abm.op.glow_value());
                }
                (None, Some(abm)) => {
                    let cbm = BlendMode::NORMAL;
                    gfx.gl.enable(glow::BLEND);
                    gfx.gl.blend_func_separate(
                        cbm.src.glow_value(),
                        cbm.dst.glow_value(),
                        abm.src.glow_value(),
                        abm.dst.glow_value(),
                    );
                    gfx.gl
                        .blend_equation_separate(cbm.op.glow_value(), abm.op.glow_value());
                }
                (None, None) => {
                    gfx.gl.disable(glow::BLEND);
                }
            }

            gfx.gl.bind_vertex_array(Some(self.vao));
            gfx.gl.use_program(Some(self.shader.inner.raw));
        }
    }

    fn options(&mut self) -> &mut PipelineOptions {
        &mut self.options
    }

    fn uniform_location(&self, id: &str) -> u32 {
        unsafe {
            self.gl
                .get_uniform_location(self.shader.inner.raw, id)
                .unwrap()
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
) -> Result<u32, String> {
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

