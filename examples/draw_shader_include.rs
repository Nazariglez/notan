use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    texture: Texture,
    pipeline: Pipeline,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();

    const FRAGMENT: ShaderSource =
        notan::include_fragment_shader!("examples/shaders/draw_shader_include.glsl");
    let pipeline = create_image_pipeline(gfx, Some(&FRAGMENT)).unwrap();

    State { texture, pipeline }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.image_pipeline().pipeline(&state.pipeline);
    draw.image(&state.texture).position(250.0, 200.0);
    draw.image_pipeline().remove();

    gfx.render(&draw);
}
