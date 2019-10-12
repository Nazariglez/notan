use super::{Context};
use super::shaders::ColorBatcher;

pub struct Renderer {
    ctx: Context,
    color_batcher: ColorBatcher,
    //sprite_batcher: SpriteBatcher
}

impl Renderer { //remove Context and mov the logic here...
    pub fn new(win: &web_sys::HtmlCanvasElement) -> Result<Self, String> {
        let ctx = Context::new(win)?;
        let color_batcher = ColorBatcher::new(&ctx)?;
        Ok(Self {
            ctx: ctx,
            color_batcher: color_batcher
        })
    }

    pub fn begin() {

    }
}