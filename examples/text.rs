use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init().draw(draw).build().unwrap();
}

fn draw(app: &mut App, _: &mut ()) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.text("Hello World!", 188.0, 260.0, 80.0);
    draw.end();
}
