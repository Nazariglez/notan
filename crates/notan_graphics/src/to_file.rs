use crate::Device;
use crate::Texture;
use image::ColorType;
use notan_utils::save_file;

pub(crate) fn save_to_png_file<P: AsRef<std::path::Path>>(
    gfx: &mut Device,
    texture: &Texture,
    inverse: bool,
    path: P,
) -> Result<(), String> {
    use image::ImageEncoder;

    let bpp = texture.format().bytes_per_pixel() as usize;
    let width = texture.width() as usize;
    let height = texture.height() as usize;
    let len = width * height * bpp;

    let mut bytes = vec![0; len];
    gfx.read_pixels(texture).read_to(&mut bytes)?;

    if inverse {
        bytes = bytes.chunks(width * bpp).rev().flatten().cloned().collect();
    }

    let p = path.as_ref();
    p.with_extension(".png");

    let typ = match bpp {
        4 => ColorType::Rgba8,
        1 => ColorType::L8,
        _ => return Err("Invalid type format".to_string()),
    };

    let mut data = vec![];
    let encoder = image::codecs::png::PngEncoder::new(&mut data);
    encoder
        .write_image(&bytes, width as _, height as _, typ)
        .map_err(|e| e.to_string())?;

    save_file(p, &data)
}
