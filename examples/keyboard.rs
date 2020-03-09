use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(|app| State::default())
        .draw(draw)
        .event(event)
        .update(update)
        .build()
        .unwrap();
}

fn event(app: &mut App, state: &mut State, evt: Event) {
    match evt {
        Event::KeyDown { key } => {
            state.last_key = Some(key);
        }
        _ => {}
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::W) {
        state.y -= 2.0;
    }

    if app.keyboard.is_down(KeyCode::A) {
        state.x -= 2.0;
    }

    if app.keyboard.is_down(KeyCode::S) {
        state.y += 2.0;
    }

    if app.keyboard.is_down(KeyCode::D) {
        state.x += 2.0;
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::RED);
    draw.circle(state.x, state.y, 50.0);

    draw.set_color(Color::WHITE);
    draw.text("Use WASD to move the circle.", 10.0, 10.0, 20.0);

    if let Some(key) = &state.last_key {
        draw.text(&format!("Last key: {:?}", key), 10.0, 560.0, 20.0);
    }

    draw.end();
}

struct State {
    x: f32,
    y: f32,
    last_key: Option<KeyCode>,
}

impl Default for State {
    fn default() -> Self {
        State {
            x: 400.0,
            y: 300.0,
            last_key: None,
        }
    }
}
