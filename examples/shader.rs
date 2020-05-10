use nae::prelude::*;

struct State {
    pipeline: Pipeline,
    size_loc: Uniform,
    tex_size_loc: Uniform,
    image: nae_gfx::texture::Texture,
    count: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let pipeline =
        Pipeline::from_image_fragment(app.gfx(), include_bytes!("assets/shaders/pixel.frag.spv"))
            .unwrap();
    let size_loc = pipeline.uniform_location("u_size").unwrap();
    let tex_size_loc = pipeline.uniform_location("u_tex_size").unwrap();

    State {
        pipeline,
        size_loc,
        tex_size_loc,
        image: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/ferris.png"))
            .unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    if !state.image.is_loaded() {
        return;
    }
    let width = state.image.width();
    let height = state.image.height();
    let size = 5.0 + state.count.sin();

    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    // Draw the original sprite as visual reference
    draw.image(&state.image, 20.0, 150.0);

    // Set the new pipeline using the pixel shader
    draw.set_pipeline(Some(&state.pipeline));
    draw.set_uniform(&state.size_loc, &[size, size]);
    draw.set_uniform(&state.tex_size_loc, &[width, height]);

    // Draw the image using the new shader
    draw.image(&state.image, 400.0, 150.0);
    // Reset the pipeline to the default
    draw.set_pipeline(None);

    draw.end();

    state.count += 0.005;
}
