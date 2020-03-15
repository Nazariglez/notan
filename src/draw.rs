use nae_core::{BaseGfx, ClearOptions, Color};
use nae_gfx::Graphics;
use std::cell::RefMut;

pub struct Draw<'gfx> {
    pub gfx: RefMut<'gfx, Graphics>,
    clear_options: ClearOptions,
    current_color: Color,
}

impl<'gfx> Draw<'gfx> {
    pub fn new(gfx: RefMut<'gfx, Graphics>) -> Self {
        Self {
            gfx,
            clear_options: Default::default(),
            current_color: Color::WHITE,
        }
    }

    pub fn begin(&mut self, color: Color) {
        self.clear_options.color = Some(color);

        self.gfx.begin(&self.clear_options);
    }

    pub fn end(&mut self) {
        self.gfx.end();
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {}
}
