use nae::prelude::*;

struct State {
    font: Font,
    tex: Texture,
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

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.push_translation(x, y);
    draw.image(img, 0.0, 0.0);
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Top);
    draw.text(
        &state.font,
        &format!("x: {}, y: {}", x.round(), y.round()),
        img.width() * 0.5,
        img.height(),
        24.0,
    );
    draw.pop();

    draw.end();

    state.count += 0.005;
}

fn init(app: &mut App) -> State {
    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        tex: Texture::from_bytes(app, include_bytes!("assets/ferris.png")).unwrap(),
        count: 0.0,
    }
}
