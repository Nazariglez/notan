// check this https://docs.rs/wgpu/0.8.1/wgpu/struct.Limits.html

/// Limit are overrided by the graphic implementation
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_texture_size: u32,
    pub max_uniform_blocks: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_texture_size: 8192,
            max_uniform_blocks: 8,
        }
    }
}
