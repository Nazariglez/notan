use notan::glyph::Font;
use notan::prelude::*;

#[derive(notan::AppState)]
struct State {
    font: Font,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(app: &mut App, gfx: &mut Graphics, glyphs: &mut GlyphManager) -> State {
    let font = glyphs
        .load_font(include_bytes!("./assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.text(&state.font, "Hello world!")
        .position(400.0, 300.0)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
