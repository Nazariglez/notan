use crate::render_target::RenderTarget;
use crate::render_texture::RenderTextureDescriptor;
use crate::renderer::Renderer;
use crate::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutId,
    BindGroupLayoutRef, BlendMode, Buffer, BufferDescriptor, BufferUsage, ColorMask, CompareMode,
    CullMode, DepthStencil, Device, DrawFrame, GfxAttributes, GfxConfig, IndexFormat, NotanBuffer,
    Primitive, RenderPipeline, RenderTexture, Sampler, SamplerDescriptor, Stencil, Texture,
    TextureData, TextureDescriptor, TextureFilter, TextureFormat, TextureWrap, VertexLayout,
};
use crate::{NotanDevice, RenderPipelineDescriptor};
use image::EncodableLayout;
use notan_core::window::{NotanWindow, WindowId};
use notan_core::Plugin;

pub struct Gfx {
    pub(crate) raw: Device,
}

impl Plugin for Gfx {}

impl<'b> Gfx
where
    Self: 'b,
{
    pub fn new(attrs: GfxAttributes) -> Result<Self, String> {
        let raw = Device::new(attrs)?;
        Ok(Self { raw })
    }

    pub fn config() -> GfxConfig {
        GfxConfig::default()
    }

    pub fn create_frame(&mut self, window_id: WindowId) -> Result<DrawFrame, String> {
        self.raw.create_frame(window_id)
    }

    pub fn init_surface<W: NotanWindow>(&mut self, win: &W) -> Result<(), String> {
        self.raw.init_surface(win)
    }

    pub fn create_render_pipeline<'a>(&'a mut self, shader: &'a str) -> RenderPipelineBuilder {
        RenderPipelineBuilder::new(self, shader)
    }

    pub fn create_vertex_buffer<'a, D: bytemuck::Pod>(
        &'a mut self,
        data: &'a [D],
    ) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Vertex, data)
    }

    pub fn create_index_buffer<'a, D: bytemuck::Pod>(&'a mut self, data: &'a [D]) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Index, data)
    }

    pub fn create_uniform_buffer<'a, D: bytemuck::Pod>(
        &'a mut self,
        data: &'a [D],
    ) -> BufferBuilder {
        BufferBuilder::new(self, BufferUsage::Uniform, data)
    }

    pub fn create_texture(&mut self) -> TextureBuilder {
        TextureBuilder::new(self)
    }

    pub fn create_render_texture(&mut self) -> RenderTextureBuilder {
        RenderTextureBuilder::new(self)
    }

    pub fn write_buffer<'a>(&'a mut self, buffer: &'a Buffer) -> BufferWriteBuilder {
        BufferWriteBuilder::new(self, buffer)
    }

    pub fn create_sampler(&mut self) -> SamplerBuilder {
        SamplerBuilder::new(self)
    }

    pub fn create_bind_group(&mut self) -> BindGroupBuilder {
        BindGroupBuilder::new(self)
    }

    pub fn resize(&mut self, id: WindowId, width: u32, height: u32) -> Result<(), String> {
        self.raw.resize(id, width, height)
    }

    pub fn size(&self, id: WindowId) -> (u32, u32) {
        self.raw.size(id)
    }

    pub fn render<'a, T>(&mut self, target: T, renderer: &Renderer) -> Result<(), String>
    where
        T: Into<RenderTarget<'a, DrawFrame, RenderTexture>>,
    {
        match target.into() {
            RenderTarget::Frame(frame) => self.raw.render_to_frame(frame, renderer),
            RenderTarget::Texture(texture) => self.raw.render_to_texture(texture, renderer),
        }
    }

    pub fn present(&mut self, frame: DrawFrame) -> Result<(), String> {
        self.raw.present(frame)
    }
}

pub struct RenderPipelineBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: RenderPipelineDescriptor<'a>,
}

