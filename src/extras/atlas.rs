use backend::{BaseSystem, Context2d, Resource};
use serde::{Deserialize, Serialize};

struct Atlas {}

impl Resource for Atlas {
    type Context2d = Context2d;

    fn new(file: &str) -> Self {
        unimplemented!()
    }

    // TODO https://serde.rs/derive.html
    fn parse<T>(&mut self, sys: &mut T, data: Vec<u8>) -> Result<(), String>
    where
        T: BaseSystem<Context2d = Self::Context2d>,
    {
        unimplemented!()
    }

    fn is_loaded(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasRoot {
    frames: Vec<u32>,
    meta: AtlasMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasFrame {
    filename: String,
    frame: AtlasRect,
    rotated: bool,
    trimmed: bool,
    sprite_source_size: AtlasRect,
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
    scale: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AtlasPoint {
    x: i32,
    y: i32,
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
