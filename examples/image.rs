use nae::prelude::*;
use nae_core::logger::LogLevel;
use nae_core::resources::Resource;

fn init(app: &mut App) -> Texture {
    logger::init(LogLevel::Trace);
    app.load_file("../assets/rust.png").unwrap()
}

fn draw(app: &mut App, logo: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::ORANGE);
    draw.image(logo, 160.0, 60.0);
    draw.end();
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
