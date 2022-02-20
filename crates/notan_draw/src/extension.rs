use crate::{Draw, DrawManager};
use notan_app::graphics::*;
use notan_text::{Text, TextExtension};

pub trait CreateDraw {
    fn create_draw(&self) -> Draw;
}

impl CreateDraw for Graphics {
    fn create_draw(&self) -> Draw {
        let (width, height) = self.device.size();
        Draw::new(width, height)
    }
}

pub struct DrawExtension {
    manager: DrawManager,
}

impl DrawExtension {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            manager: DrawManager::new(gfx)?,
        })
    }
}

impl GfxExtension<Draw> for DrawExtension {
    fn commands<'a>(&'a mut self, _device: &mut Device, _renderer: &'a Draw) -> &'a [Commands] {
        &[]
    }
}

impl GfxRenderer for Draw {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) {
        let mut text_ext = extensions.get_mut::<Text, TextExtension>().unwrap();
        let mut ext = extensions.get_mut::<Self, DrawExtension>().unwrap();
        let cmds = ext
            .manager
            .process_draw(self, device, text_ext.glyph_brush_mut());
        match target {
            None => device.render(cmds),
            Some(rt) => device.render_to(rt, cmds),
        }
    }
}
