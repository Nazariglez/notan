use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    loading: Option<Asset<Texture>>,
    loaded: Option<Texture>,
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

        Self {
            font,
            loading: Some(texture),
            loaded: None,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(state: &mut State) {
    // Unwrapping an asset reference after it has been loaded we avoid the performance penalty
    // of the Arc<RwLock<T>> inside the Asset instance

    let was_just_loaded = state
        .loading
        .as_ref()
        .map(|assets| assets.is_loaded())
        .unwrap_or(false);

    if was_just_loaded {
        // Get ownership of the Asset inside the Option
        let asset = state.loading.take().unwrap();

        // Unwrap the asset reference to get the inner
        let texture = asset.try_unwrap().unwrap();
        state.loaded = Some(texture);
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    match &state.loaded {
        None => {
            draw.text(&state.font, "Loading...")
                .position(10.0, 10.0)
                .size(25.0);
        }
        Some(texture) => {
            draw.image(texture).position(150.0, 50.0);
        }
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

    format!("{}/{}", base, path)
}
