use nae::prelude::{m2d::TextureAtlas, *};
use std::collections::HashMap;

struct State {
    atlas: TextureAtlas,
}

fn init(app: &mut App) -> State {
    State {
        atlas: app
            .load_resource("./examples/assets/sunnyland.json")
            .unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    if !state.atlas.is_loaded() {
        return;
    }

    let textures = state.atlas.textures();

    let draw = app.draw();
    draw.begin(Color::ORANGE);

    // Draw all the frames
    let mut xx = 10.0;
    let mut yy = 10.0;
    for (_, tex) in textures.iter() {
        draw.image(tex, xx, yy);
        xx += tex.width() + 4.0;
    }

    // Draw patterns using the textures on the atlas
    draw.pattern(
        textures.get("tree").unwrap(),
        400.0,
        150.0,
        320.0,
        300.0,
        0.0,
        0.0,
    );
    // draw.image(textures.get("door").unwrap(), 750.0, 550.0);
    draw.pattern(
        textures.get("house").unwrap(),
        100.0,
        150.0,
        280.0,
        300.0,
        0.0,
        0.0,
    );

    draw.pattern(
        textures.get("face-block").unwrap(),
        100.0,
        500.0,
        600.0,
        80.0,
        0.0,
        0.0,
    );

    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
