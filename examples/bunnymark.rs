use nae::prelude::*;

struct Bunny {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

struct State {
    img: Option<Texture>,
    bunnies: Vec<Bunny>,
}

fn on_start(app: &mut State, state: ()) {
    state.img = app.load("b.png").ok();
}

fn on_update(app: &mut State, state: ()) {
    for _ in 0..10 {
        state.bunnies.push(Bunny {
            x: 0.0,
            y: 0.0,
            speed_x: js_sys::Math::random() as f32 * 10.0,
            speed_y: js_sys::Math::random() as f32 * 10.0 - 5.0,
        });
    }

    state.bunnies.iter_mut().for_each(|b| {
        b.x += b.speed_x;
        b.y += b.speed_y;
        b.speed_y += 0.75;

        if b.x > 800.0 {
            b.speed_x *= -1.0;
            b.x = 800.0;
        } else if b.x < 0.0 {
            b.speed_x *= -1.0;
            b.x = 0.0
        }

        if b.y > 600.0 {
            b.speed_y *= -0.85;
            b.y = 600.0;
            if js_sys::Math::random() > 0.5 {
                b.speed_y -= js_sys::Math::random() as f32 * 6.0;
            }
        } else if b.y < 0.0 {
            b.speed_y = 0.0;
            b.y = 0.0;
        }
    });
}

fn on_draw(app: &mut App, state: ()) {
    let img = state.img.as_mut().unwrap();
    let gfx = &mut app.graphics;
    gfx.begin().clear(color::rgba(0.1, 0.2, 0.3, 1.0));
    for b in &state.bunnies {
        gfx.image(img, b.x, b.y);
    }
    gfx.end();
}

#[nae_start]
fn main() {
    nae::init({})
        .start(on_start)
        .update(on_update)
        .draw(on_draw)
        .build()
        .unwrap();
}
