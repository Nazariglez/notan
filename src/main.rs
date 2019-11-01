mod glm;
mod graphics;
mod math;
mod window;

use crate::graphics::color::{rgba, Color};
use crate::graphics::{Asset, Texture, Vertex};
use crate::math::Geometry;
use std::rc::Rc;
use wasm_bindgen::__rt::core::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct App {
    window: window::Window,
    graphics: graphics::Context,
}

impl App {
    pub fn load_texture(&mut self, file: &str) -> Texture {
        unimplemented!()
    }
}

pub struct AppBuilder<S>
where
    S: 'static,
{
    state: Option<S>,
    draw_callback: Option<fn(&mut App, &mut S)>,
    update_callback: Option<fn(&mut App, &mut S)>,
}

impl<S> AppBuilder<S> {
    pub fn build(&mut self) -> Result<(), String> {
        let win = window::Window::new();
        let gfx = graphics::Context::new(win.window())?;

        let mut app = App {
            window: win,
            graphics: gfx,
        };

        let mut state = self.state.take().unwrap();
        let mut draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});
        let mut update_cb = self.update_callback.take().unwrap_or(|_, _| {});

        //        let rc_app = Rc::new(RefCell::new(app));

        window::run(move || {
            update_cb(&mut app, &mut state);
            draw_cb(&mut app, &mut state);
        });
        //cb(&mut app);

        //        Err("".to_string())
        Ok(())
    }

    pub fn draw(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.draw_callback = Some(cb);
        self
    }

    pub fn resource(&mut self, cb: fn(&mut App, &mut S, res: &str)) -> &mut Self {
        //TODO call this every time a new resource is loaded
        self
    }

    pub fn update(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.update_callback = Some(cb);
        self
    }
}

