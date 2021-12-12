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

    tt.chain_text(" SUP!").size(50.0).color(Color::BLUE);

    gfx.render(&tt);
}
