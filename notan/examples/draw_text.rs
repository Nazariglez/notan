use notan::prelude::*;

#[derive(notan::AppState)]
struct State {
    font: Font,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.text(&state.font, "Hello World!")
        .position(400.0, 300.0)
        .size(60.0)
        .color(Color::ORANGE)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
