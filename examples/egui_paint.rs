use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    triangle: Triangle,
    angle: f32,
}

impl State {
    pub fn new(gfx: &mut Graphics) -> Self {
        Self {
            triangle: Triangle::new(gfx),
            angle: 0.0,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .lazy_loop(true)
        .vsync(true)
        .high_dpi(true);
    notan::init_with(State::new)
        .add_config(win)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
                state.angle += response.drag_delta().x * 0.01;

                // Pass as callback the triangle to be draw
                let triangle = state.triangle.clone();
                let angle = state.angle;

                let cb = EguiCallbackFn::new(move |info, device| {
                    triangle.draw(device, info, angle);
                });

                let callback = egui::PaintCallback {
                    rect,
                    callback: std::sync::Arc::new(cb),
                };

                ui.painter().add(callback);
            });

            ui.label("Drag to rotate!");
        });
    });

    output.clear_color(Color::BLACK);

    gfx.render(&output);
}

// --- TRIANGLE
//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec3 a_color;

    layout(location = 0) out vec3 v_color;

    layout(set = 0, binding = 0) uniform Locals {
        float u_angle;
    };

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_pos, 0.0, 1.0);
        gl_Position.x *= cos(u_angle);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec3 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
    "#
};

#[derive(Clone)]
struct Triangle {
    pipeline: Pipeline,
    vbo: Buffer,
    ubo: Buffer,
}

impl Triangle {
    fn new(gfx: &mut Graphics) -> Self {
        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float32x2)
            .attr(1, VertexFormat::Float32x3);

        let pipeline = gfx
            .create_pipeline()
            .from(&VERT, &FRAG)
            .with_vertex_info(&vertex_info)
            .build()
            .unwrap();

        #[rustfmt::skip]
        let vertices = [
            0.0, 1.0,   1.0, 0.2, 0.3,
            -1.0, -1.0,   0.1, 1.0, 0.3,
            1.0, -1.0,   0.1, 0.2, 1.0,
        ];

        let vbo = gfx
            .create_vertex_buffer()
            .with_info(&vertex_info)
            .with_data(&vertices)
            .build()
            .unwrap();

        let ubo = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(&[0.0])
            .build()
            .unwrap();

        Self { pipeline, vbo, ubo }
    }

    fn draw(&self, device: &mut Device, info: egui::PaintCallbackInfo, angle: f32) {
        // update angle
        device.set_buffer_data(&self.ubo, &[angle]);

        // create a new renderer
        let mut renderer = device.create_renderer();

        // set scissors using the clip_rect passed by egui
        renderer.set_scissors(
            info.clip_rect.min.x,
            info.clip_rect.min.y,
            info.clip_rect.width(),
            info.clip_rect.height(),
        );

        // start the pass
        renderer.begin(None);

        // set the viewport using the rect passed by egui
        renderer.set_viewport(
            info.viewport.min.x,
            info.viewport.min.y,
            info.viewport.width(),
            info.viewport.height(),
        );

        // draw the triangle
        renderer.set_pipeline(&self.pipeline);
        renderer.bind_buffer(&self.vbo);
        renderer.draw(0, 3);
        renderer.end();

        device.render(renderer.commands());
    }
}
