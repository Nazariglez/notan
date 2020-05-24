use nae::extras::Random;
use nae::prelude::*;
use specs::prelude::*;
use specs::System;

// Componentes
struct Vel {
    x: f32,
    y: f32,
}

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

struct Pos {
    x: f32,
    y: f32,
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

// Movement system
struct MovementSys {
    rng: Random,
}

impl MovementSys {
    pub fn new() -> Self {
        Self {
            rng: Random::default(),
        }
    }
}

impl<'a> System<'a> for MovementSys {
    type SystemData = (WriteStorage<'a, Pos>, WriteStorage<'a, Vel>);

    fn run(&mut self, (mut pos, mut vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &mut vel).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            vel.y += 0.75;

            if pos.x > 800.0 {
                vel.x *= -1.0;
                pos.x = 800.0;
            } else if pos.x < 0.0 {
                vel.x *= -1.0;
                pos.x = 0.0
            }

            if pos.y > 600.0 {
                vel.y *= -0.85;
                pos.y = 600.0;
                if self.rng.gen::<bool>() {
                    vel.y -= self.rng.gen_range(0.0, 6.0);
                }
            } else if pos.y < 0.0 {
                vel.y = 0.0;
                pos.y = 0.0;
            }
        }
    }
}

// Spawn system
#[derive(Default)]
struct SpawnCounter {
    to_spawn: usize,
    spawned: usize,
}

#[derive(Default)]
struct SpawnSys {
    rng: Random,
}

impl<'a> System<'a> for SpawnSys {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        Write<'a, SpawnCounter>,
    );

    fn run(&mut self, (mut entities, mut pos, mut vel, mut counter): Self::SystemData) {
        let to_spawn = counter.to_spawn;
        if to_spawn > 0 {
            for _ in 0..to_spawn {
                counter.spawned += 1;

                entities
                    .build_entity()
                    .with(Pos { x: 0.0, y: 0.0 }, &mut pos)
                    .with(
                        Vel {
                            x: self.rng.gen_range(0.0, 10.0),
                            y: self.rng.gen_range(-5.0, 5.0),
                        },
                        &mut vel,
                    )
                    .build();
            }

            counter.to_spawn = 0;
        }
    }
}

// Game state
struct State {
    font: Font,
    bunny: Texture,
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl State {
    pub fn new(app: &mut App) -> Self {
        let mut world = World::new();

        world.register::<Pos>();
        world.register::<Vel>();
        world.insert(SpawnCounter {
            to_spawn: 10,
            spawned: 0,
        });

        let mut dispatcher = DispatcherBuilder::new()
            .with(SpawnSys::default(), "spawn_sys", &[])
            .with(MovementSys::new(), "movement_sys", &[])
            .build();

        dispatcher.dispatch(&world);

        Self {
            font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
            bunny: Texture::from_bytes(app, include_bytes!("assets/bunny.png")).unwrap(),
            world,
            dispatcher,
        }
    }
}

fn init(app: &mut App) -> State {
    State::new(app)
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn update(app: &mut App, state: &mut State) {
    if app.mouse.is_down(MouseButton::Left) {
        let mut counter = state.world.write_resource::<SpawnCounter>();
        counter.to_spawn = 50;
    }
    state.dispatcher.dispatch(&state.world);
    state.world.maintain();
}

// The draw should be down here and not in a specs component because window, opengl and the textures should live on the main thread
fn draw(app: &mut App, state: &mut State) {
    let bunnies = state.world.read_resource::<SpawnCounter>().spawned;
    let calls = app.gfx().draw_calls();
    let fps = app.fps().round();

    let draw = app.draw();
    draw.begin(Color::ORANGE);

    for pos in state.world.read_component::<Pos>().join() {
        draw.image(&state.bunny, pos.x, pos.y);
    }

    let debug_text = format!(
        "Bunnies: {} - Fps: {} - Draw Calls: {}",
        bunnies, fps, calls
    );
    draw.text(&state.font, &debug_text, 10.0, 1.0, 24.0);

    draw.end();
}
