use nae::prelude::*;
use std::collections::HashMap;

struct State {
    tex: Texture,
    atlas: HashMap<String, Texture>,
}

fn init(app: &mut App) -> State {
    State {
        tex: app.load_file("./examples/assets/sunnyland.png").unwrap(),
        atlas: HashMap::new(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    if state.tex.is_loaded() && !state.atlas.contains_key("house") {
        state.atlas.insert(
            "house".to_string(),
            state.tex.with_frame(2.0, 223.0, 87.0, 108.0),
        );
        state.atlas.insert(
            "tree".to_string(),
            state.tex.with_frame(2.0, 128.0, 105.0, 93.0),
        );
    }

    let draw = app.draw();
    draw.begin();
    draw.clear(Color::ORANGE);
    draw.image(&state.tex, 0.0, 0.0);
    draw.set_color(Color::RED);
    draw.stroke_rect(0.0, 0.0, state.tex.width(), state.tex.height(), 2.0);

    draw.set_color(Color::WHITE);
    if let Some(t) = state.atlas.get("house") {
        draw.image(t, 300.0, 300.0);
    }

    if let Some(t) = state.atlas.get("tree") {
        for i in 0..3 {
            draw.image(t, 400.0 + t.width() * i as f32, 0.0)
        }
    }

    draw.end();
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
