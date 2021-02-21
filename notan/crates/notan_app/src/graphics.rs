use notan_draw::Draw;
pub use notan_graphics::*;

pub struct Graphics {
    gfx: Device,
    // draw: DrawManager,
}

impl Graphics {
    pub fn new(backend: Box<DeviceBackend>) -> Result<Self, String> {
        let gfx = Device::new(backend)?;
        Ok(Self { gfx })
    }

    #[inline(always)]
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.gfx.set_size(width, height)
    }

    #[inline(always)]
    pub fn create_renderer<'a>(&self) -> renderer::Renderer<'a> {
        self.gfx.create_renderer()
    }

    #[inline(always)]
    pub fn create_draw<'a>(&self) -> Draw<'a> {
        Draw::new(self.create_renderer())
    }

    #[inline(always)]
    pub fn clean(&mut self) {
        self.gfx.clean()
    }

    #[inline(always)]
    pub fn size(&self) -> (i32, i32) {
        self.gfx.size()
    }

    #[inline(always)]
    pub fn create_texture(&mut self, info: TextureInfo) -> Result<Texture, String> {
        self.gfx.create_texture(info)
    }
}
