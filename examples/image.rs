use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    app.load("../assets/t.png").unwrap()
}

fn draw(app: &mut App, logo: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
//    draw.clear(Color::White);
    draw.transform().translate(160.0, 60.0);
    draw.transform().scale(7.0, 7.0);
    draw.set_color(rgba(1.0, 0.2, 0.3, 1.0));
    draw.image(logo, 0.0, 0.0);
    draw.transform().pop();
    draw.transform().pop();
    draw.end();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
