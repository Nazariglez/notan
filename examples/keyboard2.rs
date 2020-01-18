use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(|app| State {
        msg: "".to_string(),
    })
    .draw(draw)
    .event(event)
    .build()
    .unwrap();
}

fn event(app: &mut App, state: &mut State, evt: Event) {
    match evt {
        Event::ReceivedCharacter(c) if c != '\u{7f}' => {
            state.msg.push(c);
        }
        Event::KeyDown { key } => match key {
            KeyCode::Back => {
                let _ = state.msg.pop();
            }
            _ => {}
        },
        _ => {}
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::YELLOW);
    draw.text(
        "Just write down whatever is on your head:",
        10.0,
        10.0,
        20.0,
    );

    draw.set_color(Color::WHITE);
    draw.text_ext(
        &state.msg,
        10.0,
        50.0,
        20.0,
        HorizontalAlign::Left,
        VerticalAlign::Top,
        Some(780.0),
    );

    draw.end();
}

struct State {
    msg: String,
}
