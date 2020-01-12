use nae::prelude::*;

const LINE_COLORS: [Color; 8] = [
    Color::WHITE,
    Color::ORANGE,
    Color::GREEN,
    Color::RED,
    Color::PINK,
    Color::BLUE,
    Color::FUCHSIA,
    Color::YELLOW,
];

struct Line {
    color: Color,
    points: Vec<(f32, f32)>,
}

struct State {
    is_drawing: bool,
    color_index: usize,
    lines: Vec<Line>,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .event(event)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    State {
        is_drawing: false,
        color_index: 0,
        lines: vec![],
    }
}

fn event(app: &mut App, state: &mut State, evt: Event) {
    match evt {
        Event::MouseDown { x, y, .. } => {
            if state.is_drawing {
                return;
            }
            state.is_drawing = true;
            state.lines.push(Line {
                color: LINE_COLORS[state.color_index],
                points: vec![(x as _, y as _)],
            });
        }
        Event::MouseMove { x, y } => {
            if !state.is_drawing {
                return;
            }

            if let Some(line) = state.lines.last_mut() {
                line.points.push((x as _, y as _));
            }
        }
        Event::MouseUp { x, y, .. } => {
            if !state.is_drawing {
                return;
            }

            if let Some(line) = state.lines.last_mut() {
                line.points.push((x as _, y as _));
            }

            state.color_index = (state.color_index + 1) % (LINE_COLORS.len() - 1);
            state.is_drawing = false;
        }
        _ => {}
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    state.lines.iter().for_each(|line| {
        draw.set_color(line.color);
        if line.points.len() > 1 {
            for i in 1..line.points.len() {
                let p1 = line.points[i - 1];
                let p2 = line.points[i];
                draw.line(p1.0, p1.1, p2.0, p2.1, 10.0);
            }
        }
    });

    draw.end();
}
