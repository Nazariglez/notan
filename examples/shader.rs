use nae::prelude::*;

struct State {
    pipeline: Pipeline,
    a: Uniform,
    // b: Uniform,
    // size_loc: Uniform,
    // tex_size_loc: Uniform,
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
    let a = pipeline.uniform_location("u_size3").unwrap();
    println!("a {}", a);
    // let b = pipeline.uniform_location("u_b").unwrap();
    // let size_loc = pipeline.uniform_location("u_size").unwrap();
    // let tex_size_loc = pipeline.uniform_location("u_tex_size").unwrap();

    State {
        pipeline,
        a,
        // b,
        // size_loc,
        // tex_size_loc,
        image: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/ferris.png"))
            .unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    if !state.image.is_loaded() { return }
    let width = state.image.width();
    let height = state.image.height();
    let size = 5.0 + state.count.sin();

    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.image(&state.image, 20.0, 150.0);

    draw.set_pipeline(Some(&state.pipeline));
    draw.set_uniform(&state.a, &[0.0, 0.0, 1.0, 1.0]);
    // draw.set_uniform(&state.b, &[0.0, 0.0, 1.0, 1.0]);
    // draw.set_uniform(&state.size_loc, &[size, size]);
    // draw.set_uniform(&state.tex_size_loc, &[width, height]);

    draw.image(&state.image, 400.0, 150.0);
    //
    // println!("# set_pipeline none");
    draw.set_pipeline(None);

    draw.end();

    state.count += 0.005;

    // panic!();
}
