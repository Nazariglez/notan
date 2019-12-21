use nae_core::graphics::BaseShader;
use nae_core::BaseApp;

pub struct Shader {}

impl BaseShader for Shader {
    type Uniform = ();
    type Buffer = ();
    type Attr = ();

    fn new<T: BaseApp>(
        app: &T,
        vertex: &str,
        fragment: &str,
        attributes: Vec<Self::Attr>,
    ) -> Result<Self, String> {
        unimplemented!()
    }

    fn set_uniform(&self, name: &str, value: Self::Uniform) -> Result<(), String> {
        unimplemented!()
    }

    fn buffer(&self, name: &str) -> Option<Self::Buffer> {
        unimplemented!()
    }

    fn from_image_fragment<T: BaseApp>(app: &T, fragment: &str) -> Result<Self, String> {
        unimplemented!()
    }

    fn from_text_fragment<T: BaseApp>(app: &T, fragment: &str) -> Result<Self, String> {
        unimplemented!()
    }

    fn from_color_fragment<T: BaseApp>(app: &T, fragment: &str) -> Result<Self, String> {
        unimplemented!()
    }

    fn is_equal<T: BaseShader>(&self, shader: &T) -> bool {
        unimplemented!()
    }
}
