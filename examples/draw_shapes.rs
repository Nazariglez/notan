use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().add_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.line((20.0, 30.0), (780.0, 30.0)).width(4.0);

    draw.triangle((100.0, 100.0), (150.0, 200.0), (200.0, 100.0))
        .color(Color::YELLOW);

    draw.rect((500.0, 100.0), (200.0, 150.0))
        .fill_color(Color::GREEN)
        .fill()
        .stroke_color(Color::WHITE)
        .stroke(15.0);

    draw.ellipse((400.0, 300.0), (50.0, 100.0))
        .color(Color::RED)
        .rotate_degrees(-45.0);

    draw.circle(40.0)
        .position(600.0, 450.0)
        .fill_color(Color::BLUE)
        .fill()
        .stroke_color(Color::WHITE)
        .stroke(5.0);

    draw.rect((100.0, 250.0), (150.0, 100.0))
        .corner_radius(20.0)
        .color(Color::ORANGE)
        .stroke(15.0);

    draw.star(10, 80.0, 40.0)
        .position(150.0, 480.0)
        .fill_color(Color::PINK)
        .fill()
        .stroke_color(Color::PURPLE)
        .stroke(6.0);

    draw.polygon(5, 50.0)
        .position(350.0, 150.0)
        .color(Color::WHITE)
        .stroke(8.0);

    draw.polygon(8, 80.0)
        .position(350.0, 450.0)
        .fill_color(Color::WHITE)
        .fill()
        .stroke_color(Color::ORANGE)
        .stroke(8.0);

    gfx.render(&draw);
}
