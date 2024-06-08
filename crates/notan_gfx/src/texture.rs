use notan_macro2::ResourceId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ResourceId)]
pub struct TextureId(u64);

pub trait NotanTexture {
    fn id(&self) -> TextureId;
    fn size(&self) -> (u32, u32);

    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub format: TextureFormat,
    pub write: bool,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureFormat {
    #[default]
    Rgba8UnormSrgb,
    Bgra8UnormSrgb,
    // Depth16,
    Depth32Float,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TextureData<'a> {
    pub bytes: &'a [u8],
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureWrap {
    #[default]
    Clamp,
    Repeat,
    MirrorRepeat,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureFilter {
    #[default]
    Linear,
    Nearest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ResourceId)]
pub struct SamplerId(u64);

pub trait NotanSampler {
    fn id(&self) -> SamplerId;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct SamplerDescriptor<'a> {
    pub label: Option<&'a str>,
    pub wrap_x: TextureWrap,
    pub wrap_y: TextureWrap,
    pub wrap_z: TextureWrap,
    pub mag_filter: TextureFilter,
    pub min_filter: TextureFilter,
    pub mipmap_filter: Option<TextureFilter>,
}
