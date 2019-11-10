use nae::prelude::*;

struct State {
    image: Texture,
    count: f32,
}

fn init(app: &mut App) -> State {
    State {
        image: app.load("../assets/t.png").unwrap(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.pattern_ext(
        &mut state.image,
        10.0,
        10.0,
        780.0,
        580.0,
        0.0,
        0.0,
        2.0 + state.count.sin(),
        2.0 + state.count.cos(),
    );
    draw.end();

    state.count += 0.005 * app.delta();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build();
}
