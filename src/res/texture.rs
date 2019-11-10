use super::loader::load_file;
use super::resource::*;
use crate::graphics::batchers::GraphicTexture;

use futures::future::Future;
use glow::HasContext;
use std::cell::RefCell;
use std::rc::Rc;

//TODO add rect and rotation to support texturepacker?

#[derive(Debug, Clone)]
pub(crate) struct TextureData {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) raw: Vec<u8>,
    pub(crate) graphics: Option<GraphicTexture>,
}

impl TextureData {
    pub(crate) fn init_graphics(&mut self, g: GraphicTexture) {
        self.graphics = Some(g);
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
/// Represents an image resource
pub struct Texture {
    pub(crate) resource: Rc<RefCell<ResourceData>>,
    pub(crate) data: Rc<RefCell<Option<TextureData>>>, //TODO use a rc here to avoid clone the raw every time?
}

impl Texture {
    pub(crate) fn data(&mut self) -> &Rc<RefCell<Option<TextureData>>> {
        &self.data
    }

    /// Returns the graphics texture to be draw on the GPU
    pub fn tex(&self) -> Option<glow::WebTextureKey> {
        if let Some(d) = self.data.borrow().as_ref() {
            if let Some(g) = d.graphics.as_ref() {
                return Some(g.tex);
            }
        }

        None
    }

    /// Returns the texture's width
    pub fn width(&self) -> f32 {
        if let Some(d) = self.data.borrow().as_ref() {
            d.width as f32
        } else {
            0.0
        }
    }

    /// Returns the texture's height
    pub fn height(&self) -> f32 {
        if let Some(d) = self.data.borrow().as_ref() {
            d.height as f32
        } else {
            0.0
        }
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
            graphics: None,
        });

        Ok(())
    }

    fn is_loaded(&self) -> bool {
        self.data.borrow().is_some()
    }
}
