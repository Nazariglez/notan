use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    app.load_file("../assets/green_panel.png").unwrap()
}

fn draw(app: &mut App, panel: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::WHITE);
    draw.image(panel, 10.0, 10.0);
    draw.image_9slice(panel, 200.0, 120.0, 400.0, 300.0);
    draw.end();
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}
