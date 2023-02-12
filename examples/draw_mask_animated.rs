use notan::{draw::*, prelude::*};

fn main() {
    notan::init()
        .add_config(WindowConfig::new().size(500, 500))
        .add_config(DrawConfig)
        .draw(|app: &mut App, gfx: &mut Graphics| {
            let mut draw = gfx.create_draw();
            draw.clear(Color::BLACK);

            let elapsed = app.timer.time_since_init();
            let pulse_progress = ((elapsed % 4.) - 2.).abs() / 2.; // 4s roundtrip pulse
            let radius = 200. * (1. - pulse_progress);
            dbg!(radius, pulse_progress);

            let mut mask = gfx.create_draw();
            mask.clear(Color::BLACK);
            mask.circle(radius).position(250., 250.);

            // draw.circle(10.0).position(100.0, 100.0).color(Color::RED);

            draw.mask(Some(&mask));
            draw.rect((0., 0.), (500., 500.)).color(Color::GREEN);
            draw.mask(None);

            // draw.circle(10.0).position(10.0, 10.0).color(Color::RED);

            gfx.render(&draw);
        })
        .build()
        .unwrap();
}
