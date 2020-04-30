use nae::prelude::*;

struct State {
    btn: nae_gfx::texture::Texture,
    count: f32,
}

fn init(app: &mut App) -> State {
    State {
        btn: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/grey_button.png"))
            .unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let width = (2.0 + state.count.sin()) * state.btn.width() * 2.0;
    let height = (2.0 + state.count.cos()) * state.btn.height() * 2.0;

    let draw = app.draw2();
    draw.begin(Color::WHITE);
    draw.image(&mut state.btn, 10.0, 10.0);
    draw.image_9slice_ext(
        &mut state.btn,
        200.0,
        120.0,
        width,
        height,
        10.0,
        10.0,
        5.0,
        28.0,
    );
    draw.end();

    state.count += 0.03;
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
