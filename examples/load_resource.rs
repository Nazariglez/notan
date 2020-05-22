use nae::prelude::*;

struct Resources {
    image: Texture,
    blob: Blob,
    font: Font,
}

fn init(app: &mut App) -> Resources {
    Resources {
        blob: app.load_resource("./examples/assets/blob.txt").unwrap(),
        font: app.load_resource("./examples/assets/Ubuntu-B.ttf").unwrap(),
        image: app.load_resource("./examples/assets/rust.png").unwrap(),
    }
}

fn draw(app: &mut App, resources: &mut Resources) {
    let draw = app.draw();
    draw.begin(Color::ORANGE);

    if resources.image.is_loaded() && resources.blob.is_loaded() && resources.font.is_loaded() {
        draw.text(&resources.font, "All loaded", 160.0, 160.0, 29.0);
    }

    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
