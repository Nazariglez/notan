use glow::{Context, HasContext};
use nae_core::{BlendFactor, BlendMode, Color};
use std::rc::Rc;
use glutin::event_loop::ControlFlow;
use glutin::event::{Event, WindowEvent};
use crate::shader::{Shader, Driver};
use std::cell::Ref;
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

    let shader = Shader::new(
        include_bytes!("../resources/shaders/color.vert.spv"),
        include_bytes!("../resources/shaders/color.frag.spv"),
        &g,
    ).unwrap();



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
                unsafe {
                    g.gl.clear(glow::COLOR_BUFFER_BIT);
                    g.gl.draw_arrays(glow::TRIANGLES, 0, 3);
                }
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

