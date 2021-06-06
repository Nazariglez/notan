use glyph_brush::{ab_glyph::*, *};
use notan_graphics::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    pub(crate) id: FontId,
}

impl Font {
    pub fn id(&self) -> i32 {
        self.id.0 as _
    }
}
