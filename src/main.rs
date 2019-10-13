mod graphics;
mod window;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct App {
    window: window::Window,
    graphics: graphics::Context,
}

pub struct AppBuilder<S> {
    state: Option<S>,
}

impl<S> AppBuilder<S> {
    pub fn build(&self) -> Result<App, String> {
        let win = window::Window::new();
        let mut gfx = graphics::Context::new(win.window())?;

        gfx.begin(Some(graphics::color::rgba(0.1, 0.2, 0.3, 1.0)));
        gfx.end();

        Ok(App {
            window: win,
            graphics: gfx,
        })
    }
}

pub fn init<S>(state: S) -> AppBuilder<S> {
    AppBuilder { state: Some(state) }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    println!("Hello, world!");
    let app = init({}).build().unwrap();
}
