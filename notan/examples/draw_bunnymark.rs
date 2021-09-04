use notan::app::config::WindowConfig;
use notan::draw::*;
use notan::glyph::*;
use notan::prelude::*;
use notan::utils::{Duration, Instant};
use notan_app::Plugins;

struct Bunny {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

#[derive(notan::AppState)]
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
            .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
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
                x: 0.0,
                y: 0.0,
                speed_x: self.rng.gen_range(0.0, 10.0),
                speed_y: self.rng.gen_range(-5.0, 5.0),
            })
        });
    }
}

fn init(gfx: &mut Graphics, plugins: &mut Plugins) -> State {
    let draw_ext = DrawPlugin::new(gfx).unwrap();
    gfx.plugins.set(draw_ext);

    let glyph_ext = GlyphManager::new(gfx).unwrap();
    plugins.set(glyph_ext);

    let mut state = State::new(gfx);
    state.spawn(5);
    state
}

fn update(app: &mut App, state: &mut State) {
    if app.mouse.left_is_down() {
        state.spawn(50);
    }

    let rng = &mut state.rng;
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
            if rng.gen::<bool>() {
                b.speed_y -= rng.gen_range(0.0, 6.0);
            }
        } else if b.y < 0.0 {
            b.speed_y = 0.0;
            b.y = 0.0;
        }
    });
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear([0.0, 0.0, 0.0, 1.0].into());
    state.bunnies.iter().for_each(|b| {
        draw.image(&state.texture).position(b.x, b.y);
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

    gfx.r(&draw);
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .set_config(WindowConfig::new().vsync())
        .update(update)
        .draw(draw)
        .build()
}
