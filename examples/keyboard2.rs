use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(|app| State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
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
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.color = Color::YELLOW;
    draw.text(
        &state.font,
        "Just write down whatever is on your head:",
        10.0,
        10.0,
        20.0,
    );

    draw.color = Color::WHITE;
    draw.text_ext(&state.font, &state.msg, 10.0, 50.0, 20.0, 780.0);

    draw.end();
}

struct State {
    font: Font,
    msg: String,
}
