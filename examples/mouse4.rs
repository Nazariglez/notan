use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .event(event)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> (f32, f32) {
    (400.0, 300.0)
}

fn event(app: &mut App, pos: &mut (f32, f32), evt: Event) {
    match evt {
        Event::MouseWheel { delta_x, delta_y } => {
            pos.0 = (pos.0 + delta_x).max(0.0).min(800.0);
            pos.1 = (pos.1 + delta_y).max(0.0).min(600.0);
        }
        _ => {}
    }
}

fn draw(app: &mut App, pos: &mut (f32, f32)) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::WHITE);
    draw.text_ext(
        "Scroll with your mouse's wheel or touchpad",
        400.0,
        300.0,
        40.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    draw.set_color(Color::RED);
    draw.circle(pos.0, pos.1, 30.0);

    draw.end();
}
