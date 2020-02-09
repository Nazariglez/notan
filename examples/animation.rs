use nae::extras::Animation;
use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(|app| State::new(app))
        .draw(draw)
        .update(update)
        .build()
        .unwrap();
}

fn update(app: &mut App, state: &mut State) {
    // add the frame time to calculate which frame we need to draw
    state.anim.tick(app.delta);
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.push_scale(2.0, 2.0);

    // draw the texture returned by the animation
    draw.image(state.anim.texture(), 60.0, 50.0);

    draw.pop_matrix();
    draw.end();
}

// Create an animation using textures as frames
fn create_animation(app: &mut App) -> Result<Animation, String> {
    let frames = vec![
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion00.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion01.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion02.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion03.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion04.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion05.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion06.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion07.png"))?,
        Texture::from_bytes(app, include_bytes!("./assets/pixelExplosion08.png"))?,
    ];

    Ok(Animation::new(frames, 0.1))
}

struct State {
    anim: Animation,
}

impl State {
    fn new(app: &mut App) -> Self {
        Self {
            anim: create_animation(app).unwrap(),
        }
    }
}
