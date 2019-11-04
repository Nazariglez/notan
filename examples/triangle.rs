use nae::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

//#[nae_start]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    nae::init({})
        .draw(on_draw)
        .build()
        .unwrap();
}

fn on_draw(app: &mut App, state: &mut ()) {
    let mut draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.set_color(Color::Green);
    draw.triangle(
        400.0, 100.0,
        100.0, 500.0,
        700.0, 500.0);
    draw.end();
}
