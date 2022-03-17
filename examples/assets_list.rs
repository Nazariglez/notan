use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    list: AssetList,
    logo: Option<Texture>,
}

impl State {
    fn new(assets: &mut Assets, gfx: &mut Graphics) -> Self {
        // Define a list of assets to load asynchronously
        let list = assets
            .load_list(&[
                &asset_path("rust-logo-512x512.png"),
                &asset_path("rust-logo-256x256.png"),
                &asset_path("bunny.png"),
                &asset_path("ferris.png"),
                &asset_path("golem-walk.png"),
                &asset_path("green_panel.png"),
                &asset_path("grey_button.png"),
                &asset_path("kenney_pixel-webfont.ttf"),
                &asset_path("Ubuntu-B.ttf"),
                &asset_path("pattern.png"),
                &asset_path("sunnyland.png"),
            ])
            .unwrap();

        // Load a font only for debug info
        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();

        Self {
            font,
            list,
            logo: None,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(DrawConfig) // Simple way to add the draw extension
        .update(update)
        .draw(draw)
        .build()
}

fn update(state: &mut State) {
    let logo_path = asset_path("rust-logo-512x512.png");
    if state.list.is_loaded() && state.list.contains(&logo_path) {
        // Remove the asset from the list taking the ownership
        match state.list.take::<Texture>(&logo_path) {
            Ok(asset) => {
                let logo = asset.try_unwrap().unwrap();
                state.logo = Some(logo);
            }
            Err(err) => {
                notan::log::error!("Error taking asset from list: {}", err);
            }
        }
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let progress = state.list.progress();
    let total = state.list.len();
    let loaded = (progress * total as f32).floor();
    draw.text(
        &state.font,
        &format!(
            "Loading progress: {}% ({}/{})",
            progress * 100.0,
            loaded,
            total
        ),
    )
    .position(10.0, 10.0)
    .size(25.0);

    // If the list is loaded we draw the rust logo
    if let Some(tex) = &state.logo {
        draw.image(tex).position(150.0, 50.0);
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
