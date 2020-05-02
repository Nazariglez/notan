use nae::prelude::*;

fn init(app: &mut App) -> nae_gfx::texture::Texture {
    nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/t.png")).unwrap()
}

fn draw(app: &mut App, tex: &mut nae_gfx::texture::Texture) {
    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.pattern(tex, 10.0, 10.0, 780.0, 580.0, 0.0, 0.0);
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build();
}
