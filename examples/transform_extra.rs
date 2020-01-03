use nae::extras::Transform2d;
use nae::prelude::*;

struct State {
    tex: Texture,
    transform: Transform2d,
}

#[nae::main]
fn main() {
    nae::with_state(init)
        .draw(draw)
        .update(update)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    let mut transform = Transform2d::new(0.0, 0.0);
    transform.set_anchor(0.5, 0.5).set_position(400.0, 300.0);

    State {
        tex: app.load_file("./examples/assets/ferris.png").unwrap(),
        transform: transform,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let img = &state.tex;

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.push_matrix(&state.transform.matrix());
    draw.image(img, 0.0, 0.0);
    draw.pop_matrix();

    draw.text(
        &format!(
            "pos: ({}, {})\nanchor: ({}, {})",
            state.transform.x,
            state.transform.y,
            state.transform.anchor_x,
            state.transform.anchor_y
        ),
        10.0,
        10.0,
        20.0,
    );

    draw.end();
}

// Sets the size of the texture on the transform once is loaded
fn update(app: &mut App, state: &mut State) {
    if state.transform.width != 0.0 {
        return;
    }

    if state.tex.is_loaded() {
        state.transform.width = state.tex.width();
        state.transform.height = state.tex.height();
    }
}
