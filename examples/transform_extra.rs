use nae::extras::Transform2d;
use nae::prelude::*;

struct State {
    tex: Texture,
    transforms: Vec<Transform2d>,
}

#[nae::main]
fn main() {
    nae::with_state(init)
        .draw(draw)
        .update(update)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    State {
        tex: app.load_file("./examples/assets/ferris_chef.png").unwrap(),
        transforms: vec![],
    }
}

// Sets the size of the texture on the transform once is loaded
fn update(app: &mut App, state: &mut State) {
    if state.tex.is_loaded() && state.transforms.len() == 0 {
        state.transforms = init_transforms(&state.tex);
    }
}

fn init_transforms(img: &Texture) -> Vec<Transform2d> {
    let mut transforms: Vec<Transform2d> = (0..5)
        .map(|_| Transform2d::new(img.width(), img.height()))
        .collect();

    transforms[0]
        .set_pivot(1.0, 1.0)
        .set_anchor(0.0, 0.0)
        .set_position(0.0, 0.0);
    transforms[1]
        .set_pivot(0.0, 1.0)
        .set_anchor(1.0, 0.0)
        .set_position(800.0, 0.0);
    transforms[2]
        .set_pivot(0.5, 0.5)
        .set_anchor(0.5, 0.5)
        .set_position(400.0, 300.0);
    transforms[3]
        .set_pivot(1.0, 0.0)
        .set_anchor(0.0, 1.0)
        .set_position(0.0, 600.0);
    transforms[4]
        .set_pivot(0.0, 0.0)
        .set_anchor(1.0, 1.0)
        .set_position(800.0, 600.0);

    transforms
}

fn draw(app: &mut App, state: &mut State) {
    //    let img = &state.tex;

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    draw.set_color(Color::WHITE);
    for (i, t) in state.transforms.iter_mut().enumerate() {
        draw.set_color(Color::WHITE);

        draw.push_matrix(t.matrix());
        draw.image(&mut state.tex, 0.0, 0.0);
        draw.pop_matrix();

        t.rotation += 0.2 * math::PI / 180.0;
    }

    //Debug info
    draw.set_color(Color::BLACK);
    for (i, t) in state.transforms.iter_mut().enumerate() {
        draw.text_ext(
            &format!(
                "pos: ({}, {})\nanchor: ({}, {})\npivot: ({}, {})",
                t.x, t.y, t.anchor_x, t.anchor_y, t.pivot_x, t.pivot_y,
            ),
            t.x + t.width * (0.5 - t.anchor_x),
            t.y + t.height * (0.5 - t.anchor_y),
            24.0,
            HorizontalAlign::Center,
            VerticalAlign::Center,
            None,
        );
    }

    draw.end();
}
