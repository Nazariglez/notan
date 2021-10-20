use notan::prelude::*;

// Create a new asset loaded to load .txt files as strings
fn create_text_loader() -> AssetLoader {
    AssetLoader::new().use_parser(parse_text).extension("txt")
}

// This parses the &[u8] from the file to the type that we want, string in this case
fn parse_text(_id: &str, data: Vec<u8>) -> Result<String, String> {
    String::from_utf8(data).map_err(|e| e.to_string())
}

#[derive(AppState)]
struct State {
    font: Font,
    text: Asset<String>,
}

impl State {
    fn new(assets: &mut Assets, gfx: &mut Graphics) -> Self {
        // Start loading the file
        let text = assets.load_asset(&asset_path("lorem.txt")).unwrap();

        // Load a font only for debug info
        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();

        Self { font, text: text }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        // Add the new loaded here
        .add_loader(create_text_loader())
        .set_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    if state.text.is_loaded() {
        draw.text(&state.font, &state.text.lock().unwrap())
            .max_width(750.0)
            .position(10.0, 10.0)
            .size(25.0);
    } else {
        draw.text(&state.font, "Loading...")
            .position(10.0, 10.0)
            .size(25.0);
    };

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