impl<'a> RenderPipelineBuilder<'a> {
    fn new(gfx: &'a mut Gfx, shader: &'a str) -> Self {
        let desc = RenderPipelineDescriptor {
            shader,
            ..Default::default()
        };
        Self { desc, gfx }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_vertex_layout(mut self, layout: VertexLayout) -> Self {
        self.desc.vertex_layout.push(layout);
        self
    }

    pub fn with_index_format(mut self, format: IndexFormat) -> Self {
        self.desc.index_format = format;
        self
    }

    pub fn with_primitive(mut self, primitive: Primitive) -> Self {
        self.desc.primitive = primitive;
        self
    }

    pub fn with_bind_group_layout(mut self, layout: BindGroupLayout) -> Self {
        self.desc.bind_group_layout.push(layout);
        self
    }

    pub fn with_blend_mode(mut self, mode: BlendMode) -> Self {
        self.desc.blend_mode = Some(mode);
        self
    }

    pub fn with_cull_mode(mut self, mode: CullMode) -> Self {
        self.desc.cull_mode = Some(mode);
        self
    }

    pub fn with_vertex_entry(mut self, entry: &'a str) -> Self {
        self.desc.vs_entry = Some(entry);
        self
    }

    pub fn with_fragment_entry(mut self, entry: &'a str) -> Self {
        self.desc.fs_entry = Some(entry);
        self
    }

    pub fn with_depth_stencil(mut self, mode: CompareMode, write: bool) -> Self {
        self.desc.depth_stencil = Some(DepthStencil {
            write,
            compare: mode,
        });
        self
    }

    pub fn with_stencil(mut self, opts: Stencil) -> Self {
        self.desc.stencil = Some(opts);
        self
    }

    pub fn with_color_mask(mut self, mask: ColorMask) -> Self {
        self.desc.color_mask = mask;
        self
    }

    pub fn build(self) -> Result<RenderPipeline, String> {
        let Self { desc, gfx } = self;
        gfx.raw.create_render_pipeline(desc)
    }
}

pub struct BufferBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: BufferDescriptor<'a>,
}

impl<'a> BufferBuilder<'a> {
    fn new<D: bytemuck::Pod>(gfx: &'a mut Gfx, usage: BufferUsage, data: &'a [D]) -> Self {
        let desc = BufferDescriptor {
            content: bytemuck::cast_slice(data),
            usage,
            ..Default::default()
        };
        Self { gfx, desc }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_write_flag(mut self, writable: bool) -> Self {
        self.desc.write = writable;
        self
    }

    pub fn build(self) -> Result<Buffer, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_buffer(desc)
    }
}

enum TextureRawData<'a> {
    Empty,
    Image(&'a [u8]),
    Raw {
        bytes: &'a [u8],
        width: u32,
        height: u32,
    },
}

pub struct TextureBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: TextureDescriptor<'a>,
    data: TextureRawData<'a>,
}

impl<'a> TextureBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx) -> Self {
        let desc = TextureDescriptor::default();
        let data = TextureRawData::Empty;
        Self { gfx, desc, data }
    }

    pub fn from_image(mut self, image: &'a [u8]) -> Self {
        self.data = TextureRawData::Image(image);
        self
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.desc.format = format;
        self
    }

    pub fn with_write_flag(mut self, writable: bool) -> Self {
        self.desc.write = writable;
        self
    }

    pub fn build(self) -> Result<Texture, String> {
        let Self { gfx, desc, data } = self;
        match data {
            TextureRawData::Empty => gfx.raw.create_texture(desc, None),
            TextureRawData::Image(bytes) => {
                let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
                let rgba = img.to_rgba8();
                gfx.raw.create_texture(
                    desc,
                    Some(TextureData {
                        bytes: rgba.as_bytes(),
                        width: rgba.width(),
                        height: rgba.height(),
                    }),
                )
            }
            TextureRawData::Raw {
                bytes,
                width,
                height,
            } => gfx.raw.create_texture(
                desc,
                Some(TextureData {
                    bytes,
                    width,
                    height,
                }),
            ),
        }
    }
}

pub struct SamplerBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: SamplerDescriptor<'a>,
}

