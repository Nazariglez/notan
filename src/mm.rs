//mod glm;
//mod graphics;
//mod math;
//mod res;
//mod window;
//mod app;
//
//use crate::graphics::color::{rgba, Color};
//use crate::graphics::Vertex;
//use crate::math::Geometry;
//use std::rc::Rc;
//use wasm_bindgen::__rt::core::cell::RefCell;
//use wasm_bindgen::__rt::std::collections::HashMap;
//use app::*;
//#[cfg(target_arch = "wasm32")]
//use wasm_bindgen::prelude::*;
//
//use res::*;
//
//#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
//pub fn wasm_main() {
//    main();
//}
//
////TODO think about this:
//// - draw_2d() -> API easy like nannou api
////     draw_2d().transform().push(parentMatrix);
////     draw_2d().sprite(image)
////              .anchor(0.5, 0.5)
////              .rotation(1)
////              .scale(2, 2)
////              .filters([Filter::Blur, etc...])
////              .blend(BlendMode::Alpha)
////              .pos(100, 100);
//// - draw() (or draw_raw())-> Stateful API like kha
////      gfx.begin(Some(Color::Red));
////      gfx.transform().push(matrix::scale(2, 2));
////      gfx.draw_image(image, 100, 100);
////      gfx.transform().pop();
//
//fn load_resource(app: &mut App, state: &mut State, res: &str) {}
//
//fn update_cb(app: &mut App, state: &mut State) {
//    if !state.img.is_loaded() {
//        state.img.try_load().unwrap();
//    }
//}
//
//fn start_cb(app: &mut App, state: &mut State) {
//    state.img2 = Some(app.load::<Texture>("b.png").unwrap());
//}
//
//fn draw_shapes(app: &mut App, state: &mut State) {
//    let gfx = &mut app.graphics;
//    gfx.begin();
//    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
//
//    //Moving circles
//    let c = rgba(
//        (state.i % 360) as f32 / 360.0,
//        (state.i % 720) as f32 / 720.0,
//        (state.i % 1080) as f32 / 1080.0,
//        1.0,
//    );
//    gfx.set_color(c);
//    gfx.transform().translate(150.0, 450.0);
//    gfx.transform()
//        .skew_deg((state.i % 720) as f32, (state.i % 720) as f32);
//    gfx.circle(0.0, 0.0, 50.0);
//    gfx.transform().pop();
//    gfx.set_color(Color::White);
//    gfx.transform()
//        .skew_deg(-(state.i % 720) as f32, -(state.i % 720) as f32);
//    gfx.stroke_circle(0.0, 0.0, 50.0, 5.0);
//    gfx.transform().pop();
//    gfx.transform().pop();
//
//    //top rect
//    gfx.set_color(Color::Red);
//    gfx.transform().scale(0.5, 0.5);
//    gfx.rect(0.0, 0.0, 100.0, 100.0);
//    gfx.transform().pop();
//
//    //middle triangle
//    gfx.set_color(Color::Green);
//    gfx.transform().scale(2.0, 2.0);
//    gfx.triangle(200.0, 200.0, 300.0, 300.0, 100.0, 300.0);
//    gfx.transform().pop();
//    gfx.vertex(&[
//        Vertex::new(600.0, 200.0, Color::Red),
//        Vertex::new(700.0, 300.0, Color::Green),
//        Vertex::new(500.0, 300.0, Color::Blue),
//    ]);
//    gfx.set_color(Color::Red.with_alpha(0.3));
//    gfx.stroke_triangle(600.0, 200.0, 700.0, 300.0, 500.0, 300.0, 10.0);
//
//    //rect arrow
//    let max = 55;
//    let len = state.i / 3 % max;
//    for i in (0..len) {
//        let n = i as f32;
//        let r = (1.0 / len as f32) * n;
//        let g = 0.5;
//        let b = 1.0 - (1.0 / len as f32) * n;
//        let a = 1.0;
//        gfx.set_color(graphics::color::rgba(r, b, g, a));
//        gfx.rect(
//            10.0 * n,
//            10.0 * n,
//            (100.0 / len as f32) * n,
//            (100.0 / len as f32) * n,
//        );
//    }
//
//    gfx.set_color(Color::Blue);
//    gfx.circle(200.0, 200.0, 50.0);
//    gfx.stroke_circle(200.0, 200.0, 70.0, 10.0);
//    gfx.set_color(Color::White);
//    gfx.line(200.0, 200.0, 300.0, 300.0, 10.0);
//    gfx.line(200.0, 300.0, 300.0, 200.0, 10.0);
//
//    gfx.set_color(rgba(0.5, 0.5, 0.1, 1.0));
//    gfx.rounded_rect(300.0, 10.0, 200.0, 50.0, 20.0);
//    gfx.set_color(rgba(1.0, 0.5, 0.5, 0.3));
//    gfx.stroke_rounded_rect(300.0, 10.0, 200.0, 50.0, 20.0, 10.0);
//
//    gfx.rect(400.0, 100.0, 300.0, 80.0);
//    gfx.set_color(Color::Green.with_alpha(0.3));
//    gfx.stroke_rect(400.0, 100.0, 300.0, 80.0, 10.0);
//
//    let (ww, hh) = (60.0, 60.0);
//    gfx.set_color(Color::Red);
//    gfx.set_alpha(0.5);
//    gfx.transform().translate(430.0, 300.0);
//    gfx.transform().rotate_deg(state.i as f32);
//    gfx.rect(-ww * 0.5, -hh * 0.5, ww, hh);
//    gfx.transform().pop();
//    gfx.transform().pop();
//
//    gfx.set_color(Color::Blue);
//    gfx.transform().translate(430.0, 300.0);
//    gfx.transform().rotate_deg(state.i as f32 * 0.5);
//    gfx.rect(-ww * 0.5, -hh * 0.5, ww, hh);
//    gfx.transform().pop();
//    gfx.transform().pop();
//
//    gfx.set_color(Color::Green);
//    gfx.transform().translate(430.0, 300.0);
//    gfx.transform().rotate_deg(-state.i as f32 * 0.5);
//    gfx.rect(-ww * 0.5, -hh * 0.5, ww, hh);
//    gfx.transform().pop();
//    gfx.transform().pop();
//    gfx.set_alpha(1.0);
//
//    gfx.end();
//
//    state.i += 1;
//}
//
//struct State {
//    pub i: i32,
//    pub geom: Geometry,
//    pub img: Texture,
//    pub img2: Option<Texture>,
//}
//
//fn draw_geometry(app: &mut App, state: &mut State) {
//    let gfx = &mut app.graphics;
//    gfx.begin();
//    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
//    gfx.geometry(&mut state.geom);
//    gfx.end();
//}
//
//fn draw_sprite(app: &mut App, state: &mut State) {
//    //    if !state.img.is_loaded() {
//    //        return;
//    //    }
//    let gfx = &mut app.graphics;
//    gfx.begin();
//    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
//    //    gfx.transform().scale(3.0, 3.0);
//    //    gfx.draw_geometry(&mut state.geom);
//    //    gfx.image(0.0, 0.0, &mut state.img);
//    //    gfx.image(10.0, 10.0, &mut state.img);
//    //    gfx.image(20.0, 20.0, &mut state.img);
//    //    gfx.image(30.0, 30.0, &mut state.img);
//    //    gfx.transform().pop();
//    //    gfx.transform().scale(5.0, 5.0);
//    //    gfx.draw_image(300.0, 300.0, &mut state.img);
//    gfx.set_color(Color::Green);
//    gfx.transform().translate(300.0, 300.0);
//    gfx.transform().scale(15.0, 15.0);
//    //    gfx.cropped_image(0.0, 0.0, 10.0, 10.0, 10.0, 10.0, &mut state.img);
//    gfx.transform().pop();
//    gfx.transform().pop();
//    gfx.set_color(Color::White);
//    gfx.end();
//}
//
//struct Bunny {
//    x: f32,
//    y: f32,
//    speed_x: f32,
//    speed_y: f32,
//    color: Color,
//}
//
//fn random_color() -> Color {
//    rgba(
//        js_sys::Math::random() as f32,
//        js_sys::Math::random() as f32,
//        js_sys::Math::random() as f32,
//        1.0,
//    )
//}
//
//fn bunny_update(app: &mut App, state: &mut BState) {
//    for _ in 0..10 {
//        state.bunnies.push(Bunny {
//            x: 0.0,
//            y: 0.0,
//            speed_x: js_sys::Math::random() as f32 * 10.0,
//            speed_y: js_sys::Math::random() as f32 * 10.0 - 5.0,
//            color: random_color(),
//        });
//    }
//
//    state.bunnies.iter_mut().for_each(|b| {
//        b.x += b.speed_x;
//        b.y += b.speed_y;
//        b.speed_y += 0.75;
//
//        if b.x > 800.0 {
//            b.speed_x *= -1.0;
//            b.x = 800.0;
//        } else if b.x < 0.0 {
//            b.speed_x *= -1.0;
//            b.x = 0.0
//        }
//
//        if b.y > 600.0 {
//            b.speed_y *= -0.85;
//            b.y = 600.0;
//            if js_sys::Math::random() > 0.5 {
//                b.speed_y -= js_sys::Math::random() as f32 * 6.0;
//            }
//        } else if b.y < 0.0 {
//            b.speed_y = 0.0;
//            b.y = 0.0;
//        }
//    });
//}
//
//fn bunny(app: &mut App, state: &mut BState) {
//    let bunny = state.bunny.as_mut().unwrap();
//    let gfx = &mut app.graphics;
//    gfx.begin();
//    gfx.clear(rgba(0.1, 0.2, 0.3, 1.0));
//    for b in &state.bunnies {
//        gfx.set_color(b.color);
//        gfx.image(bunny, b.x, b.y);
//    }
//    gfx.end();
//}
//
//fn start_bunny(app: &mut App, state: &mut BState) {
//    state.bunny = app.load("b.png").ok();
//}
//
//struct BState {
//    bunny: Option<Texture>,
//    bunnies: Vec<Bunny>,
//}
//
//fn main() {
//    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//    log("Hello, world!");
//    let mut g = Geometry::new();
//    g.rect(100.0, 100.0, 200.0, 200.0)
//        .move_to(100.0, 100.0)
//        .line_to(150.0, 150.0)
//        .line_to(150.0, 200.0)
//        .line_to(200.0, 400.0)
//        .cubic_bezier_to(250.0, 400.0, 300.0, 450.0, 300.0, 100.0)
//        .close_path()
//        .circle(200.0, 400.0, 50.0)
//        .stroke(Color::Green, 2.0)
//        .rounded_rect(200.0, 400.0, 100.0, 100.0, 4.0)
//        //        .stroke(Color::White, 2.0)
//        .fill(Color::Red)
//        .triangle(100.0, 100.0, 50.0, 150.0, 150.0, 150.0)
//        .fill(Color::White)
//        .move_to(100.0, 100.0)
//        .quadratic_bezier_to(350.0, 150.0, 150.0, 300.0)
//        //        .arc_to(150.0, 20.0, 150.0, 70.0, math::PI/180.0 * 50.0)
//        .stroke(Color::White, 2.0)
//        .build();
//
//    let b_state = BState {
//        bunny: None,
//        bunnies: vec![],
//    };
//
//    init(b_state)
//        .start(start_bunny)
//        .draw(bunny)
//        .update(bunny_update)
//        .build()
//        .unwrap();
//
//    let state = State {
//        i: 0,
//        geom: g,
//        img: Texture::new("h.png"),
//        img2: None,
//    };
//
//    //    init(state)
//    //        //                .draw(draw_shapes)
//    //        //        .draw(draw_geometry)
//    //        .draw(draw_sprite)
//    //        .resource(load_resource)
//    //        .update(update_cb)
//    //        .build()
//    //        .unwrap();
//}
//
//pub fn log(msg: &str) {
//    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
//}
