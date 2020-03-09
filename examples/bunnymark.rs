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
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .event(event)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    let mut state = State {
        rng: Random::default(),
        img: app.load_file("./examples/assets/bunny.png").unwrap(),
        bunnies: vec![],
        spawning: false,
    };
    spawn(&mut state);
    state
}

fn event(app: &mut App, state: &mut State, event: Event) {
    match event {
        Event::MouseDown { .. } => {
            state.spawning = true;
        }
        Event::MouseUp { .. } => {
            state.spawning = false;
        }
        _ => {}
    }
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
    if state.spawning {
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

    let draw = app.draw();
    draw.begin();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));
    for b in &state.bunnies {
        draw.image(&state.img, b.x, b.y);
    }

    draw.text(
        &format!("Bunnies: {} - Fps: {}", state.bunnies.len(), fps),
        10.0,
        1.0,
        24.0,
    );
    draw.end();
}
