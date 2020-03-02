use crate::shader::{BufferKey, Driver, GlowValue, Shader, VertexFormat};
use glow::{Context, HasContext};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use nae_core::{BlendFactor, BlendMode, Color};
use std::cell::Ref;
use std::rc::Rc;
use ultraviolet::mat::Mat4;
use ultraviolet::projection::perspective_gl as perspective;
use ultraviolet::vec::Vec3;

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

mod shader;
fn mat4_to_slice(m: &ultraviolet::mat::Mat4) -> *const [f32; 16] {
    m.as_slice().as_ptr() as *const [f32; 16]
}

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
    use_indices: bool,
}

impl Graphics {
    pub fn new(gl: GlContext) -> Self {
        Self {
            gl,
            driver: Driver::OpenGl3_3,
            use_indices: false,
        }
    }

    pub fn viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(x, y, width, height);
        }
    }

    pub fn begin(&mut self) {
        unsafe {
            self.gl.enable(glow::DEPTH_TEST);
            self.gl.depth_func(glow::LESS);
            // self.gl.enable(glow::CULL_FACE);
            // self.gl.cull_face(glow::BACK);
        }
    }

    pub fn end(&mut self) {
        unsafe {
            self.use_indices = false;
            self.gl.bind_vertex_array(None);
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }

    pub fn use_shader(&mut self, shader: &Shader) {
        unsafe {
            self.gl.bind_vertex_array(Some(shader.vao));
            self.gl.use_program(Some(shader.program));
        }
    }

    pub fn clear(&mut self, color: Option<[f32; 4]>) {
        unsafe {
            if let Some([r, g, b, a]) = color {
                self.gl.clear_color(r, g, b, a);
            }
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn bind_vertex_buffer(&mut self, buffer: &VertexBuffer, data: &[f32]) {
        buffer.bind(&self.gl, data);
    }

    pub fn bind_index_buffer(&mut self, buffer: &IndexBuffer, data: &[u32]) {
        self.use_indices = true;
        buffer.bind(&self.gl, data);
    }

    pub fn draw(&mut self, offset: i32, count: i32) {
        // TODO draw instanced?
        unsafe {
            if self.use_indices {
                self.gl
                    .draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, offset * 4);
            } else {
                self.gl.draw_arrays(glow::TRIANGLES, offset, count);
            }
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

impl UniformValue for ultraviolet::mat::Mat4 {
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
            .with_depth_buffer(24)
            .with_stencil_buffer(8)
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
    let mut cube = Cube::new(&mut gfx);
    let mut triangle = Triangle::new(&mut gfx);
    let mut textured_cube = TexturedCube::new(&mut gfx);

    unsafe {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    windowed_context.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    gfx.begin();
                    gfx.viewport(0, 0, 1024, 768);
                    gfx.clear(Some([0.1, 0.2, 0.4, 1.0]));
                    textured_cube.draw(&mut gfx);
                    cube.draw(&mut gfx);
                    triangle.draw(&mut gfx);
                    gfx.end();

                    windowed_context.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed_context.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

struct Triangle {
    shader: Shader,
    vertices: [f32; 21],
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    mvp: Mat4,
}

impl Triangle {
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
            Usage::Dynamic,
        )
        .unwrap();

        let index_buffer = IndexBuffer::new(gfx, Usage::Dynamic).unwrap();

        #[rustfmt::skip]
        let vertices = [
            -0.1, -0.1, 0.0,    1.0, 0.2, 0.3, 1.0,
            0.1, -0.1, 0.0,     0.1, 1.0, 0.3, 1.0,
            0.0, 0.1, 0.0,      0.1, 0.2, 1.0, 1.0,
        ];

        let mvp = Mat4::identity();

        Self {
            shader,
            vertices,
            vertex_buffer,
            index_buffer,
            mvp,
        }
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        gfx.use_shader(&self.shader);
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &[0, 1, 2]);
        gfx.bind_uniform(0, &self.mvp);
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
            Usage::Dynamic,
        )
        .unwrap();

        let index_buffer = IndexBuffer::new(gfx, Usage::Dynamic).unwrap();

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

        gfx.use_shader(&self.shader);
        gfx.bind_uniform(0, &mvp);
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &self.indices);
        gfx.draw(0, self.indices.len() as i32);
    }
}

struct TexturedCube {
    shader: Shader,
    vertices: [f32; 108],
    uvs: [f32; 72],
    vertex_buffer: VertexBuffer,
    uvs_buffer: VertexBuffer,
    rotation: (f32, f32),
    mvp: Mat4,
    tex: u32,
}

impl TexturedCube {
    fn new(gfx: &mut Graphics) -> Self {
        let shader = Shader::new(
            gfx,
            include_bytes!("../resources/shaders/textured.vert.spv"),
            include_bytes!("../resources/shaders/textured.frag.spv"),
        )
        .unwrap();

        let vertex_buffer = VertexBuffer::new(
            gfx,
            &[VertexAttr::new(0, VertexFormat::Float3)],
            Usage::Dynamic,
        )
        .unwrap();

        let uvs_buffer = VertexBuffer::new(
            gfx,
            &[VertexAttr::new(1, VertexFormat::Float2)],
            Usage::Dynamic,
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
            shader,
            vertices,
            vertex_buffer,
            uvs_buffer,
            rotation: (0.0, 0.0),
            mvp,
            uvs,
            tex,
        }
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        let (ref mut rx, ref mut ry) = self.rotation;

        *rx += 0.01;
        *ry += 0.01;

        let rxm = Mat4::from_rotation_x(-*rx);
        let rym = Mat4::from_rotation_y(-*ry);
        let model = rxm * rym;
        let mvp:Mat4 = self.mvp * model;
        let (mvp_loc, tex_loc) = unsafe {
            (
                gfx.gl
                    .get_uniform_location(self.shader.program, "u_matrix")
                    .unwrap(),
                gfx.gl
                    .get_uniform_location(self.shader.program, "u_texture")
                    .unwrap(),
            )
        };

        gfx.use_shader(&self.shader);
        gfx.bind_uniform(mvp_loc, &mvp);
        unsafe {
            gfx.gl.active_texture(glow::TEXTURE0);
            gfx.gl.bind_texture(glow::TEXTURE_2D, Some(self.tex));
            gfx.gl.uniform_1_i32(Some(tex_loc), 0);
        }
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_vertex_buffer(&self.uvs_buffer, &self.uvs);
        gfx.draw(0, (self.vertices.len() / 3) as i32);
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
