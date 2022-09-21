use notan::draw::*;
use notan::prelude::*;
use wasm_bindgen::JsCast;

fn create_image() -> web_sys::HtmlImageElement {
    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();
    let mut img = doc
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    img.set_src("assets/ferris.png");
    let body = doc.body().unwrap();
    body.append_child(&img).unwrap();
    img
}

#[derive(AppState)]
struct State {
    img: Option<Texture>,
    html: web_sys::HtmlImageElement,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(|| State {
        img: None,
        html: create_image(),
    })
    .add_config(DrawConfig)
    .draw(draw)
    .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.img.is_none() && app.keyboard.was_pressed(KeyCode::Space) {
        let texture = gfx
            .create_texture()
            .from_html_image(&state.html)
            .build()
            .unwrap();
        state.img = Some(texture);
    }

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    if let Some(img) = &state.img {
        draw.image(img).position(250.0, 200.0);
    }
    gfx.render(&draw);
}
