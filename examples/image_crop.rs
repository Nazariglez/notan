use nae::prelude::*;

fn init(app: &mut App) -> nae_gfx::texture::Texture {
    nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/rust.png")).unwrap()
}

fn draw(app: &mut App, logo: &mut nae_gfx::texture::Texture) {
    let draw = app.draw2();
    draw.begin(Color::WHITE);
    draw.image_crop(logo, 0.0, 0.0, 0.0, 0.0, logo.width() * 0.5, 0.0);
    draw.image_crop(
        logo,
        logo.width() * 0.6,
        150.0,
        0.0,
        logo.height() * 0.5,
        0.0,
        logo.height() * 0.5,
    );
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
