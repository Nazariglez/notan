use notan::prelude::*;

// todo webaudio https://developer.chrome.com/blog/web-audio-autoplay/#policy-adjustments

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(|app: &mut App| {
        app.audio
            .create_audio(include_bytes!("assets/assets_music.ogg"));
    })
    .draw(draw)
    .build()
}

fn draw(app: &mut App, gfx: &mut Graphics) {
    if app.keyboard.was_pressed(KeyCode::Space) {
        app.audio.play(0);
    } else if app.keyboard.was_pressed(KeyCode::Z) {
        app.audio.stop(0);
    }

    // "Random" color bases on the app's time
    let color = Color::from_rgb(
        app.timer.time_since_init().cos(),
        app.timer.time_since_init().sin(),
        1.0,
    );

    // create a renderer object
    let mut renderer = gfx.create_renderer();

    // begin a pass to clear the screen
    renderer.begin(Some(&ClearOptions::color(color)));
    renderer.end();

    // render to screen
    gfx.render(&renderer);
}
