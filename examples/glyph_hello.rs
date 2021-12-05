use notan::gly::{ab_glyph, GlyphBrushBuilder, Section, Text};
use notan::prelude::*;
use notan_gly::ab_glyph::FontArc;
use notan_gly::{DefaultGlyphPipeline, GlyConfig, GlyphBrush, GlyphExtension, Glyphs};

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
    let mut ext = gfx.extension_mut::<Glyphs, GlyphExtension>().unwrap();
    ext.create_font(include_bytes!("./assets/Ubuntu-B.ttf"));

    State {}
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let (width, height) = gfx.size();

    let mut glyphs = Glyphs::default();

    // Queue sections to draw
    glyphs.queue(
        Section::new()
            .with_screen_position((30.0, 30.0))
            .with_bounds((width as _, height as _))
            .add_text(
                Text::default()
                    .with_text("Hello notan_glyph!")
                    .with_color([1.0, 0.0, 0.0, 1.0])
                    .with_scale(40.0),
            ),
    );

    glyphs.queue(
        Section::new()
            .with_screen_position((30.0, 90.0))
            .with_bounds((width as _, height as _))
            .add_text(
                Text::default()
                    .with_text("Hello notan_glyph!")
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(40.0),
            ),
    );

    gfx.render(&glyphs);
}
