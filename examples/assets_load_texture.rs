use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    texture: Asset<Texture>,
}

impl State {
    fn new(assets: &mut Assets, gfx: &mut Graphics) -> Self {
        // Start loading the texture
        let texture = assets
            .load_asset(&asset_path("rust-logo-512x512.png"))
            .unwrap();

        // Load a font only for debug info
        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();

        Self { font, texture }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Draw the image if the lock returns something or the "Loading" text otherwise
    if state.texture.is_loaded() {
        // To access the inner texture we need to lock the reference
        draw.image(&state.texture.lock().unwrap())
            .position(150.0, 50.0);
    } else {
        draw.text(&state.font, "Loading...")
            .position(10.0, 10.0)
            .size(25.0);
    }

    gfx.render(&draw);
}

// The relative path for the example is different on browsers
fn asset_path(path: &str) -> String {
    let base = if cfg!(target_arch = "wasm32") {
        "./assets"
    } else {
        "./examples/assets"
    };

    format!("{base}/{path}")
}
