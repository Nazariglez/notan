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
    }
}

fn process_input(app: &mut App, state: &mut State) {
    if app.mouse.was_pressed(MouseButton::Left) {
        state.geom.move_to(app.mouse.x, app.mouse.y);
        state.drawing = true;
    }

    if app.mouse.is_down(MouseButton::Left) {
        if state.drawing {
            state.geom.line_to(app.mouse.x, app.mouse.y);
        }
    }

    if app.mouse.was_released(MouseButton::Left) {
        if state.drawing {
            state.drawing = false;
            state.geom.line_to(app.mouse.x, app.mouse.y);

            let color = LINE_COLORS[state.color_index];
            state.color_index = (state.color_index + 1) % (LINE_COLORS.len() - 1);

            state.geom.stroke(color, 10.0);
            println!("stroke {:?} {}", color, state.color_index);
        }
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

    draw.geometry(&state.geom);
    draw.end();
}
