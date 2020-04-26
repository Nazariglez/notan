use nae::prelude::*;

fn init(app: &mut App) -> nae_gfx::texture::Texture {
    nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/rust.png")).unwrap()
}

fn draw(app: &mut App, logo: &mut nae_gfx::texture::Texture) {
    let draw = app.draw2();
    draw.begin(Color::ORANGE);
    draw.image(logo, 160.0, 60.0);
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
