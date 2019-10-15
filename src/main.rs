mod glm;
mod graphics;
mod window;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::graphics::color::Color;

pub struct App {
    window: window::Window,
    graphics: graphics::Context,
}

pub struct AppBuilder<S> {
    state: Option<S>,
    draw_callback: Option<fn(&mut App)>,
}

impl<S> AppBuilder<S> {
    pub fn build(&self) -> Result<App, String> {
        let win = window::Window::new();
        let mut gfx = graphics::Context::new(win.window())?;

        let mut app = App {
            window: win,
            graphics: gfx,
        };

        if let Some(cb) = self.draw_callback {
            cb(&mut app);
        }

        //        Err("".to_string())
        Ok(app)
    }

    pub fn draw(&mut self, cb: fn(&mut App)) -> &Self {
        self.draw_callback = Some(cb);
        self
    }
}

pub fn init<S>(state: S) -> AppBuilder<S> {
    AppBuilder {
        state: Some(state),
        draw_callback: None,
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn my_draw_cb(app: &mut App) {
    let mut gfx = &mut app.graphics;
    gfx.begin(Some(graphics::color::rgba(0.1, 0.2, 0.3, 1.0)));
    gfx.set_color(Color::Red);
    gfx.push_transform(glm::scaling2d(&glm::vec2(2.0, 2.0)));
    gfx.fill_rect(0.0, 0.0, 100.0, 100.0);
    gfx.pop_transform();

    gfx.set_color(Color::Green);
    gfx.push_transform(glm::translation2d(&glm::vec2(-000.0, 0.0)));
    gfx.push_transform(glm::scaling2d(&glm::vec2(2.0, 2.0)));
        gfx.fill_triangle(200.0, 200.0, 300.0, 300.0, 100.0, 300.0);
//    gfx.fill_triangle(0.0, 0.0, 0.0, 0.5, 0.7, 0.0);
    gfx.pop_transform();

    let len = 50;
    for i in (0..len) {
        let n = i as f32;
        let r = (1.0/len as f32) * n;
        let g = 0.5;
        let b = 1.0 - (1.0/len as f32) * n;
        let a = 1.0;
        gfx.set_color(graphics::color::rgba(r, b, g, a));
        gfx.fill_rect(10.0 * n, 10.0 * n, (100.0/len as f32) * n, (100.0/len as f32) * n);
    }
    gfx.end();
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    println!("Hello, world!");
    let app = init({}).draw(my_draw_cb).build().unwrap();
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}
