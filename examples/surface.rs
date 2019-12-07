use nae::prelude::*;
use nae::prelude::surface::Surface;

#[nae_start]
fn main() {
    nae::with_state(|app| {
        Surface::from_size(app, 200, 200).unwrap()
    }).draw(draw).build().unwrap();
}

fn draw(app: &mut App, surface: &mut Surface) {
    let draw = app.draw();
    draw_to_surface(draw, surface);

    draw.begin();
    draw.clear(Color::GRAY);
    draw.set_color(Color::WHITE);
    draw.image(surface.texture(), 400.0, 300.0);
    draw.set_color(Color::GREEN);
    draw.circle(0.0, 0.0, 10.0);
    draw.set_color(Color::WHITE);
    draw.text_ext("Holi", 100.0, 100.0, 40.0, HorizontalAlign::Center, VerticalAlign::Center, None);
    draw.end();
}

fn draw_to_surface(draw: &mut Context2d, surface: &Surface) {
    draw.begin_to_surface(Some(surface));
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::GREEN);
    draw.circle(10.0, 10.0, 10.0);

    for i in (0..10) {
        draw.set_color(rgba(0.1*i as f32, 0.0, 0.0, 1.0));
        draw.circle(100.0, 100.0, 100.0-(5.0*i as f32));
    }

    draw.set_color(Color::WHITE);
    draw.text_ext("Holi", 100.0, 100.0, 40.0, HorizontalAlign::Center, VerticalAlign::Center, None);
    draw.end();

}