pub fn init<S>(state: S) -> AppBuilder<S> {
    AppBuilder {
        state: Some(state),
        draw_callback: None,
        update_callback: None,
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

//TODO think about this:
// - draw_2d() -> API easy like nannou api
//     draw_2d().transform().push(parentMatrix);
//     draw_2d().sprite(image)
//              .anchor(0.5, 0.5)
//              .rotation(1)
//              .scale(2, 2)
//              .filters([Filter::Blur, etc...])
//              .blend(BlendMode::Alpha)
//              .pos(100, 100);
// - draw() (or draw_raw())-> Stateful API like kha
//      gfx.begin(Some(Color::Red));
//      gfx.transform().push(matrix::scale(2, 2));
//      gfx.draw_image(image, 100, 100);
//      gfx.transform().pop();

fn load_resource(app: &mut App, state: &mut State, res: &str) {}

fn update_cb(app: &mut App, state: &mut State) {
    if !state.img.is_loaded() {
        state.img.try_load().unwrap();
    }
}

fn draw_shapes(app: &mut App, state: &mut State) {
    let gfx = &mut app.graphics;
    gfx.begin();
    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));

    //Moving circles
    let c = rgba(
        (state.i % 360) as f32 / 360.0,
        (state.i % 720) as f32 / 720.0,
        (state.i % 1080) as f32 / 1080.0,
        1.0,
    );
    gfx.set_color(c);
    gfx.transform().translate(150.0, 450.0);
    gfx.transform()
        .skew_deg((state.i % 720) as f32, (state.i % 720) as f32);
    gfx.draw_circle(0.0, 0.0, 50.0);
    gfx.transform().pop();
    gfx.set_color(Color::White);
    gfx.transform()
        .skew_deg(-(state.i % 720) as f32, -(state.i % 720) as f32);
    gfx.stroke_circle(0.0, 0.0, 50.0, 5.0);
    gfx.transform().pop();
    gfx.transform().pop();

    //top rect
    gfx.set_color(Color::Red);
    gfx.transform().scale(0.5, 0.5);
    gfx.draw_rect(0.0, 0.0, 100.0, 100.0);
    gfx.transform().pop();

    //middle triangle
    gfx.set_color(Color::Green);
    gfx.transform().scale(2.0, 2.0);
    gfx.draw_triangle(200.0, 200.0, 300.0, 300.0, 100.0, 300.0);
    gfx.transform().pop();
    gfx.draw_vertex(&[
        Vertex::new(600.0, 200.0, Color::Red),
        Vertex::new(700.0, 300.0, Color::Green),
        Vertex::new(500.0, 300.0, Color::Blue),
    ]);
    gfx.set_color(Color::Red.with_alpha(0.3));
    gfx.stroke_triangle(600.0, 200.0, 700.0, 300.0, 500.0, 300.0, 10.0);

    //rect arrow
    let max = 55;
    let len = state.i / 3 % max;
    for i in (0..len) {
        let n = i as f32;
        let r = (1.0 / len as f32) * n;
        let g = 0.5;
        let b = 1.0 - (1.0 / len as f32) * n;
        let a = 1.0;
        gfx.set_color(graphics::color::rgba(r, b, g, a));
        gfx.draw_rect(
            10.0 * n,
            10.0 * n,
            (100.0 / len as f32) * n,
            (100.0 / len as f32) * n,
        );
    }

    gfx.set_color(Color::Blue);
    gfx.draw_circle(200.0, 200.0, 50.0);
    gfx.stroke_circle(200.0, 200.0, 70.0, 10.0);
    gfx.set_color(Color::White);
    gfx.draw_line(200.0, 200.0, 300.0, 300.0, 10.0);
    gfx.draw_line(200.0, 300.0, 300.0, 200.0, 10.0);

    gfx.set_color(rgba(0.5, 0.5, 0.1, 1.0));
    gfx.draw_rounded_rect(300.0, 10.0, 200.0, 50.0, 20.0);
    gfx.set_color(rgba(1.0, 0.5, 0.5, 0.3));
    gfx.stroke_rounded_rect(300.0, 10.0, 200.0, 50.0, 20.0, 10.0);

    gfx.draw_rect(400.0, 100.0, 300.0, 80.0);
    gfx.set_color(Color::Green.with_alpha(0.3));
    gfx.stroke_rect(400.0, 100.0, 300.0, 80.0, 10.0);

    let (ww, hh) = (60.0, 60.0);
    gfx.set_color(Color::Red);
    gfx.set_alpha(0.5);
    gfx.transform().translate(430.0, 300.0);
    gfx.transform().rotate_deg(state.i as f32);
    gfx.draw_rect(-ww * 0.5, -hh * 0.5, ww, hh);
    gfx.transform().pop();
    gfx.transform().pop();

    gfx.set_color(Color::Blue);
    gfx.transform().translate(430.0, 300.0);
    gfx.transform().rotate_deg(state.i as f32 * 0.5);
    gfx.draw_rect(-ww * 0.5, -hh * 0.5, ww, hh);
    gfx.transform().pop();
    gfx.transform().pop();

    gfx.set_color(Color::Green);
    gfx.transform().translate(430.0, 300.0);
    gfx.transform().rotate_deg(-state.i as f32 * 0.5);
    gfx.draw_rect(-ww * 0.5, -hh * 0.5, ww, hh);
    gfx.transform().pop();
    gfx.transform().pop();
    gfx.set_alpha(1.0);

    gfx.end();

    state.i += 1;
}

struct State {
    pub i: i32,
    pub geom: Geometry,
    pub img: Texture,
}

fn draw_geometry(app: &mut App, state: &mut State) {
    let gfx = &mut app.graphics;
    gfx.begin();
    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
    gfx.draw_geometry(&mut state.geom);
    gfx.end();
}

