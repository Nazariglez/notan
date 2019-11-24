use nae::prelude::*;

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> Font {
    Font::default()
}

fn draw(app: &mut App, font: &mut Font) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.set_color(Color::Red);
    //    draw.text(&font, "Hello World! - - - - - ", 0.0, 0.0);
    draw.text(&font, "aaaa aaaa lalalalalala !!!! - - ", 0.0, 0.0);
    draw.set_color(Color::Green);
    draw.text(&font, "Hello world! This is hard...", 0.0, 150.0);
    draw.end();
}
