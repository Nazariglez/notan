use notan::draw::*;
use notan::prelude::*;

const LOREM_IPSUM: &str = r#"
    Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of "de Finibus Bonorum et Malorum" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, "Lorem ipsum dolor sit amet..", comes from a line in section 1.10.32.

The standard chunk of Lorem Ipsum used since the 1500s is reproduced below for those interested. Sections 1.10.32 and 1.10.33 from "de Finibus Bonorum et Malorum" by Cicero are also reproduced in their exact original form, accompanied by English versions from the 1914 translation by H. Rackham.
"#;

#[derive(AppState)]
struct State {
    font: Font,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    State { font }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.text(&state.font, LOREM_IPSUM)
        .position(200.0, 300.0)
        .size(18.0)
        .color(Color::ORANGE)
        .max_width(360.0)
        .h_align_center()
        .v_align_middle();

    draw.text(&state.font, LOREM_IPSUM)
        .position(600.0, 300.0)
        .size(14.0)
        .color(Color::MAGENTA)
        .max_width(300.0)
        .h_align_center()
        .v_align_middle();

    gfx.render(&draw);
}
