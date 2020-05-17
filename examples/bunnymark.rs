use nae::extras::Random;
use nae::prelude::*;

struct Bunny {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

struct State {
    rng: Random,
    img: Texture,
    bunnies: Vec<Bunny>,
    spawning: bool,
    font: Font,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    let mut state = State {
        rng: Random::default(),
        img: Texture::from_bytes(app, include_bytes!("assets/bunny.png")).unwrap(),
        bunnies: vec![],
        spawning: false,
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
    };
    spawn(&mut state);
    state
}

fn spawn(state: &mut State) {
    for _ in 0..50 {
        state.bunnies.push(Bunny {
            x: 0.0,
            y: 0.0,
            speed_x: state.rng.gen_range(0.0, 10.0),
            speed_y: state.rng.gen_range(-5.0, 5.0),
        });
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.mouse.is_down(MouseButton::Left) {
        spawn(state);
    }

    for b in &mut state.bunnies {
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
            if state.rng.gen::<bool>() {
                b.speed_y -= state.rng.gen_range(0.0, 6.0);
            }
        } else if b.y < 0.0 {
            b.speed_y = 0.0;
            b.y = 0.0;
        }
    }
}

fn draw(app: &mut App, state: &mut State) {
    let fps = app.fps().round();
    let calls = app.gfx().draw_calls();
    let bunnies = state.bunnies.len();

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    state
        .bunnies
        .iter()
        .for_each(|b| draw.image(&state.img, b.x, b.y));

    let debug_text = format!(
        "Bunnies: {} - Fps: {} - Draw Calls: {}",
        bunnies, fps, calls
    );
    draw.text(&state.font, &debug_text, 10.0, 1.0, 24.0);

    draw.end();
}
