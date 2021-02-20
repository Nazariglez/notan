use notan_draw::Draw;
use notan_graphics as ngfx;

pub struct Graphics {
    gfx: ngfx::Graphics,
    // draw: DrawManager,
}

impl Graphics {
    pub fn new(backend: Box<ngfx::GraphicsBackend>) -> Result<Self, String> {
        let gfx = ngfx::Graphics::new(backend)?;
        Ok(Self { gfx })
    }

    #[inline(always)]
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.gfx.set_size(width, height)
    }

    #[inline(always)]
    pub fn create_renderer<'a>(&self) -> ngfx::renderer::Renderer<'a> {
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
    pub fn create_texture(&mut self, info: ngfx::TextureInfo) -> Result<ngfx::Texture, String> {
        self.gfx.create_texture(info)
    }
}
