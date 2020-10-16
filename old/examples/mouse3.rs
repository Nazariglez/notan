use nae::prelude::*;

const LINE_COLORS: [Color; 8] = [
    Color::WHITE,
    Color::ORANGE,
    Color::GREEN,
    Color::RED,
    Color::PINK,
    Color::BLUE,
    Color::MAGENTA,
    Color::YELLOW,
];

struct State {
    geom: Geometry,
    color_index: usize,
    drawing: bool,
    lines: Vec<(Vec<(f32, f32)>, Color)>,
    font: Font,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .update(process_input)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    State {
        geom: Geometry::new(),
        color_index: 0,
        drawing: false,
        lines: vec![],
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
    }
}

fn process_input(app: &mut App, state: &mut State) {
    // On press down create a new line
    if app.mouse.was_pressed(MouseButton::Left) && !state.drawing {
        state.drawing = true;

        let color = LINE_COLORS[state.color_index];
        state.lines.push((vec![(app.mouse.x, app.mouse.y)], color));

        state.color_index = (state.color_index + 1) % (LINE_COLORS.len() - 1);
    }

    // If the user keeps the mouse button down track the position of the cursor
    if app.mouse.is_down(MouseButton::Left) && state.drawing {
        if let Some((positions, _)) = state.lines.last_mut() {
            positions.push((app.mouse.x, app.mouse.y));
        }
    }

    // When the button is released stops drawing
    if app.mouse.was_released(MouseButton::Left) && state.drawing {
        state.drawing = false;

        if let Some((positions, _)) = state.lines.last_mut() {
            positions.push((app.mouse.x, app.mouse.y));
        }
    }

    // Clear and repopulate the geometry object every frame to see in real time the draw.
    populate_geometry(state);
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.color = Color::WHITE;
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text(&state.font, "Click and drag to paint!", 400.0, 300.0, 40.0);

    draw.geometry(&state.geom);
    draw.end();
}

fn populate_geometry(state: &mut State) {
    state.geom.clear();
    for (positions, color) in state.lines.iter() {
        for (i, (x, y)) in positions.iter().enumerate() {
            if i == 0 {
                state.geom.move_to(*x, *y);
            } else {
                state.geom.line_to(*x, *y);
            }
        }
        state.geom.stroke(*color, 10.0);
    }
}
