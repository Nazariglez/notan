use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .event(event)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> (String, Color) {
    (String::from(""), Color::new(0.1, 0.2, 0.3, 1.0))
}

fn event(app: &mut App, state: &mut (String, Color), evt: Event) {
    match evt {
        Event::MouseMove { .. } => {
            state.0 = "Moving...".to_string();
        }
        Event::MouseDown { button, .. } => {
            state.0 = format!("{:?} pressed...", button);
        }
        Event::MouseUp { button, .. } => {
            state.0 = format!("{:?} released...", button);
        }
        Event::MouseEnter { .. } => {
            state.0 = "Entered...".to_string();
            state.1 = Color::new(0.1, 0.2, 0.3, 1.0);
        }
        Event::MouseLeft { .. } => {
            state.0 = "Outside...".to_string();
            state.1 = Color::ORANGE;
        }
        Event::MouseWheel { .. } => {
            state.0 = "Using Wheel...".to_string();
        }
        _ => {}
    }
}

fn draw(app: &mut App, state: &mut (String, Color)) {
    let draw = app.draw();
    draw.begin();
    draw.clear(state.1);

    draw.text_ext(
        &state.0,
        400.0,
        300.0,
        80.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    draw.end();
}
