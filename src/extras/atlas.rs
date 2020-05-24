use crate::res::{Resource, ResourceParser};
use crate::{resource_parser, App};
use backend::{BaseApp, BaseSystem, Texture};
use hashbrown::HashMap;
use nae_gfx::Draw;
use serde::{Deserialize, Serialize};
use std::cell::Ref;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

struct InnerAtlas {
    data: AtlasRoot,
    pub(crate) tex: Texture,
}

#[derive(Clone)]
pub struct TextureAtlas {
    root: String,
    inner: Rc<RefCell<Option<InnerAtlas>>>,
    textures: Rc<RefCell<HashMap<String, Texture>>>,
}

impl TextureAtlas {
    pub fn textures(&self) -> Ref<HashMap<String, Texture>> {
        return Ref::map(self.textures.borrow(), |textures| textures);
    }

    pub fn is_loaded(&self) -> bool {
        match &*self.inner.borrow() {
            Some(inner) => inner.tex.is_loaded(),
            _ => false,
        }
    }
}

impl ResourceParser for TextureAtlas {
    type App = App;

    fn parse_resource(&mut self, app: &mut Self::App, data: Vec<u8>) -> Result<(), String> {
        let data: AtlasRoot = serde_json::from_slice(&data).map_err(|e| e.to_string())?;
        let path = Path::new(&self.root).join(&data.meta.image);
        let tex: Texture = app.load_resource(&path.display().to_string())?;

        let mut textures = self.textures.borrow_mut();
        for frame in &data.frames {
            textures.insert(
                frame.filename.to_string(),
                tex.with_frame(
                    frame.frame.x as _,
                    frame.frame.y as _,
                    frame.frame.w as _,
                    frame.frame.h as _,
                ),
            );
        }

        *self.inner.borrow_mut() = Some(InnerAtlas { data, tex });

        Ok(())
    }
}

impl<T, S> Resource<T> for TextureAtlas
where
    T: BaseApp<System = S>,
    S: BaseSystem<Draw = Draw>,
{
    fn prepare(app: &mut T, file: &str) -> Result<Self, String> {
        let path = std::path::Path::new(file);
        let root = path.parent().unwrap_or(path).display().to_string();
        Ok(Self {
            inner: Rc::new(RefCell::new(None)),
            root: root,
            textures: Rc::new(RefCell::new(HashMap::new())),
        })
    }

    fn set_data(&mut self, _app: &mut T, _data: Vec<u8>) -> Result<(), String> {
        Ok(()) //no-op
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasRoot {
    frames: Vec<AtlasFrame>,
    meta: AtlasMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasFrame {
    filename: String,
    frame: AtlasRect,
    rotated: bool,
    trimmed: bool,
    #[serde(alias = "spriteSourceSize")]
    sprite_source_size: AtlasRect,
    #[serde(alias = "sourceSize")]
    source_size: AtlasSize,
    pivot: AtlasPoint,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasMeta {
    app: String,
    version: String,
    image: String,
    format: String,
    size: AtlasSize,
    scale: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasPoint {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasSize {
    w: i32,
    h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
