/*
   This example shows how to use GlyphBrush directly without the GlyphExtension
*/

use notan::gly::{ab_glyph, GlyphBrushBuilder, Section, Text};
use notan::prelude::*;
use notan_gly::ab_glyph::FontArc;
use notan_gly::{DefaultGlyphPipeline, GlyConfig, Glyph, GlyphBrush, GlyphExtension};

#[derive(AppState)]
struct State {}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(GlyConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let mut ext = gfx.extension_mut::<Glyph, GlyphExtension>().unwrap();
    ext.create_font(include_bytes!("./assets/Ubuntu-B.ttf"));

    State {}
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();

    let mut glyph = Glyph::default();
    // Queue sections to draw
    glyph.queue(Section {
        screen_position: (30.0, 30.0),
        bounds: (width as f32, height as f32),
        text: vec![Text::default()
            .with_text("Hello glow_glyph!")
            .with_color([1.0, 0.0, 0.0, 1.0])
            .with_scale(40.0)],
        ..Section::default()
    });

    glyph.queue(Section {
        screen_position: (30.0, 90.0),
        bounds: (width as f32, height as f32),
        text: vec![Text::default()
            .with_text("Hello glow_glyph!")
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(40.0)],
        ..Section::default()
    });
    //
    // // Process the sections queued and create a renderer
    // let renderer = state
    //     .glyph_brush
    //     .create_renderer_from_queue(gfx, &mut state.pipeline);

    // Draw the renderer to the screen
    gfx.render(&glyph);
}
