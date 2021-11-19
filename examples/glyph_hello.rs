use notan::gly::{ab_glyph, GlyphBrushBuilder, Section, Text};
use notan::prelude::*;
use notan_gly::ab_glyph::FontArc;
use notan_gly::{BasicGlyphPipeline, Glyph, GlyphBrush};

#[derive(AppState)]
struct State {
    g: GlyphBrush,
    p: BasicGlyphPipeline,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    // Prepare glyph_brush
    let f = ab_glyph::FontArc::try_from_slice(include_bytes!("./assets/Ubuntu-B.ttf")).unwrap();
    let g = GlyphBrushBuilder::using_font(f).build(gfx);
    let p = BasicGlyphPipeline::new(gfx).unwrap();

    State { g, p }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();

    // let mut glyphs = Glyph::default();
    // glyphs.queue(Section {
    //     screen_position: (30.0, 30.0),
    //     bounds: (width as f32, height as f32),
    //     text: vec![Text::default()
    //         .with_text("Hello glow_glyph!")
    //         .with_color([1.0, 0.0, 0.0, 1.0])
    //         .with_scale(40.0)],
    //     ..Section::default()
    // });
    //
    // glyphs.queue(Section {
    //     screen_position: (30.0, 90.0),
    //     bounds: (width as f32, height as f32),
    //     text: vec![Text::default()
    //         .with_text("Hello glow_glyph!")
    //         .with_color([1.0, 1.0, 1.0, 1.0])
    //         .with_scale(40.0)],
    //     ..Section::default()
    // });
    //
    // state.g.process(&glyphs);

    // let mut renderer = gfx.create_renderer();
    state.g.queue(&Section {
        screen_position: (30.0, 30.0),
        bounds: (width as f32, height as f32),
        text: vec![Text::default()
            .with_text("Hello glow_glyph!")
            .with_color([1.0, 0.0, 0.0, 1.0])
            .with_scale(40.0)],
        ..Section::default()
    });

    state.g.queue(Section {
        screen_position: (30.0, 90.0),
        bounds: (width as f32, height as f32),
        text: vec![Text::default()
            .with_text("Hello glow_glyph!")
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(40.0)],
        ..Section::default()
    });

    // Draw the text!
    // state
    //     .g
    //     .draw_queued(gfx, &mut state.p, width as _, height as _);

    let renderer = state.g.renderer_queue(gfx, &mut state.p);
    gfx.render(&renderer);
}
