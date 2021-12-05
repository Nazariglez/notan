use notan::glyph::*;
use notan::prelude::*;

const TEXT: &str = include_str!("assets/lorem.txt");

#[derive(AppState)]
struct State {
    renderer: BasicPipeline,
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .set_config(GlyphConfig) // Adds the GlyphPlugin in a simple way
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics, plugins: &mut Plugins) -> State {
    let renderer = BasicPipeline::new(gfx).unwrap();
    let mut glyph = plugins.get_mut::<GlyphPlugin>().unwrap();
    let font = glyph
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();

    State { font, renderer }
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut glyphs = plugins.get_mut::<GlyphPlugin>().unwrap();

    // Process text
    glyphs.process_text(
        &state.font,
        &Text::new("Lorem Ipsum")
            .size(40.0)
            .color(Color::ORANGE)
            .h_align_center()
            .position(400.0, 80.0, 0.0),
    );

    glyphs.process_text(
        &state.font,
        &Text::new(TEXT)
            .h_align_center()
            .v_align_middle()
            .size(18.0)
            .max_width(700.0)
            .position(400.0, 300.0, 0.0),
    );

    // Update the font manager texture
    glyphs.update(gfx, &mut state.renderer).unwrap();

    let mut renderer = gfx.create_renderer();
    renderer.begin(Some(&ClearOptions::color(Color::new(0.7, 0.2, 0.3, 1.0))));

    // Pass to the GlyphRender the texture and the renderer to use
    state.renderer.render(&glyphs.texture, &mut renderer);

    renderer.end();

    gfx.render(&renderer);
}
