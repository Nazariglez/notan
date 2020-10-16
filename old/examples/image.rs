use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    Texture::from_bytes(app, include_bytes!("assets/rust.png")).unwrap()
}

fn draw(app: &mut App, logo: &mut Texture) {
    let draw = app.draw();
    draw.begin(Color::ORANGE);
    draw.image(logo, 160.0, 60.0);
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
