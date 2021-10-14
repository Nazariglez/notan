use notan::prelude::*;

#[derive(AppState)]
struct State {
    img: Texture,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .set_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();
    State { img: texture }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.img).position(250.0, 200.0);
    gfx.render(&draw);
}
