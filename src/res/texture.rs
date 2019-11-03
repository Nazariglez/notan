use std::rc::Rc;
use std::cell::RefCell;
use futures::future::Future;
use super::resource::*;
use super::loader::load_file;
use glow::HasContext;
use crate::graphics::shaders::GraphicTexture;
use crate::log;

#[derive(Debug, Clone)]
pub struct TextureData {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) raw: Vec<u8>,
    pub(crate) graphics: Option<GraphicTexture>,
}

impl TextureData {
    pub(crate) fn init_graphics(&mut self, g:GraphicTexture) {
        if self.has_context() {
            return;
        }

        self.graphics = Some(g);
    }

    pub fn has_context(&self) -> bool {
        self.graphics.is_some()
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        if let Some(g) = &self.graphics {
            let tex = g.tex;
            unsafe {
                g.gl.delete_texture(tex);
            }
        }
    }
}


#[derive(Clone)]
pub struct Texture {
    pub(crate) resource: Rc<RefCell<ResourceData>>,
    pub(crate) data: Rc<RefCell<Option<TextureData>>>, //TODO use a rc here to avoid clone the raw every time?
}

impl Texture {
    pub fn data(&mut self) -> &Rc<RefCell<Option<TextureData>>> {
        &self.data
    }

    pub fn tex(&self) -> glow::WebTextureKey {
        self.data
            .borrow()
            .as_ref()
            .unwrap()
            .graphics
            .as_ref()
            .unwrap()
            .tex
    }

    pub fn width(&self) -> i32 {
        self.data
            .borrow()
            .as_ref()
            .unwrap()
            .width
    }

    pub fn height(&self) -> i32 {
        self.data
            .borrow()
            .as_ref()
            .unwrap()
            .height
    }
}

impl ResourceConstructor for Texture {
    fn new(file: &str) -> Self {
        Self {
            resource: ResourceData::rc(Box::new(load_file(file))),
            data: Rc::new(RefCell::new(None)),
        }
    }
}

impl Resource for Texture {
    fn resource_data(&self) -> &Rc<RefCell<ResourceData>> {
        &self.resource
    }

    fn on_load(&mut self) -> Result<(), String> {
        let data = image::load_from_memory(self.resource_data().borrow().data())
            .map_err(|e| e.to_string())?
            .to_rgba();

        let width = data.width() as i32;
        let height = data.height() as i32;
        let raw = data.to_vec();

        *self.data.borrow_mut() = Some(TextureData {
            width,
            height,
            raw,
            graphics: None
        });

        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.data.borrow().is_some()
    }
}
