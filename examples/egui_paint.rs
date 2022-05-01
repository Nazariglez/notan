use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    triangle: Triangle,
}

impl State {
    pub fn new(gfx: &mut Graphics) -> Self {
        Self {
            triangle: Triangle::new(gfx),
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, _) =
                    ui.allocate_exact_size(egui::Vec2::splat(200.0), egui::Sense::drag());

                // TODO angle https://github.com/emilk/egui/blob/master/egui_demo_app/src/apps/custom3d.rs

                let triangle = state.triangle.clone();
                let callback = egui::PaintCallback {
                    rect,
                    callback: std::sync::Arc::new(move |info, ctx| {
                        if let Some(device) = ctx.downcast_mut::<Device>() {
                            println!("{:?}, {:?}", info.viewport, info.clip_rect);
                            triangle.draw(device, info);
                        }
                    }),
                };

                ui.painter().add(callback);
            });

            ui.label("Drag to rotate!");
        });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}

// --- TRIANGLE
//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec3 a_color;

    layout(location = 0) out vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_pos - 0.5, 0.0, 1.0);
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
    clear_options: ClearOptions,
}

impl Triangle {
    fn new(gfx: &mut Graphics) -> Self {
        let clear_options = ClearOptions::color(Color::new(0.1, 0.2, 0.3, 1.0));

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
            0.5, 1.0,   1.0, 0.2, 0.3,
            0.0, 0.0,   0.1, 1.0, 0.3,
            1.0, 0.0,   0.1, 0.2, 1.0,
        ];

        let vbo = gfx
            .create_vertex_buffer()
            .with_info(&vertex_info)
            .with_data(&vertices)
            .build()
            .unwrap();

        Self {
            clear_options,
            pipeline,
            vbo,
        }
    }

    fn draw(&self, device: &mut Device, info: &egui::PaintCallbackInfo) {
        let mut renderer = device.create_renderer();
        // renderer.set_size(info.viewport.width() as _, info.viewport.height() as _);

        let screen_height = device.size().1 as f32 * device.dpi() as f32;
        // renderer.set_scissors(
        //     info.viewport.min.x,
        //     -screen_height + info.viewport.max.y - 100.0,
        //     info.viewport.width(),
        //     info.viewport.height(),
        // );
        renderer.begin(Some(&self.clear_options));

        renderer.set_viewport(
            info.viewport.min.x,
            screen_height - info.viewport.max.y,
            info.viewport.width(),
            info.viewport.height(),
        );

        // renderer.set_scissors(
        //     info.clip_rect.min.x,
        //     info.clip_rect.min.y,
        //     info.clip_rect.width(),
        //     info.clip_rect.height(),
        // );

        renderer.set_pipeline(&self.pipeline);
        renderer.bind_buffer(&self.vbo);
        renderer.draw(0, 3);
        renderer.end();

        device.render(renderer.commands());
    }
}
