use notan::draw::*;
use notan::prelude::*;
use std::collections::HashMap;

#[derive(AppState)]
struct State {
    base_texture: Texture,
    atlas: HashMap<String, Texture>,
    font: Font,
    count: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(DrawConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    // Load base texture
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/sunnyland.png"))
        .build()
        .unwrap();

    // Load atlas data
    let data = include_bytes!("assets/sunnyland.json");

    // Create a hashmap of textures where the key is the name of the texture
    let atlas = create_textures_from_atlas(data, &texture).unwrap();

    // This font is only to show the name of the textures in the example
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        base_texture: texture,
        atlas,
        font,
        count: 0.0,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let mut x = 10.0;
    let mut y = 10.0;

    // Draw the base texture to have a visual reference
    draw_texture_with_name(
        x,
        y,
        &mut draw,
        "BaseTexture",
        &state.base_texture,
        &state.font,
    );

    let max_height = (gfx.size().1 as f32) * 0.8;
    let min_x = x + state.base_texture.width() + 50.0;
    x = min_x;

    // Draw all the textures in the HashMap created from the atlas data
    state.atlas.iter().for_each(|(k, tex)| {
        if y + tex.height() > max_height {
            y = 10.0;
            x += 200.0;
        }

        draw_texture_with_name(x, y, &mut draw, k, tex, &state.font);
        y += tex.height() + 30.0;
    });

    // Draw a pattern using a texture from the atlas
    draw.pattern(state.atlas.get("face-block").unwrap())
        .position(520.0, 320.0)
        .size(260.0, 260.0)
        .image_offset(state.count, state.count);

    state.count += app.timer.delta_f32() * 20.0;

    gfx.render(&draw);
}

fn draw_texture_with_name(x: f32, y: f32, draw: &mut Draw, id: &str, tex: &Texture, font: &Font) {
    let font_size = 16.0;

    draw.text(font, id)
        .color(Color::YELLOW)
        .size(font_size)
        .position(x, y);

    let y = y + font_size + 2.0;

    draw.rect((x, y), tex.size()).stroke(2.0).alpha(0.5);
    draw.image(tex).position(x, y);
}
