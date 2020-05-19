use nae::extras::{Scaler, ScalerMode};
use nae::prelude::*;

const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct State {
    scaler: Scaler,
    font: Font,
    texture: Texture,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .size(1200, 800)
        .update(update)
        .draw(draw)
        .resizable()
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    let texture = Texture::from_bytes(app, include_bytes!("assets/rust.png")).unwrap();
    let font = Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap();
    let scaler = create_scaler();
    State {
        font,
        scaler,
        texture,
    }
}

fn create_scaler() -> Scaler {
    // Create the screen scaler using the Working size SCREEN_WIDTH and SCREEN HEIGHT
    let mut scaler = Scaler::new(SCREEN_WIDTH, SCREEN_HEIGHT, ScalerMode::None);

    // Sets the anchor to the center of the size to set the position on the center of the app
    scaler.set_anchor(0.5, 0.5);

    scaler
}

fn update(app: &mut App, state: &mut State) {
    // Set the container size to the window size. This is done here to take in account the size when the user
    // resize the window, but can be done listening the resize event too.
    state.scaler.set_container_size(app.width(), app.height());

    // We set the position of our screen to the center of the window (the anchor is already set)
    state
        .scaler
        .set_position(app.width() * 0.5, app.height() * 0.5);

    // With the keyboard we switch between scale modes
    if app.keyboard.was_pressed(KeyCode::A) {
        state.scaler.set_mode(ScalerMode::None);
    } else if app.keyboard.was_pressed(KeyCode::S) {
        state.scaler.set_mode(ScalerMode::Fill);
    } else if app.keyboard.was_pressed(KeyCode::D) {
        state.scaler.set_mode(ScalerMode::AspectFill);
    } else if app.keyboard.was_pressed(KeyCode::F) {
        state.scaler.set_mode(ScalerMode::AspectFit);
    }
}

fn draw(app: &mut App, state: &mut State) {
    let (width, height) = state.scaler.working_size();

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    // First thing is push the matrix calculated by the scaler
    draw.push(state.scaler.matrix());

    // Draw a background that covers all the working size
    draw.color = Color::new(0.5, 0.4, 0.3, 1.0);
    draw.rect(0.0, 0.0, width, height);

    // Draw some random shapes to see how the container change with the mode
    draw.color = Color::YELLOW;
    draw.circle(200.0, 200.0, 50.0);
    draw.stroke_rounded_rect(400.0, 400.0, 100.0, 100.0, 40.0, 10.0);

    // Draw the rust image
    draw.image_resized(
        &state.texture,
        100.0,
        100.0,
        state.texture.width() * 0.5,
        state.texture.height() * 0.5,
    );

    // Just help text
    draw.color = Color::WHITE;
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.text(
        &state.font,
        &format!("Mode enabled: {:?}", state.scaler.mode()),
        width * 0.5,
        height * 0.5,
        40.0,
    );

    // Just help text
    draw.color = Color::from_hex(0xc0c0c0ff);
    draw.text(&state.font, "Press A to disable", 10.0, 10.0, 20.0);
    draw.text(&state.font, "Press S to enable Fill", 10.0, 30.0, 20.0);
    draw.text(
        &state.font,
        "Press D to enable AspectFill",
        10.0,
        50.0,
        20.0,
    );
    draw.text(&state.font, "Press F to enable AspectFit", 10.0, 70.0, 20.0);

    // Just help text
    draw.text(
        &state.font,
        "Resize the screen to see how the container changes",
        width * 0.5,
        height * 0.5 + 250.0,
        20.0,
    );

    // Pop the matrix once we finish working
    draw.pop();
    draw.end();
}
