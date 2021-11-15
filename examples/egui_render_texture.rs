use glam::{Mat4, Vec3};
use notan::egui::{self, *};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    cube: Cube,
    render_texture: RenderTexture,
    egui_tex: egui::TextureId,
    img_size: egui::Vec2,
}

impl State {
    fn new(gfx: &mut Graphics) -> State {
        let cube = Cube::new(gfx);

        let render_texture = gfx
            .create_render_texture(500, 400)
            .with_depth()
            .build()
            .unwrap();

        let img_size = egui::vec2(render_texture.width(), render_texture.height());
        let egui_texture_id = gfx.register_egui_texture(&render_texture).unwrap();

        Self {
            img_size,
            egui_tex: egui_texture_id,
            cube,
            render_texture,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .set_config(WindowConfig::new().vsync())
        .set_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let cube_renderer = state.cube.create_renderer(gfx, app.timer.delta_f32());
    gfx.render_to(&state.render_texture, &cube_renderer);

    let ctx = plugins
        .get_mut::<EguiPlugin>()
        .map(|mut plugin| plugin.create_context(Some(Color::BLACK)))
        .unwrap();

    egui::Window::new("Notan Render Texture").show(&ctx, |ui| {
        ui.image(state.egui_tex, state.img_size);
    });

    gfx.render(&ctx);
}

// - Cube Code

//language=glsl
const COLOR_VERTEX: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec4 a_color;

    layout(location = 0) out vec4 v_color;

    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_matrix;
    };

    void main() {
        v_color = a_color;
        gl_Position = u_matrix * a_position;
    }
    "#
};

//language=glsl
const COLOR_FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    void main() {
        color = v_color;
    }
    "#
};

struct Cube {
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform_buffer: Buffer,
    mvp: glam::Mat4,
    angle: f32,
}

impl Cube {
    fn new(gfx: &mut Graphics) -> Self {
        let vertex_info = VertexInfo::new()
            .attr(0, VertexFormat::Float3)
            .attr(1, VertexFormat::Float4);

        let pipeline = gfx
            .create_pipeline()
            .from(&COLOR_VERTEX, &COLOR_FRAGMENT)
            .vertex_info(&vertex_info)
            .with_depth_stencil(DepthStencil {
                write: true,
                compare: CompareMode::Less,
            })
            .build()
            .unwrap();

        #[rustfmt::skip]
        let vertices = [
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

        let projection = Mat4::perspective_rh_gl(45.0, 4.0 / 3.0, 0.1, 100.0);
        let view = Mat4::look_at_rh(
            Vec3::new(4.0, 3.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let mvp = Mat4::IDENTITY * projection * view;

        let vertex_buffer = gfx
            .create_vertex_buffer()
            .with_info(&vertex_info)
            .with_data(&vertices)
            .build()
            .unwrap();

        let index_buffer = gfx
            .create_index_buffer()
            .with_data(&indices)
            .build()
            .unwrap();

        let uniform_buffer = gfx
            .create_uniform_buffer(0, "Locals")
            .with_data(&mvp.to_cols_array())
            .build()
            .unwrap();

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            mvp,
            angle: 0.0,
        }
    }

    fn create_renderer(&mut self, gfx: &mut Graphics, delta: f32) -> Renderer {
        gfx.set_buffer_data(&self.uniform_buffer, &rotated_matrix(self.mvp, self.angle));

        let mut renderer = gfx.create_renderer();
        renderer.begin(Some(&ClearOptions {
            color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
            depth: Some(1.0),
            ..Default::default()
        }));

        renderer.set_pipeline(&self.pipeline);
        renderer.bind_uniform_buffer(&self.uniform_buffer);
        renderer.bind_vertex_buffer(&self.vertex_buffer);
        renderer.bind_index_buffer(&self.index_buffer);
        renderer.draw(0, 36);
        renderer.end();

        self.angle += 0.6 * delta;

        renderer
    }
}
fn rotated_matrix(base: Mat4, angle: f32) -> [f32; 16] {
    let rot_x = Mat4::from_rotation_x(angle);
    let rot_y = Mat4::from_rotation_y(angle);
    (base * rot_x * rot_y).to_cols_array()
}
