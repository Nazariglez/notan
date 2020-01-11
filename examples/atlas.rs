use nae::extras::TextureAtlas;
use nae::prelude::*;
use std::collections::HashMap;

// TODO Warning: experimental support it's not done yet.

struct State {
    atlas: TextureAtlas,
}

fn init(app: &mut App) -> State {
    State {
        atlas: app.load_file("./examples/assets/sunnyland.json").unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    if !state.atlas.is_loaded() {
        return;
    }
    let textures = state.atlas.textures();

    let house = textures.get("house").unwrap();
    let tree = textures.get("tree").unwrap();
    let door = textures.get("door").unwrap();

    let draw = app.draw();
    draw.begin();
    draw.clear(Color::ORANGE);

    draw.image(house, 300.0, 300.0);

    for i in 0..3 {
        draw.image(tree, 400.0 + tree.width() * i as f32, 0.0)
    }

    for i in 0..20 {
        draw.image(door, 20.0 + door.width() * i as f32, 200.0);
    }

    draw.pattern(
        house,
        10.0,
        300.0,
        house.width() * 2.0,
        house.height() * 2.0,
        0.0,
        0.0,
    );
    //    panic!();
    draw.image_crop(
        house,
        550.0,
        300.0,
        house.width() * 0.5,
        house.height() * 0.5,
        house.width() * 0.5,
        house.height() * 0.5,
    );

    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
