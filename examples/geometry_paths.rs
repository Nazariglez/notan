use nae::prelude::*;

#[nae::main]
fn main() {
    nae::init_with(|_| State::new())
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn update(app: &mut App, state: &mut State) {
    state.update(app.delta);
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
    count: f32,
}

impl State {
    fn new() -> Self {
        Self {
            geom: Geometry::new(),
            count: 0.0,
        }
    }

    fn update(&mut self, delta: f32) {
        self.geom.clear();

        // Start drawing a cubic bezier line
        self.geom.cubic_bezier_to(
            0.0,
            100.0,
            100.0,
            100.0 + self.count.sin() * 100.0,
            200.0,
            100.0,
        );
        self.geom.cubic_bezier_to(
            200.0,
            100.0,
            300.0,
            100.0 - self.count.sin() * 100.0,
            400.0,
            100.0,
        );
        self.geom.cubic_bezier_to(
            400.0,
            100.0,
            500.0,
            100.0 + self.count.sin() * 100.0,
            600.0,
            100.0,
        );
        self.geom.cubic_bezier_to(
            600.0,
            100.0,
            700.0,
            100.0 - self.count.sin() * 100.0,
            800.0,
            100.0,
        );

        // Stroke it with some custom config
        self.geom.stroke_with_config(
            Color::RED,
            10.0,
            StrokeConfig {
                start_cap: LineCap::Round,
                end_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            },
        );

        // Start drawing another cubic bezier curve on the bottom of the screen
        self.geom.cubic_bezier_to(
            0.0,
            500.0,
            100.0,
            500.0 - self.count.sin() * 100.0,
            200.0,
            500.0,
        );
        self.geom.cubic_bezier_to(
            200.0,
            500.0,
            300.0,
            500.0 + self.count.sin() * 100.0,
            400.0,
            500.0,
        );
        self.geom.cubic_bezier_to(
            400.0,
            500.0,
            500.0,
            500.0 - self.count.sin() * 100.0,
            600.0,
            500.0,
        );
        self.geom.cubic_bezier_to(
            600.0,
            500.0,
            700.0,
            500.0 + self.count.sin() * 100.0,
            800.0,
            500.0,
        );

        // Stroke it with a custom config
        self.geom.stroke_with_config(
            Color::RED,
            10.0,
            StrokeConfig {
                start_cap: LineCap::Round,
                end_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            },
        );

        // Draw a quadratic bezier curve using this control point
        let ctrl_x = 400.0 + self.count.sin() * 100.0;
        let ctrl_y = 200.0;
        self.geom.move_to(300.0, 400.0);
        self.geom.quadratic_bezier_to(ctrl_x, ctrl_y, 500.0, 400.0);
        self.geom.stroke(Color::BLUE, 10.0);

        // Draw the control lines
        self.geom.move_to(300.0, 400.0);
        self.geom.line_to(ctrl_x, ctrl_y);
        self.geom.line_to(500.0, 400.0);
        self.geom.stroke(Color::GREEN, 2.0);

        self.count += delta * 1.5;
    }
}
