use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    x: f32,
    y: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(DrawConfig)
        .event(event)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State {
        font,
        x: 400.0,
        y: 300.0,
    }
}

fn event(state: &mut State, evt: Event) {
    match evt {
        Event::MouseWheel { delta_x, delta_y } => {
            state.x = (state.x + delta_x).max(0.0).min(800.0);
            state.y = (state.y + delta_y).max(0.0).min(600.0);
        }
        _ => {}
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.text(&state.font, "Scroll with your mouse's wheel or touchpad")
        .position(400.0, 300.0)
        .size(40.0)
        .h_align_center()
        .v_align_middle();

    draw.circle(30.0)
        .position(state.x, state.y)
        .color(Color::RED);

    gfx.render(&draw);
}
