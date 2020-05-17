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

// store the animation objects
struct GolemAnim {
    up: Animation,
    down: Animation,
    left: Animation,
    right: Animation,
}

// Create the animations using from the texture passing the columns, rows, and frames
fn create_animations(texture: &Texture) -> GolemAnim {
    let frame_time = 0.1;
    let cols = 7;
    let rows = 4;

    let up = Animation::from_grid(
        texture,
        frame_time,
        cols,
        rows,
        Some(7), //7 first targets
        None,
    );

    let down = Animation::from_grid(
        texture,
        frame_time,
        cols,
        rows,
        None,
        Some(vec![14, 15, 16, 17, 18, 19, 20]), // frames from 14 to 20
    );

    let left = Animation::from_grid(
        texture,
        frame_time,
        cols,
        rows,
        None,
        Some(vec![7, 8, 9, 10, 11, 12, 13]), // frames from 7 to 13
    );

    let right = Animation::from_grid(
        texture,
        frame_time,
        cols,
        rows,
        None,
        Some(vec![21, 22, 23, 24, 25, 26, 27]), // frames from 21 to 27
    );

    GolemAnim {
        up,
        down,
        left,
        right,
    }
}

fn update(app: &mut App, state: &mut State) {
    // If some arrow was pressed reset the animation time and change the dir
    if app.keyboard.was_pressed(KeyCode::Up) && state.dir != "up" {
        state.dir = "up".to_string();
        state.golem.up.reset();
    } else if app.keyboard.was_pressed(KeyCode::Down) && state.dir != "down" {
        state.dir = "down".to_string();
        state.golem.down.reset();
    } else if app.keyboard.was_pressed(KeyCode::Left) && state.dir != "left" {
        state.dir = "left".to_string();
        state.golem.left.reset();
    } else if app.keyboard.was_pressed(KeyCode::Right) && state.dir != "right" {
        state.dir = "right".to_string();
        state.golem.right.reset();
    }

    // Gets the animation bases on the direction
    let anim = match state.dir.as_ref() {
        "down" => &mut state.golem.down,
        "left" => &mut state.golem.left,
        "right" => &mut state.golem.right,
        _ => &mut state.golem.up,
    };

    // add the frame time to calculate which frame we need to draw
    anim.tick(app.delta);
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.push_scale(3.0, 3.0);

    // get the animation based on the direction
    let anim = match state.dir.as_ref() {
        "down" => &mut state.golem.down,
        "left" => &mut state.golem.left,
        "right" => &mut state.golem.right,
        _ => &mut state.golem.up,
    };

    // draw the texture returned by the animation
    if let Some(tex) = anim.texture() {
        draw.image(tex, 100.0, 50.0);
    }

    draw.pop();

    // help text
    draw.text(
        &state.font,
        "Use arrows to change the animation.",
        10.0,
        10.0,
        20.0,
    );
    draw.end();
}

struct State {
    golem: GolemAnim,
    dir: String,
    font: Font,
}

impl State {
    fn new(app: &mut App) -> Self {
        let golem = Texture::from_bytes(app, include_bytes!("./assets/golem-walk.png")).unwrap();
        let font = Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap();

        Self {
            font,
            golem: create_animations(&golem),
            dir: "down".to_string(),
        }
    }
}
