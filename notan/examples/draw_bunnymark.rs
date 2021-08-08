use notan::prelude::*;
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
    fps: f64,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let image = TextureInfo::from_image(include_bytes!("assets/bunny.png")).unwrap();
        let texture = gfx.create_texture(image).unwrap();

        let font = gfx
            .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
            .unwrap();

        Self {
            font,
            texture,
            rng: Random::default(),
            bunnies: vec![],
            fps: 0.0,
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

fn init(gfx: &mut Graphics) -> State {
    let mut state = State::new(gfx);
    state.spawn(5);
    state
}

fn update(app: &mut App, plugins: &mut Plugins, state: &mut State) {
    let fps_plugin = plugins.get::<FpsPlugin>().unwrap();
    state.fps = fps_plugin.fps();
    let delta = fps_plugin.delta() as f32;

    if app.mouse.left_is_down() {
        state.spawn(50);
    }

    let rng = &mut state.rng;
    state.bunnies.iter_mut().for_each(|b| {
        b.x += b.speed_x * delta;
        b.y += b.speed_y * delta;
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

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear([0.0, 0.0, 0.0, 1.0].into());
    state.bunnies.iter().for_each(|b| {
        draw.image(&state.texture).position(b.x, b.y);
    });

    draw.text(
        &state.font,
        &format!("{} -> {}", state.fps.round(), state.bunnies.len()),
    )
    .position(10.0, 10.0);

    gfx.render(&draw);
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .set_plugin(FpsPlugin::new())
        .update(update)
        .draw(draw)
        .build()
}

use notan_app::AppFlow;
use std::collections::VecDeque;
struct FpsPlugin {
    fps: VecDeque<f64>,
    last_time: u64,
    last_delta: f64,
}

impl FpsPlugin {
    fn new() -> Self {
        let mut fps = VecDeque::with_capacity(300);
        fps.resize(fps.capacity(), 1000.0 / 60.0);

        Self {
            fps: fps,
            last_time: 0,
            last_delta: 0.0,
        }
    }

    fn tick(&mut self, now: u64) {
        let elapsed = (now - self.last_time) as f64;
        self.last_time = now;
        self.last_delta = elapsed / 1000.0;
        self.fps.pop_front();
        self.fps.push_back(elapsed);
    }

    pub fn fps(&self) -> f64 {
        let average: f64 = self.fps.iter().sum::<f64>() / self.fps.len() as f64;
        1000.0 / average
    }

    pub fn delta(&self) -> f64 {
        self.last_delta
    }
}

impl Plugin for FpsPlugin {
    fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
        self.tick(app.date_now());
        Ok(AppFlow::Next)
    }
}
