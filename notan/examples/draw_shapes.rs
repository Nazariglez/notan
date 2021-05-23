use notan::app::assets::*;
use notan::app::config::WindowConfig;
use notan::app::graphics::prelude::*;
use notan::app::{App, AppBuilder, DrawImages, DrawPattern, DrawShapes, Graphics, Plugins};
use notan::log;
use notan::prelude::*;

// notan::handler!(Stuff u32 i32 f32 f64 S ());
// pub trait AH<S, Params> {
//     fn callback(self);
// }
//
// macro_rules! app_handler {
//     ($variant:expr, $($param:ident),*) => {
//         #[allow(unused_parens)]
//         impl<F, S> AH<S, ($(&mut $param),*)> for F
//         where
//             F: Fn($(&mut $param),*) + 'static,
//             S: AppState
//         {
//             fn callback(self) {
//                 //$variant(Box::new(self))
//             }
//         }
//     }
// }
//
// app_handler!(i32, i32, u32, f32);
// app_handler!(i32, f64, u32, f32);

//notan__handler!(Stuff<i32, u32, f32, f64, u64> -> S);
notan::handler!(Stuff);


#[notan::main]
fn main() -> Result<(), String> {
    notan::init().draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.line((20.0, 30.0), (780.0, 30.0)).width(4.0);

    draw.triangle((100.0, 100.0), (150.0, 200.0), (200.0, 100.0))
        .color(Color::YELLOW);

    draw.rect((500.0, 100.0), (200.0, 150.0))
        .color(Color::GREEN);

    draw.ellipse((400.0, 300.0), (50.0, 100.0))
        .color(Color::RED)
        .rotate_degrees(-45.0);

    draw.circle(40.0).position(600.0, 450.0).color(Color::BLUE);

    draw.rect((100.0, 400.0), (150.0, 100.0))
        .corner_radius(20.0)
        .color(Color::ORANGE)
        .stroke(15.0);

    gfx.render(&draw);
}
