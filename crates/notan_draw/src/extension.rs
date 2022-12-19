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

impl CreateDraw for RenderTexture {
    fn create_draw(&self) -> Draw {
        let (width, height) = self.size();
        Draw::new(width as _, height as _)
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

impl GfxExtension<Draw> for DrawExtension {}

impl GfxRenderer for Draw {
    fn render(
        &self,
        device: &mut Device,
        extensions: &mut ExtContainer,
        target: Option<&RenderTexture>,
    ) -> Result<(), String> {
        let mut text_ext = extensions.get_mut::<Text, TextExtension>().ok_or_else(|| {
            "Missing TextExtension. You may need to add 'DrawConfig' to notan.".to_string()
        })?;
        let mut ext = extensions.get_mut::<Self, DrawExtension>().ok_or_else(|| {
            "Missing DrawExtension. You may need to add 'DrawConfig' to notan.".to_string()
        })?;

        let cmds =
            ext.manager
                .process_draw(self, device, text_ext.glyph_brush_mut(), target.is_some());
        match target {
            None => device.render(cmds),
            Some(rt) => device.render_to(rt, cmds),
        }

        Ok(())
    }
}
