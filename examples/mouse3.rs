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

struct Line {
    color: Color,
    points: Vec<(f32, f32)>,
}

struct State {
    geom: Geometry,
    color_index: usize,
    lines: Vec<Line>,
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
        lines: vec![],
    }
}

fn process_input(app: &mut App, state: &mut State) {
    if app.mouse.was_pressed(MouseButton::Left) {
        state.lines.push(Line {
            color: LINE_COLORS[state.color_index],
            points: vec![(app.mouse.x, app.mouse.y)],
        });
    }

    if app.mouse.is_down(MouseButton::Left) {
        if let Some(line) = state.lines.last_mut() {
            line.points.push((app.mouse.x, app.mouse.y));
        }
    }

    if app.mouse.was_released(MouseButton::Left) {
        if let Some(line) = state.lines.last_mut() {
            line.points.push((app.mouse.x, app.mouse.y));
        }

        state.color_index = (state.color_index + 1) % (LINE_COLORS.len() - 1);
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::WHITE);
    draw.text_ext(
        "Click and drag to paint!",
        400.0,
        300.0,
        40.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    state.geom.clear();
    for line in &state.lines {
        for (i, (x, y)) in line.points.iter().enumerate() {
            if i == 0 {
                state.geom.move_to(*x, *y);
            } else {
                state.geom.line_to(*x, *y);
            }
        }
        state.geom.stroke(line.color, 10.0);
    }

    draw.geometry(&mut state.geom);
    draw.end();
}
