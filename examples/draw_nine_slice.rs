use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    img1: Texture,
    img2: Texture,
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
        .set_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    let texture1 = gfx
        .create_texture()
        .from_image(include_bytes!("assets/green_panel.png"))
        .build()
        .unwrap();

    let texture2 = gfx
        .create_texture()
        .from_image(include_bytes!("assets/grey_button.png"))
        .build()
        .unwrap();
    State {
        img1: texture1,
        img2: texture2,
        count: 1.0,
        multi: 1.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    state.count(60.0 * app.timer.delta_f32());
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.nine_slice(&state.img1).position(50.0, 50.0).size(
        state.img1.width() + state.count,
        state.img1.height() + state.count,
    );

    draw.nine_slice(&state.img2).position(450.0, 50.0).size(
        state.img2.width() + state.count,
        state.img2.height() + state.count,
    );

    gfx.render(&draw);
}
