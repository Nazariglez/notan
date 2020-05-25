use nae::prelude::*;

struct State {
    blob: Blob,
    font: Font,
}

fn init(app: &mut App) -> State {
    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        blob: Blob::from_bytes(app, include_bytes!("assets/blob.txt")).unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    if state.blob.is_loaded() {
        draw.text(
            &state.font,
            &format!("Blob: {:?}", state.blob.data()),
            10.0,
            10.0,
            24.0,
        );
        draw.text(
            &state.font,
            &format!(
                "Text from blob: {:?}",
                std::str::from_utf8(&state.blob.data()).unwrap()
            ),
            10.0,
            60.0,
            24.0,
        );
    }
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
