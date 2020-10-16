use nae::prelude::*;
use nae::tween::*;

struct State {
    font: Font,
    texture: Texture,
    tween_manager: TweenManager<String>,
    rotation: f32,
    x: f32,
    y: f32,
}

fn init(app: &mut App) -> State {
    let mut rotation_tween = Tween::new(0.0, 360.0 * (std::f32::consts::PI / 180.0), 3.0);
    rotation_tween.repeat_times = 10;
    rotation_tween.start();

    let mut y_tween = Tween::new(400.0, 200.0, 2.0);
    y_tween.repeat_times = 5;
    y_tween.use_yoyo = true;
    y_tween.start();

    let mut x_tween = Tween::new(200.0, 600.0, 5.0);
    x_tween.repeat_times = 3;
    x_tween.use_yoyo = true;
    x_tween.delay = 2.0;
    x_tween.start();

    let mut manager = TweenManager::new();
    manager.insert("rotation".to_string(), rotation_tween);
    manager.insert("y_axis".to_string(), y_tween);
    manager.insert("x_axis".to_string(), x_tween);

    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        texture: Texture::from_bytes(app, include_bytes!("assets/bunny.png")).unwrap(),
        tween_manager: manager,
        rotation: 0.0,
        x: 200.0,
        y: 400.0,
    }
}

fn update(app: &mut App, state: &mut State) {
    /// Clean before we start the frame to remove the tweens that did finish on the last frame
    state.tween_manager.clean();

    /// Push the delta time to the tweens
    state.tween_manager.tick(app.delta);
}

fn draw(app: &mut App, state: &mut State) {
    let ww = state.texture.width();
    let hh = state.texture.height();
    let xx = state.x;
    let yy = state.y;
    let angle = state.rotation;

    let draw = app.draw();
    draw.begin(Color::ORANGE);

    let (rotation_text, rot_color) = match state.tween_manager.get("rotation") {
        Some(t) => {
            state.rotation = t.value;
            ("Rotation running...", Color::OLIVE)
        }
        _ => ("Rotation ended...", Color::RED),
    };

    let (x_text, x_color) = match state.tween_manager.get("x_axis") {
        Some(t) => {
            state.x = t.value;
            ("X movement running...", Color::OLIVE)
        }
        _ => ("X movement ended...", Color::RED),
    };

    let (y_text, y_color) = match state.tween_manager.get("y_axis") {
        Some(t) => {
            state.y = t.value;
            ("Y movement running...", Color::OLIVE)
        }
        _ => ("Y movement ended...", Color::RED),
    };

    draw.color = rot_color;
    draw.text(&state.font, rotation_text, 20.0, 20.0, 20.0);
    draw.color = x_color;
    draw.text(&state.font, x_text, 20.0, 60.0, 20.0);
    draw.color = y_color;
    draw.text(&state.font, y_text, 20.0, 100.0, 20.0);

    // Draw the image using the tween's data as matrix
    draw.color = Color::WHITE;
    draw.push_translation(xx, yy);
    draw.push_rotation(angle);
    draw.push_translation(-ww * 0.5, -hh * 0.5);
    draw.image(&state.texture, 0.0, 0.0);
    draw.pop();
    draw.pop();
    draw.pop();

    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}
