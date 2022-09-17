use notan::draw::*;
use notan::prelude::*;

// fn create_image() -> web_sys::HtmlImageElement {
//     let win = web_sys::window().unwrap();
//     let doc = win.document().unwrap();
//     let mut img = doc
//         .create_element("img")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlImageElement>()
//         .unwrap();
//     img.set_src("https://i.pinimg.com/474x/bd/9c/c1/bd9cc1830b4263376c050f77327356db.jpg");
//     img.set_width(50);
//     img.set_height(50);
//     let body = doc.body().unwrap();
//     body.append_child(&img).unwrap();
//     img
// }

#[derive(AppState)]
struct State {
    img: Texture,
}

#[notan_main]
fn main() -> Result<(), String> {
    // let img = create_image();
    // notan::log::info!("here 1 img {}", img.src());
    notan::init_with(init)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init(gfx: &mut Graphics) -> State {
    // let tex = notan::backend::create_texture_from_html(&mut gfx.device).unwrap();
    // notan::log::info!("texture {:?}", tex);

    let source = notan::backend::TextureSourceImage(include_bytes!("assets/ferris.png").to_vec());

    notan::log::info!("HERTE?");

    let texture = gfx
        .create_texture()
        .from_raw_source(source)
        .from_image(include_bytes!("assets/ferris.png"))
        .build()
        .unwrap();

    notan::log::info!("TEXTURE ID {}", texture.id());

    State { img: texture }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.img).position(250.0, 200.0);
    gfx.render(&draw);
}
