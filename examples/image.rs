use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    app.load("../assets/rust.png").unwrap()
}

fn draw(app: &mut App, logo: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::White);
    draw.image(logo, 160.0, 60.0);
    draw.end();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
