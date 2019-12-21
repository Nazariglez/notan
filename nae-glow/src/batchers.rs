use crate::font::{Font, FontManager};

pub(crate) struct TextBatcher {
    pub font: Font,
    pub manager: FontManager<'static>,
}
