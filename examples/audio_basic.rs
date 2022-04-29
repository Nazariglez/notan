use notan::egui::{self, *};
use notan::log::LogConfig;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    music: [AudioSource; 2],
    sound: [Option<Sound>; 2],
    repeat: [bool; 2],
    volume: [f32; 2],
}

fn play_music(index: usize, app: &mut App, state: &mut State) {
    let sound = app
        .audio
        .play_sound(&state.music[index], state.repeat[index]);
    state.sound[index] = Some(sound);
}

fn stop_music(index: usize, app: &mut App, state: &mut State) {
    if let Some(s) = state.sound[index].take() {
        app.audio.stop(&s);
    }
}

fn is_playing(index: usize, app: &mut App, state: &State) -> bool {
    match &state.sound[index] {
        Some(s) => !app.audio.is_stopped(s),
        None => false,
    }
}

fn is_paused(index: usize, app: &mut App, state: &State) -> bool {
    match &state.sound[index] {
        Some(s) => app.audio.is_paused(s),
        None => false,
    }
}

fn toggle_music(index: usize, app: &mut App, state: &mut State) {
    match &state.sound[index] {
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

fn set_volume(index: usize, app: &mut App, state: &mut State) {
    match &state.sound[index] {
        None => {}
        Some(s) => app.audio.set_volume(s, state.volume[index]),
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

    let music1 = app
        .audio
        .create_source(include_bytes!("assets/jingles_PIZZI01.ogg"))
        .unwrap();

    State {
        music: [music, music1],
        sound: [None, None],
        repeat: [false, false],
        volume: [1.0, 1.0],
    }
}

// -- UI
fn draw_controls(index: usize, name: &str, ctx: &Context, app: &mut App, state: &mut State) {
    egui::Window::new(name).show(ctx, |ui| {
        ui.label("Volume");
        ui.add(egui::Slider::new(&mut state.volume[index], 0.0..=1.0));

        ui.add(egui::Checkbox::new(&mut state.repeat[index], "Repeat"));

        let is_playing = is_playing(index, app, state);
        if is_playing {
            let btn = ui.button("Stop");
            if btn.clicked() {
                stop_music(index, app, state);
            }

            let pause = if is_paused(index, app, state) {
                "Resume"
            } else {
                "Pause"
            };
            let btn = ui.button(pause);
            if btn.clicked() {
                toggle_music(index, app, state);
            }
        } else {
            let play = ui.button("Play");
            if play.clicked() {
                play_music(index, app, state);
            }
        }
    });
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let len = state.music.len();
    let mut output = plugins.egui(|ctx| {
        (0..len).for_each(|i| draw_controls(i, &format!("Music {}", i), ctx, app, state));
    });

    output.clear_color(Color::GRAY);
    gfx.render(&output);

    // set volume
    (0..len).for_each(|i| set_volume(i, app, state));
}
