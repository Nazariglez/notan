use nae::prelude::*;

struct State {
    image: nae_gfx::texture::Texture,
    count: f32,
}

fn init(app: &mut App) -> State {
    State {
        image: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/t.png")).unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.pattern_ext(
        &mut state.image,
        10.0,
        10.0,
        780.0,
        580.0,
        0.0,
        0.0,
        2.0 + state.count.sin(),
        2.0 + state.count.cos(),
    );
    draw.end();

    state.count += 0.5 * app.delta;
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build();
}
