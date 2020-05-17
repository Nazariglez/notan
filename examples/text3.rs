use nae::prelude::*;

struct State {
    ubuntu: font::Font,
    ubuntu_mono: font::Font,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    State {
        ubuntu: font::Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        ubuntu_mono: font::Font::from_bytes(app, include_bytes!("assets/UbuntuMono-R.ttf"))
            .unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.color = Color::WHITE;
    draw.text(&state.ubuntu, "I'm the font Ubuntu-B", 10.0, 10.0, 80.0);

    draw.color = Color::GREEN;
    draw.text(
        &state.ubuntu_mono,
        "And I'm the font Ubuntu-MonoR",
        10.0,
        300.0,
        50.0,
    );

    draw.end();
}
