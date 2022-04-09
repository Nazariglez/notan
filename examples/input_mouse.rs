use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    x: f32,
    y: f32,
    left: Vec<(f32, f32)>,   // red
    middle: Vec<(f32, f32)>, // green
    right: Vec<(f32, f32)>,  // blue
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        font,
        x: 0.0,
        y: 0.0,
        left: vec![],
        middle: vec![],
        right: vec![],
    }
}

fn update(app: &mut App, state: &mut State) {
    // get mouse cursor position here
    let (x, y) = app.mouse.position();

    if app.mouse.was_pressed(MouseButton::Left) {
        state.left.push((x, y));
    }

    if app.mouse.was_pressed(MouseButton::Middle) {
        state.middle.push((x, y));
    }

    if app.mouse.was_pressed(MouseButton::Right) {
        state.right.push((x, y));
    }

    state.x = x;
    state.y = y;
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Draw cursor
    draw.circle(8.0)
        .position(state.x, state.y)
        .color(Color::ORANGE);

    // Draw left clicks
    state.left.iter().for_each(|(x, y)| {
        draw.circle(4.0).position(*x, *y).color(Color::RED);
    });

    // Draw middle clicks
    state.middle.iter().for_each(|(x, y)| {
        draw.circle(4.0).position(*x, *y).color(Color::GREEN);
    });

    // Draw right clicks
    state.right.iter().for_each(|(x, y)| {
        draw.circle(4.0).position(*x, *y).color(Color::BLUE);
    });

    // Draw position
    let text = format!("x: {} - y: {}", state.x, state.y);
    draw.text(&state.font, &text)
        .position(400.0, 300.0)
        .size(80.0)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
