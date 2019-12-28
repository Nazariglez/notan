use nae::prelude::*;

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> Surface {
    let surface = Surface::from_size(app, 200, 200).unwrap();
    draw_to_surface(app.draw(), &surface);
    surface
}

fn draw(app: &mut App, surface: &mut Surface) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.set_color(Color::WHITE);

    for y in 0..8 {
        for x in 0..12 {
            draw.image(surface.texture(), x as f32 * 55.0, y as f32 * 57.0);
        }
    }
    draw.end();
}

fn draw_to_surface(draw: &mut Context2d, surface: &Surface) {
    draw.begin_to_surface(Some(surface));
    for i in 0..20 {
        draw.set_color(rgba(0.05 * i as f32, 0.0, 0.0, 1.0));
        draw.circle(100.0, 100.0, 100.0 - (5.0 * i as f32));
    }

    draw.set_color(Color::WHITE);
    draw.text_ext(
        "Surface!",
        100.0,
        100.0,
        40.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    draw.end();
}
