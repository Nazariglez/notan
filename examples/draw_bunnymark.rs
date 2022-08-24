use notan::draw::*;
use notan::math::{vec2, Vec2};
use notan::prelude::*;

struct Bunny {
    pos: Vec2,
    speed: Vec2,
}

#[derive(AppState)]
struct State {
    font: Font,
    texture: Texture,
    rng: Random,
    bunnies: Vec<Bunny>,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/bunny.png"))
            .build()
            .unwrap();

        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();

        Self {
            font,
            texture,
            rng: Random::default(),
            bunnies: vec![],
        }
    }

    fn spawn(&mut self, n: i32) {
        (0..n).for_each(|_| {
            self.bunnies.push(Bunny {
                pos: Vec2::ZERO,
                speed: vec2(self.rng.gen_range(0.0..10.0), self.rng.gen_range(-5.0..5.0)),
            })
        });
    }
}

fn init(gfx: &mut Graphics) -> State {
    let mut state = State::new(gfx);
    state.spawn(1);
    state
}

fn update(app: &mut App, state: &mut State) {
    if app.mouse.left_is_down() {
        state.spawn(50);
    }

    let rng = &mut state.rng;
    state.bunnies.iter_mut().for_each(|b| {
        b.pos += b.speed;
        b.speed.y += 0.75;

        if b.pos.x > 800.0 {
            b.speed.x *= -1.0;
            b.pos.x = 800.0;
        } else if b.pos.x < 0.0 {
            b.speed.x *= -1.0;
            b.pos.x = 0.0
        }

        if b.pos.y > 600.0 {
            b.speed.y *= -0.85;
            b.pos.y = 600.0;
            if rng.gen::<bool>() {
                b.speed.y -= rng.gen_range(0.0..6.0);
            }
        } else if b.pos.y < 0.0 {
            b.speed.y = 0.0;
            b.pos.y = 0.0;
        }
    });
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear([0.0, 0.0, 0.0, 1.0].into());
    state.bunnies.iter().for_each(|b| {
        draw.image(&state.texture).position(b.pos.x, b.pos.y);
    });

    draw.text(
        &state.font,
        &format!(
            "{} -> {} ({:.6})",
            app.timer.fps().round(),
            state.bunnies.len(),
            app.timer.delta_f32()
        ),
    )
    .position(10.0, 10.0)
    .size(24.0);

    gfx.render(&draw);
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new().vsync(true))
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}
