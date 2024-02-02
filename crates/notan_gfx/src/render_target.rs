use crate::frame::NotanDrawFrame;
use crate::render_texture::NotanRenderTexture;

#[derive(Debug)]
pub enum RenderTarget<'a, DF, RT>
where
    DF: NotanDrawFrame + 'a,
    RT: NotanRenderTexture + 'a,
{
    Frame(&'a DF),
    Texture(&'a RT),
}