impl<'a> SamplerBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx) -> Self {
        let desc = SamplerDescriptor::default();
        Self { gfx, desc }
    }

    pub fn with_wrap_x(mut self, wrap: TextureWrap) -> Self {
        self.desc.wrap_x = wrap;
        self
    }

    pub fn with_wrap_y(mut self, wrap: TextureWrap) -> Self {
        self.desc.wrap_y = wrap;
        self
    }

    pub fn with_wrap_z(mut self, wrap: TextureWrap) -> Self {
        self.desc.wrap_z = wrap;
        self
    }

    pub fn with_min_filter(mut self, filter: TextureFilter) -> Self {
        self.desc.min_filter = filter;
        self
    }

    pub fn with_mag_filter(mut self, filter: TextureFilter) -> Self {
        self.desc.mag_filter = filter;
        self
    }

    pub fn with_mipmap_filter(mut self, filter: TextureFilter) -> Self {
        self.desc.mipmap_filter = Some(filter);
        self
    }

    pub fn build(self) -> Result<Sampler, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_sampler(desc)
    }
}

pub struct BindGroupBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: BindGroupDescriptor<'a>,
}

impl<'a> BindGroupBuilder<'a> {
    fn new(gfx: &'a mut Gfx) -> Self {
        let desc = Default::default();
        Self { gfx, desc }
    }

    pub fn with_layout(mut self, layout: &'a BindGroupLayoutRef) -> Self {
        self.desc.layout = Some(layout);
        self
    }

    pub fn with_texture(mut self, location: u32, texture: &'a Texture) -> Self {
        self.desc
            .entry
            .push(BindGroupEntry::Texture { location, texture });
        self
    }

    pub fn with_sampler(mut self, location: u32, sampler: &'a Sampler) -> Self {
        self.desc
            .entry
            .push(BindGroupEntry::Sampler { location, sampler });
        self
    }

    pub fn with_uniform(mut self, location: u32, buffer: &'a Buffer) -> Self {
        self.desc
            .entry
            .push(BindGroupEntry::Uniform { location, buffer });
        self
    }

    pub fn build(self) -> Result<BindGroup, String> {
        let Self { gfx, desc } = self;
        gfx.raw.create_bind_group(desc)
    }
}

pub struct BufferWriteBuilder<'a> {
    gfx: &'a mut Gfx,
    buffer: &'a Buffer,
    offset: u64,
    data: Option<&'a [u8]>,
}

impl<'a> BufferWriteBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx, buffer: &'a Buffer) -> Self {
        Self {
            gfx,
            buffer,
            offset: 0,
            data: None,
        }
    }

    pub fn with_data<D: bytemuck::Pod>(mut self, data: &'a [D]) -> Self {
        self.data = Some(bytemuck::cast_slice(data));
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }

    pub fn build(self) -> Result<(), String> {
        let Self {
            gfx,
            buffer,
            offset,
            data,
        } = self;

        if !buffer.is_writable() {
            return Err("Buffer is not Writable".to_string());
        }

        let data = data.unwrap_or(&[]);
        gfx.raw.write_buffer(buffer, offset, data)
    }
}

pub struct RenderTextureBuilder<'a> {
    gfx: &'a mut Gfx,
    desc: RenderTextureDescriptor<'a>,
}

impl<'a> RenderTextureBuilder<'a> {
    pub fn new(gfx: &'a mut Gfx) -> Self {
        let desc = RenderTextureDescriptor::default();
        Self { gfx, desc }
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.desc.label = Some(label);
        self
    }

    pub fn with_depth(mut self, enabled: bool) -> Self {
        self.desc.depth = enabled;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.desc.width = width;
        self.desc.height = height;
        self
    }

    pub fn build(self) -> Result<RenderTexture, String> {
        let Self { gfx, desc } = self;

        let no_size = self.desc.width == 0 || self.desc.height == 0;
        if no_size {
            return Err(format!(
                "RenderTexture size cannot be zero 'width={}', 'height={}'",
                self.desc.width, self.desc.height
            ));
        }

        gfx.raw.create_render_texture(desc)
    }
}
