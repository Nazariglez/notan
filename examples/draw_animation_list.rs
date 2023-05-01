use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    textures: Vec<Texture>,
    time: f32,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let texture_data = vec![
            include_bytes!("assets/pixelExplosion00.png").to_vec(),
            include_bytes!("assets/pixelExplosion01.png").to_vec(),
            include_bytes!("assets/pixelExplosion02.png").to_vec(),
            include_bytes!("assets/pixelExplosion03.png").to_vec(),
            include_bytes!("assets/pixelExplosion04.png").to_vec(),
            include_bytes!("assets/pixelExplosion05.png").to_vec(),
            include_bytes!("assets/pixelExplosion06.png").to_vec(),
            include_bytes!("assets/pixelExplosion07.png").to_vec(),
            include_bytes!("assets/pixelExplosion08.png").to_vec(),
        ];

        let textures = texture_data
            .iter()
            .map(|d| gfx.create_texture().from_image(d).build().unwrap())
            .collect();

        Self {
            textures,
            time: 0.0,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // An animation list take a &[&Texture] as frames
    draw.animation_list(&state.textures.iter().collect::<Vec<_>>())
        // just a matrix scale
        .scale(2.0, 2.0)
        // just a matrix translation
        .translate(300.0, 200.0)
        // normalized frame time
        .time(state.time);

    gfx.render(&draw);
    state.time += app.timer.delta_f32() * 2.0;
}
