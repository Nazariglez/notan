use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    app.load("../assets/rust.png").unwrap()
}

fn draw(app: &mut App, logo: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.pattern(logo, 100.0, 100.0, 600.0, 400.0);
    draw.end();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
