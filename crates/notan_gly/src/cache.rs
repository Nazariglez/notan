use notan_app::{Graphics, Texture, TextureFilter, TextureFormat};

pub struct Cache {
    texture: Texture,
}

impl Cache {
    pub fn new(
        gfx: &mut Graphics,
        texture_width: u32,
        texture_height: u32,
    ) -> Result<Cache, String> {
        let texture = gfx
            .create_texture()
            .with_size(texture_width as _, texture_height as _)
            .with_filter(TextureFilter::Linear, TextureFilter::Linear)
            .with_format(TextureFormat::R8)
            .build()?;

        Ok(Self { texture })
    }

    pub fn update(
        &mut self,
        gfx: &mut Graphics,
        offset: [u16; 2],
        size: [u16; 2],
        data: &[u8],
    ) -> Result<(), String> {
        let [ox, oy] = offset;
        let [w, h] = size;

        gfx.update_texture(&mut self.texture)
            .with_x_offset(ox as _)
            .with_y_offset(oy as _)
            .with_width(w as _)
            .with_height(h as _)
            .with_data(data)
            .update()
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
