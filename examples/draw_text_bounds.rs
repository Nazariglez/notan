use notan::draw::*;
use notan::math::Rect;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.text(&state.font, "Let's measure this text...")
        .position(400.0, 300.0)
        .size(50.0)
        .color(Color::ORANGE)
        .h_align_center()
        .v_align_middle();

    // get text bounds
    let bounds = draw.last_text_bounds();

    // draw the size
    draw_size(&mut draw, &state.font, bounds);

    gfx.render(&draw);
}

fn draw_size(draw: &mut Draw, font: &Font, bounds: Rect) {
    // show height
    draw.line(
        (bounds.max_x() + 10.0, bounds.y),
        (bounds.max_x() + 10.0, bounds.max_y()),
    )
    .width(2.0)
    .color(Color::WHITE);

    draw.text(font, &format!("{}px", bounds.height))
        .position(bounds.max_x() + 20.0, bounds.center_y())
        .v_align_middle()
        .size(20.0);

    // show width
    draw.line(
        (bounds.x, bounds.max_y() + 10.0),
        (bounds.max_x(), bounds.max_y() + 10.0),
    )
    .width(2.0)
    .color(Color::WHITE);

    draw.text(font, &format!("{:.2}px", bounds.width))
        .position(bounds.center_x(), bounds.max_y() + 20.0)
        .h_align_center()
        .size(20.0);
}
