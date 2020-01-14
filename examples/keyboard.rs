use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .event(event)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> (i32, i32) {
    (0, 0)
}

fn event(app: &mut App, mouse: &mut (i32, i32), evt: Event) {
    match evt {
        _ => {}
    }
}

fn draw(app: &mut App, mouse: &mut (i32, i32)) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::WHITE);
    draw.text(
        &format!("x: {} - y: {}", mouse.0, mouse.1),
        188.0,
        260.0,
        80.0,
    );

    draw.set_color(Color::RED);
    draw.circle(mouse.0 as _, mouse.1 as _, 10.0);

    draw.end();
}
