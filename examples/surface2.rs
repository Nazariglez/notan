use nae::prelude::surface::Surface;
use nae::prelude::*;

struct Triangle {
    x: f32,
    y: f32,
    rotation: f32,
    color: Color,
}

struct State {
    surface_1: Surface,
    surface_2: Surface,
    triangles: Vec<Triangle>,
    count: f32,
}

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let width = app.draw().width();
    let height = app.draw().height();
    let colors = [
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::BLACK,
        Color::FUCHSIA,
        Color::YELLOW,
        Color::PINK,
    ];

    State {
        surface_1: Surface::from_size(app, width, height).unwrap(),
        surface_2: Surface::from_size(app, width, height).unwrap(),
        triangles: (0..8)
            .map(|i| Triangle {
                x: 200.0 + 25.0 * i as f32,
                y: 100.0 + 25.0 * i as f32,
                color: colors[i],
                rotation: 0.0,
            })
            .collect(),
        count: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin_to_surface(Some(&state.surface_1));
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    state.triangles.iter_mut().for_each(|t| {
        t.rotation -= 0.01;
        draw.set_color(t.color);
        draw.transform().translate(t.x, t.y);
        draw.transform().rotate(t.rotation);
        draw.rect(0.0, 0.0, 20.0, 20.0);
        draw.transform().pop();
        draw.transform().pop();
    });
    draw.end();

    let scale = 1.0 + state.count.sin() * 0.2;
    //    log(&format!("{}", scale));
    draw.begin_to_surface(Some(&state.surface_2));
    draw.set_color(Color::WHITE);
    //    draw.transform().scale(4.5, 4.5);
    draw.image(state.surface_1.texture(), 0.0, 0.0);
    //    draw.transform().pop();
    draw.end();

    draw.begin();
    //    draw.transform().scale(2.0, 2.0);
    draw.image(state.surface_2.texture(), 0.0, 0.0);
    draw.set_color(Color::GREEN);
    draw.circle(0.0, 0.0, 10.0);
    //    draw.transform().pop();
    draw.end();

    state.count += 1.0;
    //    std::mem::swap(&mut state.surface_1, &mut state.surface_2);
}
