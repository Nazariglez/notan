use notan::draw::*;
use notan::prelude::*;
use notan::text::*;
use notan_math::Rect;

#[derive(AppState)]
struct State {
    font: Font,
    font2: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(TextConfig)
        .add_config(DrawConfig)
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
    text.add("This text is a ")
        .font(&state.font)
        .position(400.0, 30.0)
        .h_align_center()
        .color(Color::ORANGE)
        .size(24.0);

    text.chain("Section")
        .font(&state.font2)
        .size(26.0)
        .color(Color::RED);

    text.chain("\nthat uses different font-styles")
        .font(&state.font)
        .size(22.0)
        .color(Color::PINK);

    // Get the boundaries for the last section
    let section_1_bounds = text.last_bounds();

    text.add("Another Section")
        .font(&state.font2)
        .color(Color::WHITE)
        .size(40.0)
        .position(50.0, 300.0)
        .v_align_middle();

    // Get the boundaries for the last section
    let section_2_bounds = text.last_bounds();

    text.add("Last section")
        .font(&state.font)
        .color(Color::PURPLE)
        .size(16.0)
        .position(650.0, 450.0)
        .h_align_right()
        .v_align_middle();

    // Get the boundaries for the last section
    let section_3_bounds = text.last_bounds();

    // Render the sizes numbers
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // get the size of all bounds combined
    let mixed_bounds = text.bounds();
    draw.rect(
        (mixed_bounds.x, mixed_bounds.y),
        (mixed_bounds.width, mixed_bounds.height),
    )
    .color(Color::WHITE)
    .alpha(0.1)
    .fill();

    draw.text(
        &state.font,
        &format!(
            "Mixed size: {:.2}x{:.2}",
            mixed_bounds.width, mixed_bounds.height
        ),
    )
    .h_align_center()
    .v_align_bottom()
    .position(400.0, 550.0)
    .size(20.0)
    .color(Color::OLIVE);

    // draw the size of each section
    draw_size(&mut draw, &state.font, section_1_bounds);
    draw_size(&mut draw, &state.font, section_2_bounds);
    draw_size(&mut draw, &state.font, section_3_bounds);

    // draw debug info (sized)
    gfx.render(&draw);

    // draw the real text
    gfx.render(&text);
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
