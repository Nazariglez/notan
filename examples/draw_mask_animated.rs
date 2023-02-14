use notan::{draw::*, prelude::*};

fn main() {
    notan::init()
        .add_config(WindowConfig::new().set_size(500, 500))
        .add_config(DrawConfig)
        .draw(|app: &mut App, gfx: &mut Graphics| {
            let mut draw = gfx.create_draw();
            draw.clear(Color::BLACK);

            let elapsed = app.timer.elapsed_f32();
            let pulse_progress = ((elapsed % 4.) - 2.).abs() / 2.; // 4s roundtrip pulse
            let radius = 200. * (1. - pulse_progress);

            let mut mask = gfx.create_draw();
            mask.clear(Color::BLACK);
            mask.circle(radius).position(250., 250.);

            draw.mask(Some(&mask));
            draw.rect((0., 0.), (500., 500.)).color(Color::GREEN);
            draw.mask(None);

            gfx.render(&draw);
        })
        .build()
        .unwrap();
}
