use nae::prelude::*;

struct State {
    tex: Texture,
    count: f32,
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    State {
        tex: app.load_file("../assets/ferris.png").unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let image = &state.tex;
    let draw = app.draw();
    let ww = image.width() * 0.5;
    let hh = image.height() * 0.5;

    draw.begin();
    //    draw.clear(Color::RED);
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::YELLOW);
    draw.circle(
        400.0 + state.count.cos() * 250.0,
        300.0 + (state.count * 7.0).cos() * 150.0,
        100.0,
    );

    draw.set_color(Color::WHITE);
    draw.circle(
        400.0 + state.count.sin() * 250.0,
        300.0 + (state.count * 5.0).sin() * 150.0,
        100.0,
    );

    [
        ("None", BlendMode::NONE),
        ("Normal", BlendMode::NORMAL),
        ("Screen", BlendMode::SCREEN),
        ("Erase", BlendMode::ERASE),
        ("Multiply", BlendMode::MULTIPLY),
        ("Add", BlendMode::ADD),
    ]
    .iter()
    .enumerate()
    .for_each(|(i, blend)| {
        let col = (i as f32) % 3.0;
        let row = (i as f32) / 3.0;
        let xx = 100.0 + (ww + 20.0) * col;
        let yy = 100.0 + (hh + 20.0) * row;

        draw.set_blend(blend.1);
        draw.image_ext(&image, xx, yy, ww, hh, 0.0, 0.0, 0.0, 0.0);

        draw.set_blend(BlendMode::NORMAL);
        draw.text(blend.0, xx, yy, 20.0);
    });

    draw.end();

    state.count += 0.002;
}
