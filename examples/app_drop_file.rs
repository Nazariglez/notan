use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    font: Font,
    dragging: usize,
    dropped: Vec<String>,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .draw(draw)
        .event(event)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    State {
        font,
        dragging: 0,
        dropped: vec![],
    }
}

fn event(state: &mut State, evt: Event) {
    match evt {
        Event::DragEnter(_path) => {
            state.dragging += 1;
        }
        Event::DragLeft => {
            state.dragging = 0;
        }
        Event::Drop(path) => {
            state.dragging = 0;
            state.dropped.push(path.to_string_lossy().to_string());
        }
        _ => {}
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    if state.dropped.len() > 0 {
        let mut y = 20.0;
        state.dropped.iter().rev().take(20).for_each(|path| {
            draw.text(&state.font, &format!("Dropped: {}", path))
                .color(Color::ORANGE)
                .alpha(0.6)
                .h_align_left()
                .v_align_top()
                .position(20.0, y)
                .size(16.0);

            y += 20.0;
        });
    }

    if state.dragging == 0 {
        draw.text(&state.font, "Drop files here")
            .size(30.0)
            .v_align_middle()
            .h_align_center()
            .position(400.0, 300.0);
    } else {
        draw.rect((10.0, 10.0), (780.0, 580.0))
            .color(Color::WHITE)
            .stroke(6.0);

        let text = format!("You're dragging {} files", state.dragging);
        draw.text(&state.font, &text)
            .size(30.0)
            .color(Color::GRAY)
            .v_align_middle()
            .h_align_center()
            .position(400.0, 300.0);
    }

    gfx.render(&draw);
}
