use nae_core::resources::{BaseFont, VerticalAlign, HorizontalAlign, Resource, ResourceConstructor};
use nae_core::BaseApp;

pub struct Font {

}

impl BaseFont for Font {
    fn text_size<T: BaseApp, F: BaseFont>(app: &mut T, font: &F, text: &str, size: f32) -> (f32, f32) {
        unimplemented!()
    }

    fn text_size_ext<T: BaseApp, F: BaseFont>(app: &mut T, font: &F, text: &str, size: f32, h_align: HorizontalAlign, v_align: VerticalAlign, max_width: Option<f32>) -> (f32, f32) {
        unimplemented!()
    }
}

impl Resource for Font {
    fn parse<T: BaseApp>(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String> {
        unimplemented!()
    }

    fn is_loaded(&self) -> bool {
        unimplemented!()
    }
}

impl ResourceConstructor for Font {
    fn new(file: &str) -> Self {
        unimplemented!()
    }
}