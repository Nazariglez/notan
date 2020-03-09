use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    app.load_file("./examples/assets/t.png").unwrap()
}

fn draw(app: &mut App, tex: &mut Texture) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.pattern(tex, 10.0, 10.0, 780.0, 580.0, 0.0, 0.0);
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build();
}
