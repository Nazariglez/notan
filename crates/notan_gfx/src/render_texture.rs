use crate::{Texture, TextureFormat, TextureId};
use notan_macro2::ResourceId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ResourceId)]
pub struct RenderTextureId(u64);

pub trait NotanRenderTexture {
    fn id(&self) -> RenderTextureId;
    fn texture(&self) -> &Texture;
    fn into_inner(self) -> Texture;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RenderTextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub depth: bool,
    pub width: u32,
    pub height: u32,
}
