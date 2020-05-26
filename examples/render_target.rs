use nae::prelude::*;

struct State {
    target1: RenderTarget,
    target2: RenderTarget,
    count: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let target1 = RenderTarget::from_size(app, app.width() as _, app.height() as _, false).unwrap();
    let target2 = RenderTarget::from_size(app, app.width() as _, app.height() as _, false).unwrap();
    State {
        target1,
        target2,
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let width = app.width();
    let height = app.height();
    let half_w = width * 0.5;
    let half_h = height * 0.5;

    let x = ((app.mouse.x / width) - 0.5) * 10.0;
    let y = ((app.mouse.y / height) - 0.5) * 10.0;

    let draw = app.draw();

    // Draw target1 and some primitives to target2
    draw.begin_to(
        &state.target2,
        Color::new((0.0 + state.count) % 1.0, 0.0, 0.0, 1.0),
    );
    draw.color = Color::WHITE;
    draw.image_resized(
        &state.target1.texture,
        20.0 + x,
        20.0 + y,
        width - 40.0,
        height - 40.0,
    );
    draw.stroke_rounded_rect(0.0, 0.0, width, height, 20.0, 20.0);
    draw.color = Color::YELLOW;
    draw.circle(half_w + x, half_h + y, 25.0);
    draw.color = Color::PINK;
    draw.stroke_circle(half_w + x, half_h + y, 25.0, 10.0);
    draw.end();

    // Draw target to the screen
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.color = Color::WHITE;
    draw.image(&state.target2.texture, 0.0, 0.0);
    draw.end();

    // Swap target locations
    std::mem::swap(&mut state.target1, &mut state.target2);
    state.count += app.delta;
}
