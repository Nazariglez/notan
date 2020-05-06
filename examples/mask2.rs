use nae::prelude::*;

struct State {
    image: nae_gfx::texture::Texture,
    offset: f32,
}

fn init(app: &mut App) -> State {
    State {
        image: nae_gfx::texture::Texture::from_bytes(app, include_bytes!("assets/t.png")).unwrap(),
        offset: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw2();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    //Mask
    draw.start_mask(|draw| {
        draw.stroke_triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0, 20.0);
        draw.circle(400.0, 350.0, 30.0);
        draw.stroke_circle(400.0, 350.0, 50.0, 10.0);
        draw.stroke_circle(400.0, 350.0, 70.0, 10.0);
    });

    //Draw on the mask
    draw.pattern(
        &mut state.image,
        10.0,
        10.0,
        780.0,
        580.0,
        state.offset,
        -state.offset,
    );

    draw.end_mask();
    draw.end();

    state.offset += 50.0 * app.delta;
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build();
}
