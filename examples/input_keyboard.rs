use notan::draw::*;
use notan::prelude::*;

const MOVE_SPEED: f32 = 100.0;

#[derive(AppState)]
struct State {
    font: Font,
    x: f32,
    y: f32,
    last_key: Option<KeyCode>,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
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
        x: 400.0,
        y: 300.0,
        last_key: None,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.last_key = app.keyboard.last_key_released();

    if app.keyboard.is_down(KeyCode::W) {
        state.y -= MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::A) {
        state.x -= MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::S) {
        state.y += MOVE_SPEED * app.timer.delta_f32();
    }

    if app.keyboard.is_down(KeyCode::D) {
        state.x += MOVE_SPEED * app.timer.delta_f32();
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.circle(50.0)
        .position(state.x, state.y)
        .color(Color::RED);

    draw.text(&state.font, "Use WASD to move the circle")
        .position(10.0, 10.0)
        .size(20.0);

    if let Some(key) = &state.last_key {
        draw.text(&state.font, &format!("Last key: {:?}", key))
            .position(10.0, 560.0)
            .size(20.0);
    }

    gfx.render(&draw);
}