fn draw_sprite(app: &mut App, state: &mut State) {
//    if !state.img.is_loaded() {
//        return;
//    }
    let gfx = &mut app.graphics;
    gfx.begin();
    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
//    gfx.transform().scale(3.0, 3.0);
    //    gfx.draw_geometry(&mut state.geom);
    gfx.draw_image(0.0, 0.0, &mut state.img);
    gfx.draw_image(10.0, 10.0, &mut state.img);
    gfx.draw_image(20.0, 20.0, &mut state.img);
    gfx.draw_image(30.0, 30.0, &mut state.img);
//    gfx.transform().pop();
//    gfx.transform().scale(5.0, 5.0);
//    gfx.draw_image(300.0, 300.0, &mut state.img);
    gfx.set_color(Color::Green);
    gfx.transform().translate(300.0, 300.0);
    gfx.transform().scale(15.0, 15.0);
    gfx.draw_cropped_image(0.0, 0.0, 10.0, 10.0, 10.0, 10.0, &mut state.img);
    gfx.transform().pop();
    gfx.transform().pop();
    gfx.set_color(Color::White);
    gfx.end();
}

struct Bunny {
    x: f32, y: f32, speed_x: f32, speed_y: f32
}

fn bunny_update(app: &mut App, state: &mut BState) {
    if !state.bunny.is_loaded() {
        state.bunny.try_load();
    }

    state.bunnies.push(
        Bunny {
            x: 0.0, y: 0.0,
            speed_x: js_sys::Math::random() as f32 * 10.0,
            speed_y: js_sys::Math::random() as f32  * 10.0 - 5.0,
        }
    );
    state.bunnies.iter_mut()
        .for_each(| b| {
            b.x += b.speed_x;
            b.y += b.speed_y;
            b.speed_y += 0.75;

            if b.x > 800.0 {
                b.speed_x *= -1.0;
                b.x = 800.0;
            } else if b.x < 0.0 {
                b.speed_x *= -1.0;
                b.x = 0.0
            }

            if b.y > 600.0 {
                b.speed_y *= -0.85;
                b.y = 600.0;
                if js_sys::Math::random() > 0.5 {
                    b.speed_y -= js_sys::Math::random() as f32 * 6.0;
                }
            } else if b.y < 0.0 {
                b.speed_y = 0.0;
                b.y = 0.0;
            }
        });
}

fn bunny(app: &mut App, state: &mut BState) {
    let gfx = &mut app.graphics;
    gfx.begin();
    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
    for b in &state.bunnies {
        gfx.draw_image(b.x, b.y, &mut state.bunny);
    }
//    state.bunnies.iter_mut()
//        .for_each(|b| gfx.draw_image(b.x, b.y, &mut state.bunny));
    gfx.end();
}

struct BState {
    bunny: Texture,
    bunnies: Vec<Bunny>
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("Hello, world!");
    let mut g = Geometry::new();
    g.rect(100.0, 100.0, 200.0, 200.0)
        .move_to(100.0, 100.0)
        .line_to(150.0, 150.0)
        .line_to(150.0, 200.0)
        .line_to(200.0, 400.0)
        .cubic_bezier_to(250.0, 400.0, 300.0, 450.0, 300.0, 100.0)
        .close_path()
        .circle(200.0, 400.0, 50.0)
        .stroke(Color::Green, 2.0)
        .rounded_rect(200.0, 400.0, 100.0, 100.0, 4.0)
        //        .stroke(Color::White, 2.0)
        .fill(Color::Red)
        .triangle(100.0, 100.0, 50.0, 150.0, 150.0, 150.0)
        .fill(Color::White)
        .move_to(100.0, 100.0)
        .quadratic_bezier_to(350.0, 150.0, 150.0, 300.0)
        //        .arc_to(150.0, 20.0, 150.0, 70.0, math::PI/180.0 * 50.0)
        .stroke(Color::White, 2.0)
        .build();

    let b_state = BState {
        bunny: Texture::new("b.png"),
        bunnies: vec![]
    };

    init(b_state)
        .draw(bunny)
        .update(bunny_update)
        .build()
        .unwrap();

    let state = State {
        i: 0,
        geom: g,
        img: Texture::new("h.png"),
    };

//    init(state)
//        //                .draw(draw_shapes)
//        //        .draw(draw_geometry)
//        .draw(draw_sprite)
//        .resource(load_resource)
//        .update(update_cb)
//        .build()
//        .unwrap();
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}
