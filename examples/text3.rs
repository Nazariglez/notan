use nae::prelude::*;

struct State {
    ubuntu: Font,
    ubuntu_mono: Font,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    State {
        ubuntu: Font::default(),
        ubuntu_mono: app.load_file("./examples/assets/UbuntuMono-R.ttf").unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::WHITE);
    draw.set_font(&state.ubuntu);
    draw.text("I'm the font Ubuntu-B", 10.0, 10.0, 80.0);

    draw.set_color(Color::GREEN);
    draw.set_font(&state.ubuntu_mono);
    draw.text("And I'm the font Ubuntu-MonoR", 10.0, 300.0, 50.0);

    draw.end();
}
