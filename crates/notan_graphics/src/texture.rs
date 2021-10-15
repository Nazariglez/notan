#![allow(clippy::wrong_self_convention)]

use crate::device::{DropManager, ResourceId};
use crate::Device;
use notan_math::Rect;
use std::sync::Arc;

#[derive(Debug)]
pub struct TextureRead {
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: i32,
    pub height: i32,
    pub format: TextureFormat,
}

#[derive(Debug, Clone)]
pub struct TextureUpdate<'a> {
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: i32,
    pub height: i32,
    pub format: TextureFormat,
    pub bytes: &'a [u8],
}

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
            .to_rgba8();

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

    pub fn from_bytes(bytes: &[u8], width: i32, height: i32) -> Result<Self, String> {
        Self::from_bytes_with_options(
            bytes,
            width,
            height,
            TextureFormat::Rgba,
            TextureFormat::Rgba,
            TextureFilter::Nearest,
            TextureFilter::Nearest,
        )
    }

    pub fn from_bytes_with_options(
        bytes: &[u8],
        width: i32,
        height: i32,
        format: TextureFormat,
        internal_format: TextureFormat,
        mag_filter: TextureFilter,
        min_filter: TextureFilter,
    ) -> Result<Self, String> {
        Ok(Self {
            width,
            height,
            bytes: Some(bytes.to_vec()),
            format,
            internal_format,
            mag_filter,
            min_filter,
            depth: false,
        })
    }

    pub fn bytes_per_pixel(&self) -> u8 {
        use TextureFormat::*;
        match (self.format, self.internal_format) {
            (Red, R8) => 1,
            _ => 4, //TODO
        }
    }
}

#[derive(Debug)]
struct TextureId {
    id: u64,
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
    pub(crate) fn new(id: u64, info: TextureInfo, drop_manager: Arc<DropManager>) -> Self {
        let id = Arc::new(TextureId { id, drop_manager });

        let TextureInfo {
            width,
            height,
            format,
            internal_format,
            min_filter,
            mag_filter,
            ..
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
    pub fn id(&self) -> u64 {
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

    pub fn size(&self) -> (f32, f32) {
        (self.frame.width, self.frame.height)
    }

    pub fn base_size(&self) -> (f32, f32) {
        (self.width as _, self.height as _)
    }
}

impl std::cmp::PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.frame() == other.frame()
    }
}

impl AsRef<Texture> for Texture {
    fn as_ref(&self) -> &Texture {
        &self
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba, //rgba32
    Red,  //r32
    R8,
    Depth,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

enum TextureKind<'a> {
    Texture(&'a [u8]),
    Bytes(&'a [u8]),
    EmptyBuffer,
}

pub struct TextureBuilder<'a, 'b> {
    device: &'a mut Device,
    kind: Option<TextureKind<'b>>,
    info: TextureInfo,
}

impl<'a, 'b> TextureBuilder<'a, 'b> {
    pub fn new(device: &'a mut Device) -> Self {
        Self {
            device,
            info: Default::default(),
            kind: None,
        }
    }

    /// Creates a Texture from an image
    pub fn from_image(mut self, bytes: &'b [u8]) -> Self {
        self.kind = Some(TextureKind::Texture(bytes));
        self
    }

    /// Creates a Texture from a buffer of pixels
    pub fn from_bytes(mut self, bytes: &'b [u8], width: i32, height: i32) -> Self {
        self.kind = Some(TextureKind::Bytes(bytes));
        self.info.width = width;
        self.info.height = height;
        self
    }

    /// Creates a buffer for the size passed in and creates a Texture with it
    pub fn from_empty_buffer(mut self, width: i32, height: i32) -> Self {
        self.kind = Some(TextureKind::EmptyBuffer);
        self.with_size(width, height)
    }

    /// Set the size of the texture (ignored if used with `from_image`, image size will be used instead)
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.info.width = width;
        self.info.height = height;
        self
    }

    /// Enable depth
    pub fn with_depth(mut self) -> Self {
        self.info.depth = true;
        self
    }

    /// Set the Texture format (ignored if used with `from_image`, Rgba will be used instead )
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.info.format = format;
        self.info.internal_format = format; // todo check this, worth to allow set the internal_format independently?
        self
    }

    /// Set the Texture filter modes
    pub fn with_filter(mut self, min: TextureFilter, mag: TextureFilter) -> Self {
        self.info.min_filter = min;
        self.info.mag_filter = mag;
        self
    }

    /// Generate the mipmaps
    pub fn generate_mipmap(self) -> Self {
        todo!("generate mipmaps");
    }

    pub fn build(self) -> Result<Texture, String> {
        let TextureBuilder {
            mut info,
            device,
            kind,
        } = self;

        match kind {
            Some(TextureKind::Texture(bytes)) => {
                let data = image::load_from_memory(bytes)
                    .map_err(|e| e.to_string())?
                    .to_rgba8();

                info.bytes = Some(data.to_vec());
                info.format = TextureFormat::Rgba;
                info.internal_format = TextureFormat::Rgba;
                info.width = data.width() as _;
                info.height = data.height() as _;
            }
            Some(TextureKind::Bytes(bytes)) => {
                #[cfg(debug_assertions)]
                {
                    let size = info.width * info.height * (info.bytes_per_pixel() as i32);
                    debug_assert_eq!(bytes.len(), size as usize, "Texture bytes of len {} when it should be {} (width: {} * height: {} * bytes: {})", bytes.len(), size, info.width, info.height, info.bytes_per_pixel());
                }

                info.bytes = Some(bytes.to_vec());
            }
            Some(TextureKind::EmptyBuffer) => {
                let size = info.width * info.height * (info.bytes_per_pixel() as i32);
                info.bytes = Some(vec![0; size as _]);
            }
            _ => {}
        }

        device.create_texture(info)
    }
}

pub struct TextureReader<'a> {
    device: &'a mut Device,
    texture: &'a Texture,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: TextureFormat,
}

impl<'a> TextureReader<'a> {
    pub fn new(device: &'a mut Device, texture: &'a Texture) -> Self {
        let rect = *texture.frame();
        let x_offset = rect.x as i32;
        let y_offset = rect.y as i32;
        let width = rect.width as i32;
        let height = rect.height as i32;
        let format = texture.format;
        Self {
            device,
            texture,
            x_offset,
            y_offset,
            width,
            height,
            format,
        }
    }

