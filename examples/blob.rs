use nae::prelude::*;

fn init(app: &mut App) -> Blob {
    app.load_file("./examples/assets/blob.txt").unwrap()
}

fn draw(app: &mut App, blob: &mut Blob) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    if blob.is_loaded() {
        draw.text(&format!("Blob: {:?}", blob.data()), 10.0, 10.0, 24.0);
        draw.text(
            &format!(
                "Text from blob: {:?}",
                std::str::from_utf8(&blob.data()).unwrap()
            ),
            10.0,
            60.0,
            24.0,
        );
    }
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}
