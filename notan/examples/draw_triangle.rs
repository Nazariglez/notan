use notan::draw::*;
use notan::prelude::*;

#[notan::main]
fn main() -> Result<(), String> {
    notan::init_with(|gfx: &mut Graphics| {
        let ext = DrawExtension::new(gfx).unwrap();
        gfx.add_ext(ext);
    })
    .draw(draw)
    .build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);
}
