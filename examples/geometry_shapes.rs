use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(|_| State::new()).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin();
    draw.clear(Color::WHITE);
    draw.geometry(&state.geom);
    draw.end();
}

struct State {
    geom: Geometry,
}

impl State {
    fn new() -> Self {
        let x = 400.0;
        let y = 300.0;
        let mut geom = Geometry::new();

        // Fill shapes
        geom.circle(100.0, 100.0, 80.0);
        geom.rect(200.0, 20.0, 200.0, 200.0);
        geom.triangle(420.0, 220.0, 520.0, 20.0, 620.0, 220.0);
        geom.fill(Color::RED);

        // Stroke shapes
        geom.circle(200.0, 400.0, 80.0);
        geom.rect(300.0, 320.0, 200.0, 200.0);
        geom.triangle(520.0, 520.0, 620.0, 320.0, 720.0, 520.0);
        geom.stroke(Color::BLUE, 10.0);

        Self { geom }
    }
}
