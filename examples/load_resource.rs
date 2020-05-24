use nae::prelude::*;

struct State {
    to_load: Vec<String>,
    font: Font,
    resources: Vec<Texture>,
    time: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    State {
        to_load: file_names(),
        font: app.load_resource("./examples/assets/Ubuntu-B.ttf").unwrap(),
        resources: Vec::with_capacity(20),
        time: 0.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    // Load one texture each 0.5 seconds to do it slowly and have time to display the load bar
    if state.time >= 0.5 {
        state.time = 0.0;
        if let Some(file) = state.to_load.pop() {
            let texture = app
                .load_resource(&format!("./examples/assets/{}", file))
                .unwrap();
            state.resources.push(texture);
        }
    }

    state.time += app.delta;
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::ORANGE);

    // Resource progress
    let capacity = state.resources.capacity();
    let loaded = loaded_length(&state.resources);
    let percent = loaded as f32 / capacity as f32;
    let width = 400.0 * percent;

    // Draw a load bar
    draw.rounded_rect(200.0, 280.0, 400.0, 40.0, 10.0);
    draw.color = Color::GREEN;
    draw.rounded_rect(200.0, 280.0, width, 40.0, 10.0);
    draw.color = Color::BLACK;
    draw.stroke_rounded_rect(200.0, 280.0, 400.0, 40.0, 10.0, 10.0);

    if loaded == capacity {
        draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
        draw.text(&state.font, "All Loaded", 400.0, 300.0, 20.0);
    }

    draw.end();
}

fn loaded_length(resources: &[Texture]) -> usize {
    resources
        .iter()
        .filter(|res| res.is_loaded())
        .collect::<Vec<_>>()
        .len()
}

fn file_names() -> Vec<String> {
    [
        "bunny.png",
        "cube.png",
        "ferris.png",
        "ferris_chef.png",
        "golem-walk.png",
        "green_panel.png",
        "grey_button.png",
        "pixelExplosion00.png",
        "pixelExplosion01.png",
        "pixelExplosion02.png",
        "pixelExplosion03.png",
        "pixelExplosion04.png",
        "pixelExplosion05.png",
        "pixelExplosion06.png",
        "pixelExplosion07.png",
        "pixelExplosion08.png",
        "rust.png",
        "sunnyland.png",
        "t.png",
        "tile.png",
    ]
    .iter()
    .map(|f| f.to_string())
    .collect::<Vec<_>>()
}
