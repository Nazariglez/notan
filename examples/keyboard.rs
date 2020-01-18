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
            match key {
                KeyCode::W => state.wasd[0] = true,
                KeyCode::A => state.wasd[1] = true,
                KeyCode::S => state.wasd[2] = true,
                KeyCode::D => state.wasd[3] = true,
                _ => {}
            }
            state.last_key = Some(key);
        }
        Event::KeyUp { key } => match key {
            KeyCode::W => state.wasd[0] = false,
            KeyCode::A => state.wasd[1] = false,
            KeyCode::S => state.wasd[2] = false,
            KeyCode::D => state.wasd[3] = false,
            _ => {}
        },
        _ => {}
    }
}

fn update(app: &mut App, state: &mut State) {
    if state.wasd[0] {
        state.pos.1 -= 2.0;
    }

    if state.wasd[1] {
        state.pos.0 -= 2.0;
    }

    if state.wasd[2] {
        state.pos.1 += 2.0;
    }

    if state.wasd[3] {
        state.pos.0 += 2.0;
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::RED);
    draw.circle(state.pos.0, state.pos.1, 50.0);

    draw.set_color(Color::WHITE);
    draw.text("Use WASD to move the cirle.", 10.0, 10.0, 20.0);

    if let Some(key) = &state.last_key {
        draw.text(&format!("Last key: {:?}", key), 10.0, 560.0, 20.0);
    }

    draw.end();
}

struct State {
    pos: (f32, f32),
    wasd: [bool; 4],
    last_key: Option<KeyCode>,
}

impl Default for State {
    fn default() -> Self {
        State {
            pos: (400.0, 300.0),
            wasd: [false, false, false, false],
            last_key: None,
        }
    }
}
