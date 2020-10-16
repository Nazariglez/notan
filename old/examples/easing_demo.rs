use nae::prelude::*;
use nae::tween::*;

// List of ease functions to display
const EASING: [Easing; 31] = [
    Easing::Linear,
    Easing::InQuad,
    Easing::OutQuad,
    Easing::InOutQuad,
    Easing::InCubic,
    Easing::OutCubic,
    Easing::InOutCubic,
    Easing::InQuart,
    Easing::OutQuart,
    Easing::InOutQuart,
    Easing::InQuint,
    Easing::OutQuint,
    Easing::InOutQuint,
    Easing::InSine,
    Easing::OutSine,
    Easing::InOutSine,
    Easing::InExpo,
    Easing::OutExpo,
    Easing::InOutExpo,
    Easing::InCirc,
    Easing::OutCirc,
    Easing::InOutCirc,
    Easing::InElastic,
    Easing::OutElastic,
    Easing::InOutElastic,
    Easing::InBack,
    Easing::OutBack,
    Easing::InOutBack,
    Easing::InBounce,
    Easing::OutBounce,
    Easing::InOutBounce,
];

struct State {
    x_tween: Tween,
    y_tween: Tween,
    geom: Geometry,
    index: usize,
    font: Font,
}

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    let mut x_tween = Tween::new(20.0, app.width() - 20.0, 5.0);
    let mut y_tween = Tween::new(app.height() - 20.0, 40.0, 5.0);

    x_tween.repeat_forever = true;
    y_tween.repeat_forever = true;

    x_tween.start();
    y_tween.start();

    let mut geom = Geometry::new();
    geom.move_to(20.0, app.height() - 20.0);

    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        x_tween,
        y_tween,
        geom,
        index: 0,
    }
}

fn update(app: &mut App, state: &mut State) {
    check_keyboard_input(app, state);

    let already_repeated = state.x_tween.already_repeated();

    //add the delta time
    state.x_tween.tick(app.delta);
    state.y_tween.tick(app.delta);

    // prepare the geometry to be draw
    state.geom.line_to(state.x_tween.value, state.y_tween.value);
    state.geom.stroke(Color::MAGENTA, 2.0);

    if state.x_tween.already_repeated() != already_repeated {
        state.geom.clear();
        state.geom.move_to(state.x_tween.from, state.y_tween.from);
    } else {
        state.geom.move_to(state.x_tween.value, state.y_tween.value);
    }
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::BLACK);
    draw_ui(draw, state);

    // start and end point
    draw.color = Color::GREEN;
    draw.circle(state.x_tween.from, state.y_tween.from, 5.0);
    draw.circle(state.x_tween.to, state.y_tween.to, 5.0);

    // line path
    draw.geometry(&state.geom);

    // head of the line
    draw.color = Color::RED;
    draw.circle(state.x_tween.value, state.y_tween.value, 5.0);

    draw.end();
}

fn draw_ui(draw: &mut Draw, state: &mut State) {
    draw.color = Color::WHITE;
    draw.text_horizontal_align = HorizontalAlign::Left;
    draw.text(
        &state.font,
        "Use left and right arrows to switch the easing function.",
        20.0,
        10.0,
        16.0,
    );
    draw.text_horizontal_align = HorizontalAlign::Right;
    draw.text(
        &state.font,
        &state.x_tween.easing.to_string(),
        800.0 - 20.0,
        10.0,
        18.0,
    );

    draw.alpha = 0.8;
    draw.line(20.0, 40.0, 20.0, 600.0 - 10.0, 4.0);
    draw.line(10.0, 600.0 - 20.0, 800.0 - 20.0, 600.0 - 20.0, 4.0);
    draw.alpha = 1.0;
}

fn apply_easing(state: &mut State) {
    let easing = EASING[state.index];
    // Some easing function will exceeded the max or the minimum
    // we can move the start and end point here
    let (x1, x2) = match easing {
        Easing::InElastic => (190.0, 800.0 - 20.0),
        Easing::OutElastic => (20.0, 800.0 - 190.0),
        Easing::InOutElastic => (110.0, 800.0 - 110.0),
        Easing::InBack => (95.0, 800.0 - 20.0),
        Easing::OutBack => (20.0, 800.0 - 95.0),
        Easing::InOutBack => (90.0, 800.0 - 90.0),
        _ => (20.0, 800.0 - 20.0),
    };

    state.x_tween.from = x1;
    state.x_tween.to = x2;
    state.x_tween.easing = easing;
    state.x_tween.reset();
    state.y_tween.reset();
    state.geom.clear();
    state.geom.move_to(x1, 600.0 - 20.0);
}

// if the user use the arrow keys change the easing function
fn check_keyboard_input(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Left) {
        state.index = if state.index > 0 {
            state.index - 1
        } else {
            EASING.len() - 1
        };

        apply_easing(state);
    }

    if app.keyboard.was_pressed(KeyCode::Right) {
        state.index = if state.index < EASING.len() - 1 {
            state.index + 1
        } else {
            0
        };

        apply_easing(state);
    }
}
