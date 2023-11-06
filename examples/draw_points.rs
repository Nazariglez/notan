use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    #[cfg(feature = "notan_text")]
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::default().set_size(380, 380);
    notan::init_with(setup)
        .add_config(DrawConfig)
        .add_config(win_config)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    #[cfg(feature = "notan_text")]
    {
        let font = gfx
            .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
            .unwrap();
        State { font }
    }
    #[cfg(not(feature = "notan_text"))]
    State {}
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let (xpad, ypad) = (20., 20.); // padding
    let (mut xal, mut yal); // aligment

    for xsquare in 0..=2 {
        xal = match xsquare {
            0 => XAlignment::Left,
            1 => XAlignment::Center,
            _ => XAlignment::Right,
        };
        let xpos = xpad + xpad * xsquare as f32 + xsquare as f32 * 100.;

        #[cfg(feature = "notan_text")]
        draw.text(&state.font, &format!["X:{xal:?}"])
            .position(xpos + 50., 5.0)
            .color(Color::WHITE)
            .size(14.0)
            .alpha(0.7)
            .h_align_center()
            .v_align_middle();

        for ysquare in 0..=2 {
            yal = match ysquare {
                0 => YAlignment::Top,
                1 => YAlignment::Center,
                _ => YAlignment::Bottom,
            };
            let ypos = ypad + ypad * ysquare as f32 + ysquare as f32 * 100.;

            // draw each gray rectangle background
            draw.rect((xpos, ypos), (100.0, 100.0))
                .fill_color(Color::new(0.3, 0.3, 0.3, 1.));

            #[cfg(feature = "notan_text")]
            if xsquare == 0 {
                draw.text(&state.font, &format!["Y:\n{yal:?}"])
                    .position(0.0, ypos + 50.)
                    .color(Color::WHITE)
                    .size(14.0)
                    .alpha(0.7)
                    .h_align_left()
                    .v_align_middle();
            }

            // draw the inner rectangle points
            for x in 0..=9 {
                for y in 0..=9 {
                    #[cfg(feature = "notan_text")]
                    if xsquare == 0 && x == 0 {
                        draw.text(&state.font, &format!["{}", 1 + y])
                            .position(370., ypos + y as f32 * 10.)
                            .color(Color::WHITE)
                            .size(12.0)
                            .h_align_center()
                            .v_align_middle();
                    }
                    let (x, y) = (x as f32, y as f32);

                    // point increasing in width vertically
                    draw.point(xpos + x * 10., ypos + y * 10.)
                        .alpha(0.5)
                        .width(1.0 + y)
                        .align(xal, yal)
                        .color(Color::new(0.9, 0.9, 0.9, 0.9));
                    // origin point in red
                    draw.point(xpos + x * 10., ypos + y * 10.)
                        .alpha(0.5)
                        .color(Color::RED);
                }
            }
        }
    }
    gfx.render(&draw);
}
