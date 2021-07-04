use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();

    draw.path()
        .move_to(10.0, 10.0)
        .line_to(100.0, 100.0)
        .line_to(400.0, 500.0)
        .quadratic_bezier_to((440.0, 440.0), (310.0, 210.0))
        .line_to(790.0, 590.0)
        .round_join()
        .color(Color::ORANGE)
        .stroke(10.0);

    gfx.render(&draw);
}
