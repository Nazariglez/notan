use notan::prelude::*;

// todo webaudio https://developer.chrome.com/blog/web-audio-autoplay/#policy-adjustments

#[derive(AppState)]
struct State {
    source: AudioSource,
    sound: Option<Sound>,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(|app: &mut App| {
        let source = app
            .audio
            .create_source(include_bytes!("assets/bipbip.ogg"))
            .unwrap();

        State {
            source,
            sound: None,
        }
    })
    .add_config(notan::log::LogConfig::default())
    .draw(draw)
    .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Space) {
        let sound = app.audio.play_sound(&state.source, false);
        state.sound = Some(sound);
    } else if app.keyboard.was_pressed(KeyCode::Z) {
        if let Some(id) = &state.sound {
            app.audio.stop(id);
        }
    }

    if let Some(id) = &state.sound {
        let volume = app.audio.volume(id);
        app.audio.set_volume(id, volume - app.timer.delta_f32());
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
