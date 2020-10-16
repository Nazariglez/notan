use nae::prelude::*;

struct State {
    font: Font,
    fade: f32,
}

const text: &'static str =
    "Mr. Stark, I don't feel so good, I don't wanna go. Please, I don't wanna go...";

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let fade_value = 1.0 / (text.len() as f32 + 1.0);
    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        fade: fade_value,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mut x = 20.0;

    let draw = app.draw();
    draw.text_vertical_align = VerticalAlign::Center;

    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    text.split("").enumerate().for_each(|(i, letter)| {
        draw.alpha = 1.0 - ((state.fade * i as f32) % 1.0);
        draw.text(&state.font, letter, x, 300.0, 20.0);

        let width = state.font.size(draw, letter, 20.0, None).0;
        x += parse_width(width);
    });

    draw.end();
}

fn parse_width(width: f32) -> f32 {
    if width == 0.0 {
        10.0
    } else {
        width
    }
}
