use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    dragging: usize,
    asset: Option<Asset<Texture>>,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .add_config(notan::log::LogConfig::new(notan::log::LevelFilter::Debug))
        .draw(draw)
        .event(event)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    State {
        font,
        dragging: 0,
        asset: None,
    }
}

fn event(assets: &mut Assets, state: &mut State, evt: Event) {
    match evt {
        Event::DragEnter { .. } => {
            state.dragging += 1;
        }
        Event::DragLeft => {
            state.dragging = 0;
        }
        Event::Drop(file) => {
            state.dragging = 0;

            // Start loading the file if it's a png
            if file.name.contains(".png") {
                state.asset = Some(assets.load_dropped_file::<Texture>(&file).unwrap());
            }
        }
        _ => {}
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Display the image if it's already loaded
    if let Some(asset) = &state.asset {
        if let Some(img) = asset.lock() {
            draw.image(&img).position(30.0, 30.0).scale(0.5, 0.5);
        }
    }

    // Just UI Text
    if state.dragging == 0 {
        let text = match &state.asset {
            None => "Drop a PNG here",
            Some(asset) => {
                if asset.is_loaded() {
                    "Drop another PNG here"
                } else {
                    "Loading..."
                }
            }
        };

        draw.text(&state.font, text)
            .color(Color::ORANGE)
            .size(30.0)
            .v_align_middle()
            .h_align_center()
            .position(400.0, 300.0);
    } else {
        draw.rect((10.0, 10.0), (780.0, 580.0))
            .color(Color::WHITE)
            .stroke(6.0);

        let text = format!("You're dragging {} files", state.dragging);
        draw.text(&state.font, &text)
            .size(30.0)
            .color(Color::GRAY)
            .v_align_middle()
            .h_align_center()
            .position(400.0, 300.0);
    }

    gfx.render(&draw);
}
