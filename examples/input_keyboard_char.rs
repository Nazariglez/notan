use notan::app::keyboard::KeyCode;
use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    msg: String,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(DrawConfig)
        .event(event)
        .update(update)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        font,
        msg: String::from(""),
    }
}

fn event(state: &mut State, event: Event) {
    match event {
        Event::ReceivedCharacter(c) if c != '\u{7f}' => {
            state.msg.push(c);
        }
        _ => {}
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Back) && !state.msg.is_empty() {
        state.msg.pop();
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.text(&state.font, "Type anything:")
        .position(10.0, 10.0)
        .color(Color::YELLOW)
        .size(20.0);

    draw.text(&state.font, &state.msg)
        .position(20.0, 50.0)
        .max_width(760.0)
        .color(Color::WHITE)
        .size(20.0);

    gfx.render(&draw);
}
