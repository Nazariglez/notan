use crate::graphics::{DropManager, ResourceId};
use crate::pipeline::*;
use notan_math::Rect;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TextureInfo {
    pub width: i32,
    pub height: i32,
    pub format: TextureFormat,
    pub internal_format: TextureFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub bytes: Option<Vec<u8>>,

    /// Used for render textures
    pub depth: bool,
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
            depth: false,
        }
    }
}

impl TextureInfo {
    pub fn render_texture(depth: bool, width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            depth,
            ..Default::default()
        }
    }

    pub fn from_image(bytes: &[u8]) -> Result<Self, String> {
        Self::from_image_with_options(
            bytes,
            TextureFormat::Rgba,
            TextureFormat::Rgba,
            TextureFilter::Nearest,
            TextureFilter::Nearest,
        )
    }

    pub fn from_image_with_options(
        bytes: &[u8],
        format: TextureFormat,
        internal_format: TextureFormat,
        mag_filter: TextureFilter,
        min_filter: TextureFilter,
    ) -> Result<Self, String> {
        let data = image::load_from_memory(bytes)
            .map_err(|e| e.to_string())?
            .to_rgba();

        Ok(Self {
            width: data.width() as _,
            height: data.height() as _,
            bytes: Some(data.to_vec()),
            format,
            internal_format,
            mag_filter,
            min_filter,
            depth: false,
        })
    }

    pub fn bytes_per_pixel(&self) -> usize {
        use TextureFormat::*;
        match (self.format, self.internal_format) {
            (Red, R8) => 1,
            _ => 4, //TODO
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
    // data: Arc<Vec<u8>>,
    width: i32,
    height: i32,
    format: TextureFormat,
    internal_format: TextureFormat,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
    frame: Rect,
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
            depth,
        } = info;

        // let data = Arc::new(bytes);
        let frame = Rect {
            x: 0.0,
            y: 0.0,
            width: width as _,
            height: height as _,
        };

        Self {
            id,
            // data,
            width,
            height,
            format,
            internal_format,
            min_filter,
            mag_filter,
            frame,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.id.id
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

    // #[inline(always)]
    // pub fn data(&self) -> &[u8] {
    //     &self.data
    // }

    #[inline(always)]
    pub fn frame(&self) -> &Rect {
        &self.frame
    }

    #[inline]
    pub fn with_frame(&self, x: f32, y: f32, width: f32, height: f32) -> Texture {
        let frame = Rect {
            x,
            y,
            width,
            height,
        };

        let mut texture = self.clone();
        texture.frame = frame;
        texture
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.frame.width
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.frame.height
    }

    #[inline(always)]
    pub fn base_width(&self) -> f32 {
        self.width as _
    }

    #[inline(always)]
    pub fn base_height(&self) -> f32 {
        self.height as _
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
