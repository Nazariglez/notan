use glam::Mat4;
use notan::glyph::prelude::*;
use notan::prelude::*;

const TEXT: &'static str = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse malesuada nisl non mi pharetra, a euismod mi volutpat. Pellentesque dictum turpis id lorem ornare, quis commodo ipsum tempus. Ut a nulla sed leo ullamcorper dignissim. Nullam in dolor nunc. Phasellus vitae neque malesuada, ultrices elit vel, dapibus turpis. Aenean sodales nulla ac mauris rutrum, vel fringilla metus tincidunt. Proin ultricies ultricies posuere. Sed cursus, mauris at maximus mollis, enim nisl sodales est, sed porta justo nisi quis tortor. Aenean ornare, sem dignissim scelerisque posuere, ligula quam eleifend diam, sit amet suscipit nibh lacus at purus. Nunc vel rhoncus purus, in accumsan magna. Proin diam sem, dapibus et felis nec, vestibulum varius turpis. Donec condimentum justo nec ipsum laoreet, ac consectetur sapien luctus.

Sed sit amet elit placerat, efficitur ligula sit amet, sagittis erat. Nam dapibus risus et quam fringilla rutrum. Nullam malesuada pulvinar arcu, quis iaculis enim ultricies non. Proin vel eleifend eros. Nam iaculis lacus arcu, id malesuada dui gravida eu. Cras interdum efficitur mauris, vel suscipit ipsum iaculis et. Aenean vel elementum nunc. Maecenas erat urna, rhoncus dignissim egestas facilisis, tincidunt sed ipsum. Ut pulvinar nisl a rutrum tincidunt.
"#;

#[derive(notan::AppState)]
struct State {
    manager: FontManager<DefaultFontRenderer>,
    font: Font,
}

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(setup).update(update).draw(draw).build()
}

fn setup(app: &mut App, gfx: &mut Graphics) -> State {
    let mut manager = FontManager::new(gfx).unwrap();
    let font = manager
        .load_font(include_bytes!("./assets/Ubuntu-B.ttf"))
        .unwrap();
    State { manager, font }
}

fn update(state: &mut State) {
    state.manager.add_text(
        &state.font,
        &Text::new("Lorem Ipsum")
            .size(40.0)
            .color(Color::ORANGE)
            .h_align_center()
            .position(400.0, 80.0, 0.0),
    );

    state.manager.add_text(
        &state.font,
        &Text::new(TEXT)
            .h_align_center()
            .v_align_middle()
            .size(18.0)
            .max_width(700.0)
            .position(400.0, 300.0, 0.0),
    );
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    // Update the font manager texture
    state.manager.update(gfx).unwrap();

    let mut renderer = gfx.create_renderer();
    renderer.begin(Some(&ClearOptions::new(Color::new(0.7, 0.2, 0.3, 1.0))));

    // Pass the renderer to the manager to call draw using the pipeline and buffers from the default font render
    state.manager.render(&mut renderer);

    renderer.end();

    gfx.render(&renderer);
}
