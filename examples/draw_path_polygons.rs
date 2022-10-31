use notan::draw::*;
use notan::prelude::*;
use std::f32::consts::PI;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
        .add_config(WindowConfig::default().multisampling(8))
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw_star(&mut draw, 100.0, 100.0, 5, 60.0, 30.0);
    draw_star(&mut draw, 700.0, 100.0, 12, 60.0, 30.0);
    draw_star(&mut draw, 100.0, 500.0, 6, 60.0, 30.0);
    draw_star(&mut draw, 700.0, 500.0, 20, 60.0, 40.0);

    draw_polygon(&mut draw, 200.0, 300.0, 5, 60.0);
    draw_polygon(&mut draw, 400.0, 300.0, 6, 60.0);
    draw_polygon(&mut draw, 600.0, 300.0, 8, 60.0);

    gfx.render(&draw);
}

fn draw_star(
    draw: &mut Draw,
    center_x: f32,
    center_y: f32,
    spikes: usize,
    outer_radius: f32,
    inner_radius: f32,
) {
    let step = PI / spikes as f32;

    let mut path_builder = draw.path();
    path_builder.move_to(center_x, center_y - outer_radius);

    let mut rot = PI / 2.0 * 3.0;
    for _ in 0..spikes {
        let mut x = center_x + rot.cos() * outer_radius;
        let mut y = center_y + rot.sin() * outer_radius;
        rot += step;

        path_builder.line_to(x, y);

        x = center_x + rot.cos() * inner_radius;
        y = center_y + rot.sin() * inner_radius;
        rot += step;

        path_builder.line_to(x, y);
    }

    path_builder
        .line_to(center_x, center_y - outer_radius)
        .close()
        .color(Color::ORANGE)
        .stroke(4.0);
}

fn draw_polygon(draw: &mut Draw, center_x: f32, center_y: f32, sides: usize, radius: f32) {
    let mut path_builder = draw.path();
    path_builder.move_to(center_x + radius, center_y);

    for i in 1..sides {
        let i = i as f32;
        let sides = sides as f32;

        let offset = -PI / 2.0;
        let angle = i * 2.0 * PI / sides + offset;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();

        path_builder.line_to(x, y);
    }

    path_builder.color(Color::ORANGE).stroke(4.0);
}
