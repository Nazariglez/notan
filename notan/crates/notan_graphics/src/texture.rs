use crate::graphics::{DropManager, ResourceId};
use crate::pipeline::*;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TextureInfo {
    width: i32,
    height: i32,
    format: TextureFormat,
    internal_format: TextureFormat,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
    bytes: Option<Vec<u8>>,
}

impl Default for TextureInfo {
    fn default() -> Self {
        Self {
            format: TextureFormat::Rgba,
            internal_format: TextureFormat::Rgba,
            mag_filter: TextureFilter::Nearest,
            min_filter: TextureFilter::Nearest,
            width: 1,
            height: 1,
            bytes: None,
        }
    }
}

#[derive(Debug)]
struct TextureId {
    id: i32,
    drop_manager: Arc<DropManager>,
}

impl Drop for TextureId {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::Texture(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    id: Arc<TextureId>,
    data: Arc<Vec<u8>>,
    width: i32,
    height: i32,
    format: TextureFormat,
    internal_format: TextureFormat,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
}
//https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#getting-data-into-a-texture
impl Texture {
    pub(crate) fn new(id: i32, info: TextureInfo, drop_manager: Arc<DropManager>) -> Self {
        let id = Arc::new(TextureId { id, drop_manager });

        let TextureInfo {
            width,
            height,
            format,
            internal_format,
            min_filter,
            mag_filter,
            bytes,
        } = info;

        let data = Arc::new(bytes.unwrap_or_else(|| vec![]));

        Self {
            id,
            data,
            width,
            height,
            format,
            internal_format,
            min_filter,
            mag_filter,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
    }

    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    #[inline(always)]
    pub fn format(&self) -> &TextureFormat {
        &self.format
    }

    #[inline(always)]
    pub fn internal_format(&self) -> &TextureFormat {
        &self.internal_format
    }

    #[inline(always)]
    pub fn min_filter(&self) -> &TextureFilter {
        &self.min_filter
    }

    #[inline(always)]
    pub fn mag_filter(&self) -> &TextureFilter {
        &self.mag_filter
    }

    #[inline(always)]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba,
    Red,
    R8,
    Depth,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}
