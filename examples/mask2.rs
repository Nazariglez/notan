use nae::prelude::*;

struct State {
    image: Texture,
    offset: f32,
}

fn init(app: &mut App) -> State {
    State {
        image: app.load_file("../assets/t.png").unwrap(),
        offset: 0.0,
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    //Mask
    draw.begin_mask();
    draw.stroke_triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0, 20.0);
    draw.circle(400.0, 350.0, 30.0);
    draw.stroke_circle(400.0, 350.0, 50.0, 10.0);
    draw.stroke_circle(400.0, 350.0, 70.0, 10.0);
    draw.end_mask();

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
    draw.end();

    state.offset += 2.5 * app.delta();
}

#[nae::main]
fn main() {
    nae::with_state(init).draw(draw).build();
}
