use nae::prelude::*;

struct State {
    tex: nae_gfx::texture::Texture,
    count: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let cos = state.count.cos();
    let sin = state.count.sin();
    let img = &state.tex;
    let initial_x = 40.0 + img.width() * 0.5;
    let initial_y = 50.0 + img.height() * 0.5;

    let x = initial_x + cos * 200.0;
    let y = initial_y + sin * 150.0;

    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.push_translation(x, y);
    draw.image(img, 0.0, 0.0);
    //TODO
    // draw.text_ext(
    //     &format!("x: {}, y: {}", x.round(), y.round()),
    //     img.width() * 0.5,
    //     img.height(),
    //     24.0,
    //     HorizontalAlign::Center,
    //     VerticalAlign::Top,
    //     None,
    // );
    draw.pop();

    draw.end();

    state.count += 0.005;
}

fn init(app: &mut App) -> State {
    State {
        tex: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/ferris.png"))
            .unwrap(),
        count: 0.0,
    }
}
