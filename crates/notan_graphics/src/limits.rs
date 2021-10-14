// check this https://docs.rs/wgpu/0.8.1/wgpu/struct.Limits.html

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_texture_size: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_texture_size: 8192,
        }
    }
}
