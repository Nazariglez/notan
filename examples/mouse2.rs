use nae::prelude::*;

struct State {
    font: Font,
    text: String,
    color: Color,
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
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        color: Color::new(0.1, 0.2, 0.3, 1.0),
        text: String::from(""),
    }
}

fn event(app: &mut App, state: &mut State, evt: Event) {
    match evt {
        Event::MouseMove { .. } => {
            state.text = "Moving...".to_string();
        }
        Event::MouseDown { button, .. } => {
            state.text = format!("{:?} pressed...", button);
        }
        Event::MouseUp { button, .. } => {
            state.text = format!("{:?} released...", button);
        }
        Event::MouseEnter { .. } => {
            state.text = "Entered...".to_string();
            state.color = Color::new(0.1, 0.2, 0.3, 1.0);
        }
        Event::MouseLeft { .. } => {
            state.text = "Outside...".to_string();
            state.color = Color::ORANGE;
        }
        Event::MouseWheel { .. } => {
            state.text = "Using Wheel...".to_string();
        }
        _ => {}
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(state.color);

    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text(&state.font, &state.text, 400.0, 300.0, 80.0);

    draw.end();
}
