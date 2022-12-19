use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().add_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let center_x: f32 = 400.0; // x-coordinate of center of arc
    let center_y: f32 = 300.0; // y-coordinate of center of arc
    let radius: f32 = 150.0; // radius of arc

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    {
        let mut path = draw.path();
        path.move_to(center_x, center_y);
        draw_arc(&mut path, center_x, center_y, radius, 120.0, 240.0);
        path.color(Color::GREEN).stroke(4.0);
    }
    {
        let mut path = draw.path();
        path.move_to(center_x, center_y);
        draw_circle_section(&mut path, center_x, center_y, radius, 240.0, 360.0);
        path.color(Color::BLUE).stroke(4.0).fill();
    }
    {
        let mut path = draw.path();
        path.move_to(center_x, center_y);
        draw_arc(&mut path, center_x, center_y, radius, 0.0, 120.0);
        path.color(Color::RED).stroke(4.0);
    }
    gfx.render(&draw);
}

fn draw_arc(
    path: &mut Path,
    center_x: f32,
    center_y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> &mut Path {
    let start_angle = start_angle + 270.0;
    let end_angle = end_angle + 270.0;

    let (start_x, start_y) = get_coords(center_x, center_y, radius, start_angle);
    path.move_to(start_x, start_y);
    for degrees in ((start_angle as u32)..(end_angle as u32)).step_by(5) {
        let (x1, y1) = get_coords(center_x, center_y, radius, degrees as f32);
        path.line_to(x1, y1);
    }
    let (end_x, end_y) = get_coords(center_x, center_y, radius, end_angle as f32);
    path.line_to(end_x, end_y);
    path
}

fn draw_circle_section(
    path: &mut Path,
    center_x: f32,
    center_y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> &mut Path {
    let start_angle = start_angle + 270.0;
    let end_angle = end_angle + 270.0;

    let (start_x, start_y) = get_coords(center_x, center_y, radius, start_angle);
    path.line_to(start_x, start_y);
    for degrees in ((start_angle as u32)..(end_angle as u32)).step_by(5) {
        let (x1, y1) = get_coords(center_x, center_y, radius, degrees as f32);
        path.line_to(x1, y1);
    }
    let (end_x, end_y) = get_coords(center_x, center_y, radius, end_angle as f32);
    path.line_to(end_x, end_y);
    path.line_to(center_x, center_y);
    path
}

fn get_coords(center_x: f32, center_y: f32, radius: f32, degrees: f32) -> (f32, f32) {
    let x = center_x + radius * (degrees as f32 * std::f32::consts::PI / 180.0).cos();
    let y = center_y + radius * (degrees as f32 * std::f32::consts::PI / 180.0).sin();
    (x, y)
}
