use crate::shader::{Driver, GlowValue, Shader, VertexData};
use glow::{Context, HasContext};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use nae_core::math::{identity, scaling2d, vec2, Mat3};
use nae_core::{BlendFactor, BlendMode, Color};
use std::cell::Ref;
use std::rc::Rc;
//Sample texture array limit https://stackoverflow.com/questions/20836102/how-many-textures-can-i-use-in-a-webgl-fragment-shader

mod shader;

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
}

pub struct Draw2d<'gfx> {
    gfx: Ref<'gfx, Graphics>,
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

    let ebo = unsafe {
        let ebo = g.gl.create_buffer().unwrap();
        g.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        ebo
    };

    let shader = Shader::new(
        include_bytes!("../resources/shaders/color.vert.spv"),
        include_bytes!("../resources/shaders/color.frag.spv"),
        &g,
    )
    .unwrap();

    unsafe {
        g.gl.use_program(Some(shader.program));
        g.gl.clear_color(0.1, 0.2, 0.4, 1.0);
    }

    let buff0 = unsafe {
        let location = 0;
        let vertex_data = VertexData::Float2;
        let buff = g.gl.create_buffer().unwrap();
        g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff));
        g.gl.enable_vertex_attrib_array(location);

        let stride = 0;
        let offset = 0;
        let size = vertex_data.size();
        let data_type = vertex_data.glow_value();
        let normalized = vertex_data.normalized();
        g.gl.vertex_attrib_pointer_f32(location, size, data_type, normalized, stride, offset);
        buff
    };

    let buff1 = unsafe {
        let location = 1;
        let vertex_data = VertexData::Float4;
        let buff = g.gl.create_buffer().unwrap();
        g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff));
        g.gl.enable_vertex_attrib_array(location);

        let stride = 0;
        let offset = 0;
        let size = vertex_data.size();
        let data_type = vertex_data.glow_value();
        let normalized = vertex_data.normalized();
        g.gl.vertex_attrib_pointer_f32(location, size, data_type, normalized, stride, offset);
        buff
    };

    fn m3_to_slice(m: &Mat3) -> *const [f32; 9] {
        m.as_slice().as_ptr() as *const [f32; 9]
    }

    fn vf_to_u8(v: &[f32]) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
    }

    fn vfi_to_u8(v: &[i32]) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
    }

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

                    g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff0));
                    g.gl.buffer_data_u8_slice(
                        glow::ARRAY_BUFFER,
                        vf_to_u8(&[0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 1.5, 1.0]),
                        glow::DYNAMIC_DRAW,
                    );
                    g.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buff1));
                    g.gl.buffer_data_u8_slice(
                        glow::ARRAY_BUFFER,
                        vf_to_u8(&[
                            0.5, 1.0, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0, 1.0,
                            0.5, 1.0,
                        ]),
                        glow::DYNAMIC_DRAW,
                    );

                    let indices = [0, 1, 2, 0, 2, 3];

                    g.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
                    g.gl.buffer_data_u8_slice(
                        glow::ELEMENT_ARRAY_BUFFER,
                        vfi_to_u8(&indices),
                        glow::STATIC_DRAW,
                    );
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
