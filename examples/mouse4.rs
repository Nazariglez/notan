use nae::prelude::*;

struct State {
    font: Font,
    x: f32,
    y: f32,
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
        x: 400.0,
        y: 300.0,
    }
}

fn event(app: &mut App, state: &mut State, evt: Event) {
    match evt {
        Event::MouseWheel { delta_x, delta_y } => {
            state.x = (state.x + delta_x).max(0.0).min(800.0);
            state.y = (state.y + delta_y).max(0.0).min(600.0);
        }
        _ => {}
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.color = Color::WHITE;
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text(
        &state.font,
        "Scroll with your mouse's wheel or touchpad",
        400.0,
        300.0,
        40.0,
    );

    draw.color = Color::RED;
    draw.circle(state.x, state.y, 30.0);

    draw.end();
}
