use notan::egui::{self, *};
use notan::log::LogConfig;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    music: AudioSource,
    sound: Option<Sound>,
    repeat: bool,
    volume: f32,
}

fn play_music(app: &mut App, state: &mut State) {
    let sound = app.audio.play_sound(&state.music, state.repeat);
    state.sound = Some(sound);
}

fn stop_music(app: &mut App, state: &mut State) {
    if let Some(s) = state.sound.take() {
        app.audio.stop(&s);
    }
}

fn is_playing(app: &mut App, state: &State) -> bool {
    match &state.sound {
        Some(s) => !app.audio.is_stopped(s),
        None => false,
    }
}

fn is_paused(app: &mut App, state: &State) -> bool {
    match &state.sound {
        Some(s) => app.audio.is_paused(s),
        None => false,
    }
}

fn toggle_music(app: &mut App, state: &mut State) {
    match &state.sound {
        None => {}
        Some(s) => {
            if app.audio.is_paused(s) {
                app.audio.resume(s);
            } else {
                app.audio.pause(s);
            }
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(LogConfig::debug())
        .add_config(EguiConfig)
        .add_config(WindowConfig::default().size(300, 300))
        .draw(draw)
        .build()
}

fn setup(app: &mut App) -> State {
    let music = app
        .audio
        .create_source(include_bytes!("assets/jingles_NES00.ogg"))
        .unwrap();

    State {
        music,
        sound: None,
        repeat: false,
        volume: 1.0,
    }
}

// UI
fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    app.audio.set_global_volume(state.volume);

    let mut output = plugins.egui(|ctx| {
        egui::Window::new("Controls").show(ctx, |ui| {
            ui.label("Volume");
            ui.add(egui::Slider::new(&mut state.volume, 0.0..=1.0));

            ui.add(egui::Checkbox::new(&mut state.repeat, "Repeat"));

            let is_playing = is_playing(app, state);
            if is_playing {
                let btn = ui.button("Stop");
                if btn.clicked() {
                    stop_music(app, state);
                }

                let pause = if is_paused(app, state) {
                    "Resume"
                } else {
                    "Pause"
                };
                let btn = ui.button(pause);
                if btn.clicked() {
                    toggle_music(app, state);
                }
            } else {
                let play = ui.button("Play");
                if play.clicked() {
                    play_music(app, state);
                }
            }
        });
    });

    output.clear_color(Color::GRAY);
    gfx.render(&output);
}
