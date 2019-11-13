use nae::prelude::*;
use nae::math::Geometry;

struct State {
    geom: Geometry,
    rot: f32,
}

#[nae_start]
fn main() {
    nae::with_state(|_| {
        let mut geom = Geometry::new();
        for x in 0..20 {
            for y in 0..20 {
                geom.circle((x as f32) * 20.0 - 200.0, (y as f32) * 20.0 - 200.0, 5.0);
            }
        }

        State {
            geom: geom,
            rot: 0.0,
        }
    })
        .draw(draw)
        .build()
        .unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.transform().translate(400.0, 300.0);
    draw.transform().rotate_deg(state.rot);

//    draw.begin_mask();
    draw.geometry(&mut state.geom);
//    draw.end_mask();

    draw.transform().pop();
    draw.transform().pop();

    draw.set_color(Color::Green);
    draw.triangle(400.0, 120.0, 200.0, 400.0, 600.0, 400.0);
    draw.end();

    state.rot += 0.2;
}
