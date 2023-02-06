use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build()
}

fn draw(app: &mut App, gfx: &mut Graphics) {
    // "Random" color bases on the app's time
    let color = Color::from_rgb(
        app.timer.elapsed_f32().cos(),
        app.timer.elapsed_f32().sin(),
        1.0,
    );

    // create a renderer object
    let mut renderer = gfx.create_renderer();

    // begin a pass to clear the screen
    renderer.begin(Some(ClearOptions::color(color)));
    renderer.end();

    // render to screen
    gfx.render(&renderer);
}
