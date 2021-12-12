/*
   This example shows how to use GlyphBrush directly without the GlyphExtension
*/

use notan::prelude::*;
use notan::text::*;

#[derive(AppState)]
struct State {
    s: String,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(|gfx: &mut Graphics| {
        gfx.extension_mut::<TT, TextExtension>()
            .unwrap()
            .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
            .unwrap();

        State {
            s: "hello".to_string(),
        }
    })
    .set_config(TextConfig)
    .draw(draw)
    .build()
}

fn use_tt(tt: &TT) {
    println!("TT!");
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (w, h) = gfx.size();
    let mut tt = TT::new(w as _, h as _);
    tt.add_text(&state.s)
        .position(30.0, 30.0)
        .color(Color::RED)
        .size(30.0);
    // .position(30.0, 30.0);

    // tt.chain_text("text")
    //
    // tt.position(30.0, 30.0);
    //
    // tt.add_text("Hello text!");
    // tt.add_text("More about hello world?");
    // // tt.position(60.0, 90.0);
    // tt.add_text("another text...");
    //
    gfx.render(&tt);
    // use_tt(&tt);

    // // Queue sections to draw
    // state.glyph_brush.queue(
    //     Section::new()
    //         .with_screen_position((30.0, 30.0))
    //         .with_bounds((width as _, height as _))
    //         .add_text(
    //             Text::default()
    //                 .with_text("Hello notan_glyph!")
    //                 .with_color([1.0, 0.0, 0.0, 1.0])
    //                 .with_scale(40.0),
    //         ),
    // );
    //
    // state.glyph_brush.queue(
    //     Section::new()
    //         .with_screen_position((30.0, 90.0))
    //         .with_bounds((width as _, height as _))
    //         .add_text(
    //             Text::default()
    //                 .with_text("Hello notan_glyph!")
    //                 .with_color([1.0, 1.0, 1.0, 1.0])
    //                 .with_scale(40.0),
    //         ),
    // );
    //
    // // process the queue and return a renderer to draw
    // let renderer = state
    //     .glyph_brush
    //     .create_renderer(&mut state.pipeline)
    //     .clear(ClearOptions::color(Color::BLACK))
    //     .process(gfx);
    //
    // // Draw the renderer to the screen
    // gfx.render(&renderer);
}
