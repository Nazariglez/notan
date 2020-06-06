use m2d::point;
use nae::prelude::*;

const DEG_TO_RAD:f32 = std::f32::consts::PI / 180.0;

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        rot: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mouse_x = app.mouse.x;
    let mouse_y = app.mouse.y;

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.color = Color::WHITE;
    let text = format!("Screen: {} {}", mouse_x, mouse_y);
    draw.text(&state.font, &text, 10.0, 10.0, 20.0);

    // -- Rectangle
    draw.color = Color::MAGENTA;
    draw.push_translation(100.0, 100.0);
    draw.rect(0.0, 0.0, 200.0, 200.0);

    // Get the refernece points before pop the transform
    let (local_x, local_y) = point::screen_to_local(mouse_x, mouse_y, draw.transform());
    let (global_x, global_y) = point::local_to_screen(local_x, local_y, draw.transform());

    draw.pop();

    // Draw on top the coordinates
    draw.color = Color::WHITE;
    let text = format!("Local: {} {}", local_x, local_y);
    draw.text(&state.font, &text, 100.0, 100.0, 16.0);

    let text = format!("World: {} {}", global_x, global_y);
    draw.text(&state.font, &text, 100.0, 130.0, 16.0);

    // -- Rotated Rectangle
    draw.color = Color::ORANGE;
    draw.push_translation(500.0, 200.0);
    draw.push_rotation(state.rot);
    draw.push_translation(-100.0, -100.0);
    draw.rect(0.0, 0.0, 200.0, 200.0);

    // Get the refernece points before pop the transform
    let (local_x, local_y) = point::screen_to_local(mouse_x, mouse_y, draw.transform());
    let (global_x, global_y) = point::local_to_screen(local_x, local_y, draw.transform());

    draw.pop();
    draw.pop();
    draw.pop();

    // Draw on top the coordinates
    draw.color = Color::WHITE;
    let text = format!("Local: {} {}", local_x.round(), local_y.round());
    draw.text(&state.font, &text, 400.0, 100.0, 16.0);

    let text = format!("World: {} {}", global_x.round(), global_y.round());
    draw.text(&state.font, &text, 400.0, 130.0, 16.0);

    draw.end();

    state.rot += (1.0 * DEG_TO_RAD) * app.delta;
}

struct State {
    font: Font,
    rot: f32,
}
