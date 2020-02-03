use nae::prelude::*;

// This example is a port of javascript code made by Ibon Tolosona (@hyperandroid)
// https://codepen.io/hyperandroid/full/yLyRQmw
struct State {
    geom: Geometry,
    cx: f32,
    cy: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            geom: Geometry::new(),
            cx: 400.0,
            cy: 300.0,
        }
    }
}

#[nae::main]
fn main() {
    nae::init_with(|_| State::new()).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let time = app.time * 1000.0;

    let draw = app.draw();
    draw.begin();
    draw.clear(Color::BLACK);

    state.geom.clear();

    let mut count = 0;
    let mut line_index: f32 = 0.0;
    let start_radius: f32 = 20.0;
    let radius_increment = 13.0;
    let max_radius = 250.0;

    let max_lines = ((max_radius - start_radius) / radius_increment).ceil();

    for i in (start_radius as i32..max_radius as i32).step_by(radius_increment as usize) {
        let ti = ((time + line_index * 79.0) % 2000.0) / 2000.0;
        draw_flower(
            &mut state.geom,
            state.cx,
            state.cy,
            i as f32,
            30.0,
            6.0,
            i as f32,
            ((time % 38000.0) / 38000.0) * 2.0 * math::PI,
            count as f32,
            (ti * math::PI * 2.0).cos(),
        );
        count += if line_index <= max_lines / 2.0 { 2 } else { -2 };

        line_index += 1.0;
    }

    state.geom.stroke(Color::MAGENTA, 3.0);
    draw.geometry(&state.geom);
    draw.end();
}

fn draw_flower(
    geom: &mut Geometry,
    cx: f32,
    cy: f32,
    radius: f32,
    amplitude: f32,
    period: f32,
    modified_period_len: f32,
    initial_angle: f32,
    index: f32,
    amplitude_modifier: f32,
) {
    let segments = (2.0 * math::PI * radius).floor();
    let mut begin = false;

    for i in 0..segments as i32 {
        let period_segments: f32 = segments / period;
        let current_periods = i as f32 % period_segments;

        let radians_period = if current_periods < modified_period_len {
            current_periods / modified_period_len
        } else {
            0.0
        };

        let c_radius = radius
            + amplitude
                * (radians_period * (3.0 + index) * math::PI).sin()
                * ((radians_period * math::PI).sin() / 2.0 * amplitude_modifier);

        let radians = i as f32 / segments * 2.0 * math::PI + initial_angle;
        let x = cx + c_radius * radians.cos();
        let y = cy + c_radius * radians.sin();

        if !begin {
            geom.move_to(x, y);
            begin = true;
        }

        geom.line_to(x, y);
    }

    geom.close_path();
}
