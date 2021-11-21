use notan::gly::{ab_glyph, GlyphBrushBuilder, Section, Text};
use notan::prelude::*;
use notan_gly::ab_glyph::FontArc;
use notan_gly::{DefaultGlyphPipeline, Glyph, GlyphBrush, GlyphExtension};

#[derive(AppState)]
struct State {
    // g: GlyphBrush,
    p: DefaultGlyphPipeline,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_graphic_ext(|gfx: &mut Graphics| {
            let f = ab_glyph::FontArc::try_from_slice(include_bytes!("./assets/Ubuntu-B.ttf")).unwrap();
            let mut ext = GlyphExtension::new(gfx);
            ext.add_font(f);
            ext
        })
        .draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    // Prepare glyph_brush
    // let g = GlyphBrushBuilder::using_font(f).build(gfx);
    let p = DefaultGlyphPipeline::new(gfx).unwrap();

    State {
        // g,
        p
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();

    let renderer = {
        let mut g = gfx.get_ext_mut::<Glyph, GlyphExtension>().unwrap();

        g.queue(&Section {
            screen_position: (30.0, 30.0),
            bounds: (width as f32, height as f32),
            text: vec![Text::default()
                .with_text("Hello glow_glyph!")
                .with_color([1.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        g.queue(Section {
            screen_position: (30.0, 90.0),
            bounds: (width as f32, height as f32),
            text: vec![Text::default()
                .with_text("Hello glow_glyph!")
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        g.gen_renderer(gfx, &mut state.p)
    };
    gfx.render(&renderer);
}