    /// Read pixels from the axis x offset
    pub fn with_x_offset(mut self, offset: i32) -> Self {
        self.x_offset = offset;
        self
    }

    /// Read pixels from the axis y offset
    pub fn with_y_offset(mut self, offset: i32) -> Self {
        self.y_offset = offset;
        self
    }

    /// Read pixels until this width from the x offset value
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    /// Read pixels until this height from the y offset value
    pub fn with_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    pub fn read_to(self, bytes: &mut [u8]) -> Result<(), String> {
        let Self {
            device,
            texture,
            x_offset,
            y_offset,
            width,
            height,
            format,
        } = self;

        let info = TextureRead {
            x_offset,
            y_offset,
            width,
            height,
            format,
        };

        device.read_pixels(texture, bytes, &info)
    }
}

pub struct TextureUpdater<'a> {
    device: &'a mut Device,
    texture: &'a mut Texture,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    format: TextureFormat,
    bytes: Option<&'a [u8]>,
}

impl<'a> TextureUpdater<'a> {
    pub fn new(device: &'a mut Device, texture: &'a mut Texture) -> Self {
        let x_offset = texture.frame.x as _;
        let y_offset = texture.frame.y as _;
        let width = texture.frame.width as _;
        let height = texture.frame.height as _;
        let format = texture.format;

        Self {
            device,
            texture,
            x_offset,
            y_offset,
            width,
            height,
            format,
            bytes: None,
        }
    }

    /// Update pixels from the axis x offset
    pub fn with_x_offset(mut self, offset: i32) -> Self {
        self.x_offset = offset;
        self
    }

    /// Update pixels from the axis y offset
    pub fn with_y_offset(mut self, offset: i32) -> Self {
        self.y_offset = offset;
        self
    }

    /// Update pixels until this width from the x offset value
    pub fn with_width(mut self, width: i32) -> Self {
        self.width = width;
        self
    }

    /// Update pixels until this height from the y offset value
    pub fn with_height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }

    pub fn with_data(mut self, bytes: &'a [u8]) -> Self {
        self.bytes = Some(bytes);
        self
    }

    pub fn update(self) -> Result<(), String> {
        let Self {
            device,
            texture,
            x_offset,
            y_offset,
            width,
            height,
            format,
            bytes,
        } = self;

        let bytes =
            bytes.ok_or_else(|| "You need to provide bytes to update a texture".to_string())?;

        let info = TextureUpdate {
            x_offset,
            y_offset,
            width,
            height,
            format,
            bytes,
        };

        device.update_texture(texture, &info)
    }
}
