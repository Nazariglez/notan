use nae::prelude::*;

struct State {
    logo: Texture,
}

fn init(app: &mut App) -> State {
    State {
        logo: app.load("../assets/rust.png").unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.image(&mut state.logo, 160.0, 60.0);
    draw.end();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
