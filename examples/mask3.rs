use nae::prelude::*;

struct State {
    geom: Geometry,
    rot: f32,
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn init(_: &mut App) -> State {
    let mut geom = Geometry::new();
    for x in 0..10 {
        for y in 0..10 {
            geom.circle((x as f32) * 20.0 - 200.0, (y as f32) * 20.0 - 200.0, 5.0);
        }
    }
    geom.fill(Color::WHITE);

    State {
        geom: geom,
        rot: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.push_translate(400.0, 300.0);
    draw.push_rotation(state.rot * math::PI / 180.0);

    draw.stroke_rect(-210.0, -210.0, 200.0, 200.0, 5.0);

    draw.begin_mask();
    draw.clear(Color::TRANSPARENT);
    draw.geometry(&mut state.geom);
    draw.end_mask();

    draw.pop_matrix();
    draw.pop_matrix();

    draw.set_color(Color::GREEN);
    draw.triangle(400.0, 120.0, 200.0, 400.0, 600.0, 400.0);

    draw.clear_mask();

    draw.set_color(Color::WHITE);
    draw.stroke_triangle(400.0, 120.0, 200.0, 400.0, 600.0, 400.0, 5.0);
    draw.end();

    state.rot += 0.2;
}
