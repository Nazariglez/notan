use crate::{Draw, DrawManager};
use notan_app::graphics::*;
use notan_glyph::{Font, GlyphPlugin};

pub trait CreateDraw {
    fn create_draw(&self) -> Draw;
    fn create_font(&self, data: &[u8]) -> Result<Font, String>;
}

impl CreateDraw for Graphics {
    fn create_draw(&self) -> Draw {
        let (width, height) = self.device.size();
        Draw::new(width, height)
    }

    fn create_font(&self, data: &[u8]) -> Result<Font, String> {
        let mut ext = self
            .get_ext_mut::<Draw, DrawExtension>()
            .ok_or_else(|| "The DrawExtension is not in use.".to_string())?;

        ext.glyphs.create_font(data)
    }
}

pub struct DrawExtension {
    manager: DrawManager,
    glyphs: GlyphPlugin,
}

impl DrawExtension {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            manager: DrawManager::new(gfx)?,
            glyphs: GlyphPlugin::new(gfx)?,
        })
    }
}

impl GfxExtension<Draw> for DrawExtension {
    fn commands<'a>(&'a mut self, device: &mut Device, renderer: &'a Draw) -> &'a [Commands] {
        self.manager
            .process_draw(renderer, device, &mut self.glyphs)
    }
}

impl GfxRenderer for Draw {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let mut plugin = extensions.get_mut::<Self, DrawExtension>().unwrap();
        let cmds = plugin.commands(device, self);
        match target {
            None => device.render(cmds),
            Some(rt) => device.render_to(rt, cmds),
        }
    }
}
