use nae::prelude::*;

struct State {
    image: Texture,
    offset: f32,
}

fn init(app: &mut App) -> State {
    State {
        image: app.load("../assets/t.png").unwrap(),
        offset: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.pattern(
        &mut state.image,
        10.0,
        10.0,
        780.0,
        580.0,
        state.offset,
        -state.offset,
    );
    draw.end();

    state.offset += 2.5 * app.delta();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build();
}
