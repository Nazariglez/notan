/*
   This example shows how to use GlyphBrush directly without the GlyphExtension
*/

use notan::prelude::*;
use notan::text::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().set_config(TextConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let (w, h) = gfx.size();
    let mut tt = TT::new(w as _, h as _);
    tt.position(30.0, 30.0);
    tt.add_text("test");

    gfx.render(&tt);

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
