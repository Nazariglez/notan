use notan::prelude::*;
use notan::text::*;

#[derive(AppState)]
struct State {
    font: Font,
    font2: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(TextConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
        .unwrap();

    let font2 = gfx
        .create_font(include_bytes!("./assets/kenney_pixel-webfont.ttf"))
        .unwrap();

    State { font, font2 }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut text = gfx.create_text();
    text.clear_color(Color::BLACK);

    text.add("Hello ")
        .font(&state.font)
        .position(400.0, 30.0)
        .h_align_center()
        .color(Color::ORANGE)
        .size(30.0);

    text.chain("Notan! ").size(50.0).color(Color::RED);

    text.chain("(Using TextExtension)")
        .font(&state.font2)
        .size(20.0)
        .color(Color::GRAY.with_alpha(0.5));

    gfx.render(&text);
}
