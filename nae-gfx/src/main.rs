use crate::shader::{BufferKey, Driver, GlowValue, Shader, VertexFormat};
use glow::{Context, HasContext};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use nae_core::math::{identity, scaling2d, vec2, Mat3};
use nae_core::{BlendFactor, BlendMode, Color};
use std::cell::Ref;
use std::rc::Rc;
//Sample texture array limit https://stackoverflow.com/questions/20836102/how-many-textures-can-i-use-in-a-webgl-fragment-shader

mod shader;
//https://github.com/glium/glium/blob/master/examples/triangle.rs

pub(crate) type GlContext = Rc<Context>;

pub trait BaseGraphics {
    fn viewport(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn flush(&mut self);
    fn begin(&mut self);
    fn end(&mut self);
    fn clear(color: Option<Color>, depth: Option<f32>, stencil: Option<i32>);
    // etc...
}

pub struct Graphics {
    pub(crate) gl: GlContext,
    pub(crate) driver: Driver,
    vao: <glow::Context as HasContext>::VertexArray,
}

impl Graphics {
    pub fn new(gl: GlContext) -> Self {
        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            vao
        };

        Self {
            gl,
            driver: Driver::OpenGl3_3,
            vao,
        }
    }

    pub fn begin(&mut self) {}

    pub fn end(&mut self) {}

    pub fn use_shader(&mut self, shader: &Shader) {
        unsafe {
            self.gl.use_program(Some(shader.program));
        }
    }

    pub fn clear(&mut self, color: Option<[f32; 4]>) {
        unsafe {
            if let Some([r, g, b, a]) = color {
                self.gl.clear_color(r, g, b, a);
            }
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn bind_vertex_buffer(&mut self, buffer: &VertexBuffer, data: &[f32]) {
        buffer.bind(&self.gl, data);
    }

    pub fn bind_index_buffer(&mut self, buffer: &IndexBuffer, data: &[u32]) {
        buffer.bind(&self.gl, data);
    }

    pub fn draw(&mut self, offset: u32, count: u32) {
        // TODO draw arrays if doesn't exists index_buffer
        unsafe {
            self.gl.draw_elements(
                glow::TRIANGLES,
                count as i32,
                glow::UNSIGNED_INT,
                offset as i32,
            );
        }
    }

    pub fn bind_uniform(&mut self, location: u32, value: &UniformValue) {
        value.bind_uniform(&self, location);
    }
}

pub enum Usage {
    Static,
    Dynamic,
}

impl GlowValue for Usage {
    fn glow_value(&self) -> u32 {
        match self {
            Usage::Static => glow::STATIC_DRAW,
            Usage::Dynamic => glow::DYNAMIC_DRAW,
        }
    }
}

pub struct VertexAttr {
    pub location: u32,
    pub format: VertexFormat,
}

impl VertexAttr {
    fn new(location: u32, vertex_data: VertexFormat) -> Self {
        Self {
            location: location,
            format: vertex_data,
        }
    }
}

fn m3_to_slice(m: &Mat3) -> *const [f32; 9] {
    m.as_slice().as_ptr() as *const [f32; 9]
}

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

fn vfi_to_u8(v: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub struct IndexBuffer {
    buffer: BufferKey,
    usage: Usage,
}

impl IndexBuffer {
    fn new(graphics: &Graphics, usage: Usage) -> Result<Self, String> {
        unsafe {
            let gl = &graphics.gl;
            let buffer = gl.create_buffer()?;
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer));
            Ok(Self { buffer, usage })
        }
    }

    fn bind(&self, gl: &GlContext, indices: &[u32]) {
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
    usage: Usage,
}

impl VertexBuffer {
    pub fn new(
        graphics: &Graphics,
        attributes: &[VertexAttr],
        usage: Usage,
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

    fn bind(&self, gl: &GlContext, data: &[f32]) {
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
                .gl
                .get_attrib_location(shader.program, &self)
                .expect("Invalid location") as u32
        }
    }
}

pub trait UniformValue {
    fn bind_uniform(&self, graphics: &Graphics, location: u32);
}

impl UniformValue for i32 {
    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics.gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformValue for f32 {
    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics.gl.uniform_1_f32(Some(location), *self);
        }
    }
}

impl UniformValue for [f32; 2] {
    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics.gl.uniform_2_f32(Some(location), self[0], self[1]);
        }
    }
}

impl UniformValue for [f32; 3] {
    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics
                .gl
                .uniform_3_f32(Some(location), self[0], self[1], self[2]);
        }
    }
}

impl UniformValue for [f32; 4] {
    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        unsafe {
            graphics
                .gl
                .uniform_4_f32(Some(location), self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformValue for Mat3 {
    fn bind_uniform(&self, graphics: &Graphics, location: u32) {
        let matrix = self.as_slice().as_ptr() as *const [f32; 9];
        unsafe {
            graphics
                .gl
                .uniform_matrix_3_f32_slice(Some(location), false, &*matrix);
        }
    }
}

// TODO uniform value for matrix values

fn main() {
    let (gl, event_loop, windowed_context, shader_version) = unsafe {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 3),
                opengles_version: (2, 0),
            })
            .with_gl_profile(glutin::GlProfile::Core)
            .build_windowed(wb, &el)
            .unwrap();
        let windowed_context = windowed_context.make_current().unwrap();
        let context = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });
        (context, el, windowed_context, "#version 410")
    };

    let gl = Rc::new(gl);

    let mut gfx = Graphics::new(gl);

    let shader = Shader::new(
        &gfx,
        include_bytes!("../resources/shaders/color.vert.spv"),
        include_bytes!("../resources/shaders/color.frag.spv"),
    )
    .unwrap();

    let buffer = VertexBuffer::new(
        &gfx,
        &[
            VertexAttr::new(0, VertexFormat::Float2),
            VertexAttr::new(1, VertexFormat::Float4),
        ],
        Usage::Dynamic,
    )
    .unwrap();

    let index_buffer = IndexBuffer::new(&gfx, Usage::Dynamic).unwrap();

    #[rustfmt::skip]
    let vertices = [
        // position     //color
        0.5, 1.0,       0.5, 1.0, 0.0, 1.0,
        0.0, 0.0,       0.0, 0.0, 0.4, 1.0,
        1.0, 0.0,       1.0, 0.0, 0.6, 1.0,
        1.5, 1.0,       1.0, 0.5, 1.0, 1.0,
    ];

    let indices = [0, 1, 2, 0, 2, 3];

    let identity: Mat3 = identity();
    let mm = identity;
    println!("identity {:?}", identity);

    unsafe {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    windowed_context.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    gfx.begin();
                    gfx.clear(Some([0.1, 0.2, 0.4, 1.0]));
                    gfx.use_shader(&shader);
                    gfx.bind_vertex_buffer(&buffer, &vertices);
                    gfx.bind_index_buffer(&index_buffer, &indices);
                    gfx.bind_uniform(0, &mm);
                    gfx.draw(0, 6);
                    gfx.end();

                    windowed_context.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed_context.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
