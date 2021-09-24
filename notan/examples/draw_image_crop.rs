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
        .from_image(include_bytes!("assets/rust.png"))
        .build()
        .unwrap();
    State { img: texture }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (ww, hh) = state.img.size();

    let mut draw = gfx.create_draw();
    draw.clear(Color::WHITE);

    // Right side of the logo
    draw.image(&state.img)
        .position(100.0, 50.0)
        .size(ww * 0.5, hh)
        .crop((ww * 0.5, 0.0), (ww * 0.5, hh));

    // Left side of the logo
    draw.image(&state.img)
        .position(450.0, 50.0)
        .size(ww * 0.5, hh)
        .crop((0.0, 0.0), (ww * 0.5, hh));

    gfx.render(&draw);
}
