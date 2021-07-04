use notan::prelude::*;

struct Bunny {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

struct State {
    texture: Texture,
    rng: Random,
    bunnies: Vec<Bunny>,
    fps: f32,
}

impl AppState for State {}
impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let image = TextureInfo::from_image(include_bytes!("assets/bunny.png")).unwrap();
        let texture = gfx.create_texture(image).unwrap();

        Self {
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

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    // draw.background(Color::new(0.1, 0.2, 0.3, 1.0));
    state.bunnies.iter().for_each(|b| {
        draw.image(&state.texture).position(b.x, b.y);
    });

    gfx.render(&draw);
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        // .set_plugin(FpsPlugin(0.0))
        .update(update)
        .draw(draw)
        .build()
}
//
// struct FpsPlugin(f32);
// impl Plugin for FpsPlugin {
//     fn pre_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
//         Ok(Default::default())
//     }
//
//     fn post_frame(&mut self, app: &mut App) -> Result<AppFlow, String> {
//         Ok(Default::default())
//     }
// }
