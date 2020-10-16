use nae::prelude::{m2d::Transform2d, *};

struct State {
    tex: Texture,
    transforms: Vec<Transform2d>,
    font: Font,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .draw(draw)
        .update(update)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    State {
        transforms: vec![],
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        tex: Texture::from_bytes(app, include_bytes!("assets/ferris_chef.png")).unwrap(),
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
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.color = Color::WHITE;
    for (i, t) in state.transforms.iter_mut().enumerate() {
        draw.color = Color::WHITE;

        draw.push(t.matrix());
        draw.image(&state.tex, 0.0, 0.0);
        draw.pop();

        t.rotation += 0.2 * math::PI / 180.0;
    }

    //Debug info
    draw_debug(draw, state);

    draw.end();
}

fn draw_debug(draw: &mut Draw, state: &mut State) {
    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    draw.color = Color::BLACK;
    for t in state.transforms.iter() {
        draw.text(
            &state.font,
            &format!(
                "pos: ({}, {})\nanchor: ({}, {})\npivot: ({}, {})",
                t.x, t.y, t.anchor_x, t.anchor_y, t.pivot_x, t.pivot_y,
            ),
            t.x + t.width * (0.5 - t.anchor_x),
            t.y + t.height * (0.5 - t.anchor_y),
            24.0,
        );
    }
}
