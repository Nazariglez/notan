/*
   This example shows how to use GlyphBrush directly without the GlyphExtension
*/

use notan::gly::{ab_glyph, GlyphBrushBuilder, Section, Text};
use notan::prelude::*;
use notan_gly::ab_glyph::FontArc;
use notan_gly::{DefaultGlyphPipeline, Glyph, GlyphBrush, GlyphExtension};

#[derive(AppState)]
struct State {
    glyph_brush: GlyphBrush,
    pipeline: DefaultGlyphPipeline,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    // Load the font that we'll use to draw the text
    let font = ab_glyph::FontArc::try_from_slice(include_bytes!("./assets/Ubuntu-B.ttf")).unwrap();

    // Create GlyphBrush using the loaded font
    let glyph_brush = GlyphBrushBuilder::using_font(font).build(gfx);

    // This is the default pipeline included to draw the glyphs using instancing, you can use your custom pipeline
    let pipeline = DefaultGlyphPipeline::new(gfx).unwrap();

    State {
        glyph_brush,
        pipeline,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();

    // Queue sections to draw
    state.glyph_brush.queue(&Section {
        screen_position: (30.0, 30.0),
        bounds: (width as f32, height as f32),
        text: vec![Text::default()
            .with_text("Hello glow_glyph!")
            .with_color([1.0, 0.0, 0.0, 1.0])
            .with_scale(40.0)],
        ..Section::default()
    });

    state.glyph_brush.queue(Section {
        screen_position: (30.0, 90.0),
        bounds: (width as f32, height as f32),
        text: vec![Text::default()
            .with_text("Hello glow_glyph!")
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(40.0)],
        ..Section::default()
    });

    // Process the sections queued and create a renderer
    let renderer = state
        .glyph_brush
        .create_renderer_from_queue(gfx, &mut state.pipeline);

    // Draw the renderer to the screen
    gfx.render(&renderer);
}
