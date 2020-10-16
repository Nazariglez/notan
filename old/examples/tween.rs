use nae::prelude::*;
use nae::tween::*;

const EASING: [Easing; 10] = [
    Easing::Linear,
    Easing::InQuad,
    Easing::InCubic,
    Easing::InExpo,
    Easing::OutBack,
    Easing::OutSine,
    Easing::OutQuart,
    Easing::InOutQuint,
    Easing::InOutElastic,
    Easing::InOutBounce,
];

struct State {
    font: Font,
    texture: Texture,
    tweens: Vec<Tween>,
}

fn init(app: &mut App) -> State {
    let tweens = EASING.iter().map(create_tween).collect::<Vec<_>>();

    State {
        font: Font::from_bytes(app, include_bytes!("assets/Ubuntu-B.ttf")).unwrap(),
        texture: Texture::from_bytes(app, include_bytes!("assets/bunny.png")).unwrap(),
        tweens,
    }
}

fn create_tween(easing: &Easing) -> Tween {
    let mut tween = Tween::new(200.0, 600.0, 8.0);
    tween.repeat_forever = true;
    tween.use_yoyo = true;
    tween.easing = *easing;
    tween.delay = 0.5;
    tween.start();
    tween
}

fn update(app: &mut App, state: &mut State) {
    state.tweens.iter_mut().for_each(|t| t.tick(app.delta));
}

fn draw(app: &mut App, state: &mut State) {
    let draw = app.draw();
    draw.begin(Color::ORANGE);

    draw.set_text_align(HorizontalAlign::Center, VerticalAlign::Center);
    state.tweens.iter().enumerate().for_each(|(i, t)| {
        let yy = (40 + (50 * i)) as _;
        let hh = state.texture.height() * 0.5;

        draw.text(&state.font, &t.easing.to_string(), 400.0, yy + hh, 20.0);

        draw.image(&state.texture, t.value, yy);
    });

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
