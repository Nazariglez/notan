use notan::draw::*;
use notan::math::Vec2;
use notan::prelude::*;

const POS: Vec2 = Vec2::new(600.0, 300.0);

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec4 v_color;
    layout(location = 0) out vec4 color;

    layout(set = 0, binding = 1) uniform Surface {
        vec2 u_pos;
    };

    void main() {
        if (gl_FragCoord.y < u_pos.y) {
            if (gl_FragCoord.x < u_pos.x) {
                color = vec4(1.0, 0.0, 0.0, 1.0) * v_color;
            } else {
                color = vec4(0.0, 0.0, 1.0, 1.0) * v_color;
            }
        } else {
            if (gl_FragCoord.x < u_pos.x) {
                color = vec4(0.0, 1.0, 0.0, 1.0) * v_color;
            } else {
                color = vec4(0.5, 0.0, 1.0, 1.0) * v_color;
            }
        }
    }
"#
};

#[derive(AppState)]
struct State {
    pipeline: Pipeline,
    ubo: Buffer,
    count: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let pipeline = create_shape_pipeline(gfx, Some(&FRAGMENT)).unwrap();

    let ubo = gfx
        .create_uniform_buffer(1, "Surface")
        .with_data(&[POS.x, POS.y])
        .build()
        .unwrap();

    State {
        pipeline,
        ubo,
        count: 0.0,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    state.count += app.timer.delta_f32() * 10.0;

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // star without custom pipeline
    draw.star(5, 150.0, 70.0)
        .position(200.0, 300.0)
        .rotate_from((200.0, 300.0), state.count.to_radians());

    // add custom pipeline for shapes
    draw.shape_pipeline()
        .pipeline(&state.pipeline)
        .uniform_buffer(&state.ubo);

    draw.star(5, 150.0, 70.0)
        .position(600.0, 300.0)
        .rotate_from((600.0, 300.0), state.count.to_radians());

    // remove custom pipeline
    draw.shape_pipeline().remove();

    gfx.render(&draw);
}
