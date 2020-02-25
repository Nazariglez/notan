use crate::shader::{BufferKey, Driver, GlowValue, Shader, VertexData};
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
}

impl Graphics {
    pub fn new(gl: GlContext) -> Self {
        Self {
            gl,
            driver: Driver::OpenGl3_3,
        }
    }

    pub fn set_vertex_buffers(&mut self, buffers: Vec<VertexBuffer>) {}
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
    pub location_id: Option<u32>,
    pub location_name: Option<String>,
    pub vertex_data: VertexData,
}

impl VertexAttr {
    fn with_location(location: u32, vertex_data: VertexData) -> Self {
        Self {
            location_id: Some(location),
            location_name: None,
            vertex_data,
        }
    }

    fn with_name(name: &str, vertex_data: VertexData) -> Self {
        Self {
            location_id: None,
            location_name: Some(name.to_string()),
            vertex_data,
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
    fn new(shader: &Shader, usage: Usage) -> Result<Self, String> {
        unsafe {
            let buffer = shader.gl.create_buffer()?;
            shader
                .gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer));

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

pub trait VertexType {
    fn attributes() -> Vec<VertexAttr>;
    fn data(&self) -> Vec<f32>;
}

struct Vertex {
    a_position: [f32; 2],
    a_color: [f32; 4],
}

impl VertexType for Vertex {
    fn attributes() -> Vec<VertexAttr> {
        vec![
            VertexAttr::with_location(0, VertexData::Float2),
            VertexAttr::with_location(1, VertexData::Float4),
        ]
    }

    fn data(&self) -> Vec<f32> {
        let mut buff = vec![];
        buff.extend_from_slice(&self.a_position);
        buff.extend_from_slice(&self.a_color);
        buff
    }
}

pub struct VertexBuffer {
    buffer: BufferKey,
    usage: Usage,
}

impl VertexBuffer {
    pub fn new(shader: &Shader, attributes: &[VertexAttr], usage: Usage) -> Result<Self, String> {
        unsafe {
            shader.gl.use_program(Some(shader.program));

            let buffer = shader.gl.create_buffer()?;
            shader.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));

            let stride = attributes
                .iter()
                .fold(0, |acc, data| acc + data.vertex_data.bytes());

            let mut offset = 0;
            for attr in attributes {
                let location = match (attr.location_name.as_ref(), attr.location_id.as_ref()) {
                    (Some(name), _) => shader
                        .gl
                        .get_attrib_location(shader.program, name)
                        .ok_or("Invalid location id")?,
                    (_, Some(id)) => *id,
                    _ => return Err("Invalid location id".to_string()),
                };

                let size = attr.vertex_data.size();
                let data_type = attr.vertex_data.glow_value();
                let normalized = attr.vertex_data.normalized();

                shader.gl.enable_vertex_attrib_array(location);
                shader.gl.vertex_attrib_pointer_f32(
                    location, size, data_type, normalized, stride, offset,
                );

                offset += attr.vertex_data.bytes();
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

    let g = Graphics::new(gl);

    let vao = unsafe {
        let vao = g.gl.create_vertex_array().unwrap();

        g.gl.bind_vertex_array(Some(vao));
        vao
    };

    let shader = Shader::new(
        include_bytes!("../resources/shaders/color.vert.spv"),
        include_bytes!("../resources/shaders/color.frag.spv"),
        &g,
    )
    .unwrap();

    unsafe {
        // g.gl.use_program(Some(shader.program));
        g.gl.clear_color(0.1, 0.2, 0.4, 1.0);
    }

    let buffer = VertexBuffer::new(
        &shader,
        &[
            VertexAttr::with_location(0, VertexData::Float2),
            VertexAttr::with_location(1, VertexData::Float4),
        ],
        Usage::Dynamic,
    )
    .unwrap();

    let index_buffer = IndexBuffer::new(&shader, Usage::Dynamic).unwrap();

    // let buff0 = unsafe {
    //     let a_position = 0;
    //     let a_color = 1;
    //     let a_position_vertex_data = VertexData::Float2;
    //     let a_color_vertex_data = VertexData::Float4;
    //
    //     let buff = g.gl.create_buffer().unwrap();
    //     g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff));
    //     g.gl.enable_vertex_attrib_array(a_position);
    //     g.gl.enable_vertex_attrib_array(a_color);
    //
    //     let stride = a_position_vertex_data.bytes() + a_color_vertex_data.bytes();
    //     g.gl.vertex_attrib_pointer_f32(
    //         a_position,
    //         a_position_vertex_data.size(),
    //         a_position_vertex_data.glow_value(),
    //         a_position_vertex_data.normalized(),
    //         stride,
    //         0,
    //     );
    //     g.gl.vertex_attrib_pointer_f32(
    //         a_color,
    //         a_color_vertex_data.size(),
    //         a_color_vertex_data.glow_value(),
    //         a_color_vertex_data.normalized(),
    //         stride,
    //         a_position_vertex_data.bytes(),
    //     );
    //
    //     buff
    // };

    // let buff1 = unsafe {
    //     let location = 1;
    //     let vertex_data = VertexData::Float4;
    //     let buff = g.gl.create_buffer().unwrap();
    //     g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff));
    //     g.gl.enable_vertex_attrib_array(location);
    //
    //     let stride = 0;
    //     let offset = 0;
    //     let size = vertex_data.size();
    //     let data_type = vertex_data.glow_value();
    //     let normalized = vertex_data.normalized();
    //     g.gl.vertex_attrib_pointer_f32(location, size, data_type, normalized, stride, offset);
    //     buff
    // };

    let identity: Mat3 = identity();

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
                    g.gl.clear(glow::COLOR_BUFFER_BIT);

                    // g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff0));
                    //
                    // #[rustfmt::skip]
                    // g.gl.buffer_data_u8_slice(
                    //     glow::ARRAY_BUFFER,
                    //     vf_to_u8(&[
                    //         0.5, 1.0, 0.5, 1.0, 0.5, 1.0,
                    //         0.0, 0.0, 0.5, 1.0, 0.5, 1.0,
                    //         1.0, 0.0, 0.5, 1.0, 0.5, 1.0,
                    //         1.5, 1.0, 0.5, 1.0, 0.5, 1.0,
                    //     ]),
                    //     glow::DYNAMIC_DRAW,
                    // );
                    // g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff1));
                    // g.gl.buffer_data_u8_slice(
                    //     glow::ARRAY_BUFFER,
                    //     vf_to_u8(&[
                    //         0.5, 1.0, 0.5, 1.0,
                    //         0.0, 0.0, 0.5, 1.0,
                    //         1.0, 0.0, 0.5, 1.0,
                    //         1.0, 1.0, 0.5, 1.0,
                    //     ]),
                    //     glow::DYNAMIC_DRAW,
                    // );

                    #[rustfmt::skip]
                    let vertices = [
                        // position     //color
                        0.5, 1.0,       0.5, 1.0, 0.0, 1.0,
                        0.0, 0.0,       0.0, 0.0, 0.4, 1.0,
                        1.0, 0.0,       1.0, 0.0, 0.6, 1.0,
                        1.5, 1.0,       1.0, 0.5, 1.0, 1.0,
                    ];
                    buffer.bind(&shader.gl, &vertices);

                    // If there is not indexBuffer binded just use drawArrays
                    let indices = [0, 1, 2, 0, 2, 3];
                    index_buffer.bind(&shader.gl, &indices);

                    // g.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
                    // g.gl.buffer_data_u8_slice(
                    //     glow::ELEMENT_ARRAY_BUFFER,
                    //     vfi_to_u8(&indices),
                    //     glow::STATIC_DRAW,
                    // );
                    //
                    // g.gl.use_program(Some(shader.program));
                    let mm = identity;
                    // let mm = mm * scaling2d(&vec2(2.0, 2.0));
                    g.gl.uniform_matrix_3_f32_slice(Some(0), false, &*m3_to_slice(&mm));
                    //
                    // g.gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    g.gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

                    windowed_context.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed_context.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        // unsafe {
                        //     gl.delete_program(program);
                        //     gl.delete_vertex_array(vertex_array);
                        // }
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
