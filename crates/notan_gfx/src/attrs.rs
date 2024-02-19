use crate::TextureFormat;

/// Configuration to use with the GFX system
#[derive(Debug, Copy, Clone)]
pub struct GfxAttributes {
    /// Use VSync mode if possible
    pub vsync: bool,
    /// This format will be used to create depth textures
    pub depth_format: TextureFormat,
    // TODO wgpu backends?
}

impl Default for GfxAttributes {
    fn default() -> Self {
        Self {
            vsync: false,
            depth_format: TextureFormat::Depth32Float,
        }
    }
}
