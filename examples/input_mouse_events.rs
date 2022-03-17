use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    text: String,
    color: Color,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .event(event)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        font,
        color: Color::BLACK,
        text: String::from(""),
    }
}

fn event(state: &mut State, evt: Event) {
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
            state.color = Color::BLACK;
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

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(state.color);

    draw.text(&state.font, &state.text)
        .position(400.0, 300.0)
        .size(80.0)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
