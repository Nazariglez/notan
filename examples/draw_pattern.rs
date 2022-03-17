use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    img: Texture,
    count: f32,
    multi: f32,
}

impl State {
    pub fn count(&mut self, value: f32) {
        if self.count >= 200.0 || self.count <= 0.0 {
            self.multi *= -1.0;
        }

        self.count += value * self.multi;
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .update(|app: &mut App, state: &mut State| state.count(60.0 * app.timer.delta_f32()))
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/pattern.png"))
        .build()
        .unwrap();

    State {
        img: texture,
        count: 1.0,
        multi: 1.0,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.pattern(&state.img)
        .size(800.0, 600.0)
        .image_offset(-state.count, -state.count);

    gfx.render(&draw);
}
