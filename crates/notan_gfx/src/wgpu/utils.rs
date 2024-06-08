use crate::color::Color;
use crate::consts::SURFACE_DEFAULT_DEPTH_FORMAT;
use crate::{
    BlendComponent, BlendFactor, BlendMode, BlendOperation, BufferUsage, ColorMask, CompareMode,
    CullMode, DepthStencil, IndexFormat, Primitive, Stencil, StencilAction, TextureFilter,
    TextureFormat, TextureWrap, VertexFormat, VertexStepMode,
};
use wgpu::{BufferUsages, ColorWrites, CompareFunction};

pub fn wgpu_color(color: Color) -> wgpu::Color {
    wgpu::Color {
        r: color.r as f64,
        g: color.g as f64,
        b: color.b as f64,
        a: color.a as f64,
    }
}

pub fn wgpu_buffer_usages(usage: BufferUsage) -> wgpu::BufferUsages {
    match usage {
        BufferUsage::Vertex => BufferUsages::VERTEX,
        BufferUsage::Index => BufferUsages::INDEX,
        BufferUsage::Uniform => BufferUsages::UNIFORM,
    }
}

pub fn wgpu_vertex_format(format: VertexFormat) -> wgpu::VertexFormat {
    match format {
        VertexFormat::UInt8x2 => wgpu::VertexFormat::Uint8x2,
        VertexFormat::UInt8x4 => wgpu::VertexFormat::Uint8x4,
        VertexFormat::Int8x2 => wgpu::VertexFormat::Sint8x2,
        VertexFormat::Int8x4 => wgpu::VertexFormat::Sint8x4,
        VertexFormat::U8x2norm => wgpu::VertexFormat::Unorm8x2,
        VertexFormat::U8x4norm => wgpu::VertexFormat::Unorm8x4,
        VertexFormat::I8x2norm => wgpu::VertexFormat::Snorm8x2,
        VertexFormat::I8x4norm => wgpu::VertexFormat::Snorm8x4,
        VertexFormat::UInt16x2 => wgpu::VertexFormat::Uint16x2,
        VertexFormat::UInt16x4 => wgpu::VertexFormat::Uint16x4,
        VertexFormat::Int16x2 => wgpu::VertexFormat::Sint16x2,
        VertexFormat::Int16x4 => wgpu::VertexFormat::Sint16x4,
        VertexFormat::U16x2norm => wgpu::VertexFormat::Unorm16x2,
        VertexFormat::U16x4norm => wgpu::VertexFormat::Unorm16x4,
        VertexFormat::Int16x2norm => wgpu::VertexFormat::Snorm16x2,
        VertexFormat::Int16x4norm => wgpu::VertexFormat::Snorm16x4,
        VertexFormat::Float16x2 => wgpu::VertexFormat::Float16x2,
        VertexFormat::Float16x4 => wgpu::VertexFormat::Float16x4,
        VertexFormat::Float32 => wgpu::VertexFormat::Float32,
        VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
        VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
        VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
        VertexFormat::UInt32 => wgpu::VertexFormat::Uint32,
        VertexFormat::UInt32x2 => wgpu::VertexFormat::Uint32x2,
        VertexFormat::UInt32x3 => wgpu::VertexFormat::Uint32x3,
        VertexFormat::UInt32x4 => wgpu::VertexFormat::Uint32x4,
        VertexFormat::Int32 => wgpu::VertexFormat::Sint32,
        VertexFormat::Int32x2 => wgpu::VertexFormat::Sint32x2,
        VertexFormat::Int32x3 => wgpu::VertexFormat::Sint32x3,
        VertexFormat::Int32x4 => wgpu::VertexFormat::Sint32x4,
    }
}

pub fn wgpu_step_mode(step_mode: VertexStepMode) -> wgpu::VertexStepMode {
    match step_mode {
        VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
        VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
    }
}

pub fn wgpu_primitive(primitive: Primitive) -> wgpu::PrimitiveTopology {
    match primitive {
        Primitive::Points => wgpu::PrimitiveTopology::PointList,
        Primitive::Lines => wgpu::PrimitiveTopology::LineList,
        Primitive::LineStrip => wgpu::PrimitiveTopology::LineStrip,
        Primitive::Triangles => wgpu::PrimitiveTopology::TriangleList,
        Primitive::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
    }
}

pub fn wgpu_texture_format(format: TextureFormat) -> wgpu::TextureFormat {
    match format {
        TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
        TextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
        // TextureFormat::Depth16 => wgpu::TextureFormat::Depth16Unorm,
        TextureFormat::Depth32Float => wgpu::TextureFormat::Depth24PlusStencil8,
    }
}

pub fn wgpu_index_format(format: IndexFormat) -> wgpu::IndexFormat {
    match format {
        IndexFormat::UInt16 => wgpu::IndexFormat::Uint16,
        IndexFormat::UInt32 => wgpu::IndexFormat::Uint32,
    }
}

pub fn wgpu_texture_wrap(wrap: TextureWrap) -> wgpu::AddressMode {
    match wrap {
        TextureWrap::Clamp => wgpu::AddressMode::ClampToEdge,
        TextureWrap::Repeat => wgpu::AddressMode::Repeat,
        TextureWrap::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
    }
}

pub fn wgpu_texture_filter(filter: TextureFilter) -> wgpu::FilterMode {
    match filter {
        TextureFilter::Linear => wgpu::FilterMode::Linear,
        TextureFilter::Nearest => wgpu::FilterMode::Nearest,
    }
}

pub fn wgpu_shader_visibility(vertex: bool, fragment: bool, compute: bool) -> wgpu::ShaderStages {
    let mut v = wgpu::ShaderStages::NONE;
    if vertex {
        v |= wgpu::ShaderStages::VERTEX;
    }

    if fragment {
        v |= wgpu::ShaderStages::FRAGMENT;
    }

    if compute {
        v |= wgpu::ShaderStages::COMPUTE;
    }

    v
}

pub fn wgpu_blend_mode(mode: BlendMode) -> wgpu::BlendState {
    wgpu::BlendState {
        color: wgpu_blend_component(mode.color),
        alpha: wgpu_blend_component(mode.alpha),
    }
}

fn wgpu_blend_component(component: BlendComponent) -> wgpu::BlendComponent {
    wgpu::BlendComponent {
        src_factor: wgpu_blend_factor(component.src),
        dst_factor: wgpu_blend_factor(component.dst),
        operation: wgpu_blend_operation(component.op),
    }
}

fn wgpu_blend_factor(factor: BlendFactor) -> wgpu::BlendFactor {
    match factor {
        BlendFactor::Zero => wgpu::BlendFactor::Zero,
        BlendFactor::One => wgpu::BlendFactor::One,
        BlendFactor::SourceColor => wgpu::BlendFactor::Src,
        BlendFactor::InverseSourceColor => wgpu::BlendFactor::OneMinusSrc,
        BlendFactor::DestinationColor => wgpu::BlendFactor::Dst,
        BlendFactor::InverseDestinationColor => wgpu::BlendFactor::OneMinusDst,
        BlendFactor::SourceAlpha => wgpu::BlendFactor::SrcAlpha,
        BlendFactor::InverseSourceAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
        BlendFactor::DestinationAlpha => wgpu::BlendFactor::DstAlpha,
        BlendFactor::InverseDestinationAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
    }
}

fn wgpu_blend_operation(operation: BlendOperation) -> wgpu::BlendOperation {
    match operation {
        BlendOperation::Add => wgpu::BlendOperation::Add,
        BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
        BlendOperation::ReverseSubtract => wgpu::BlendOperation::ReverseSubtract,
        BlendOperation::Min => wgpu::BlendOperation::Min,
        BlendOperation::Max => wgpu::BlendOperation::Max,
    }
}

pub fn wgpu_cull_mode(mode: CullMode) -> wgpu::Face {
    match mode {
        CullMode::Front => wgpu::Face::Front,
        CullMode::Back => wgpu::Face::Back,
    }
}

pub fn wgpu_compare_mode(mode: CompareMode) -> wgpu::CompareFunction {
    match mode {
        CompareMode::Never => wgpu::CompareFunction::Never,
        CompareMode::Less => wgpu::CompareFunction::Less,
        CompareMode::Equal => wgpu::CompareFunction::Equal,
        CompareMode::LEqual => wgpu::CompareFunction::LessEqual,
        CompareMode::Greater => wgpu::CompareFunction::Greater,
        CompareMode::NotEqual => wgpu::CompareFunction::NotEqual,
        CompareMode::GEqual => wgpu::CompareFunction::GreaterEqual,
        CompareMode::Always => wgpu::CompareFunction::Always,
    }
}

pub fn wgpu_depth_stencil(
    depth: Option<DepthStencil>,
    stencil: Option<Stencil>,
) -> Option<wgpu::DepthStencilState> {
    if depth.is_none() && stencil.is_none() {
        return None;
    }

    let (depth_write_enabled, depth_compare) = match depth {
        None => (false, CompareFunction::Always),
        Some(depth) => (depth.write, wgpu_compare_mode(depth.compare)),
    };

    Some(wgpu::DepthStencilState {
        format: wgpu_texture_format(SURFACE_DEFAULT_DEPTH_FORMAT),
        depth_write_enabled,
        depth_compare,
        stencil: stencil.map_or(Default::default(), |stencil| {
            let stencil_face = wgpu::StencilFaceState {
                compare: wgpu_compare_mode(stencil.compare),
                fail_op: wgpu_stencil_operation(stencil.stencil_fail),
                depth_fail_op: wgpu_stencil_operation(stencil.depth_fail),
                pass_op: wgpu_stencil_operation(stencil.pass),
            };

            wgpu::StencilState {
                front: stencil_face,
                back: stencil_face,
                read_mask: stencil.read_mask,
                write_mask: stencil.write_mask,
            }
        }),
        bias: Default::default(),
    })
}

fn wgpu_stencil_operation(action: StencilAction) -> wgpu::StencilOperation {
    match action {
        StencilAction::Keep => wgpu::StencilOperation::Keep,
        StencilAction::Zero => wgpu::StencilOperation::Zero,
        StencilAction::Replace => wgpu::StencilOperation::Replace,
        StencilAction::Increment => wgpu::StencilOperation::IncrementClamp,
        StencilAction::IncrementWrap => wgpu::StencilOperation::IncrementWrap,
        StencilAction::Decrement => wgpu::StencilOperation::DecrementClamp,
        StencilAction::DecrementWrap => wgpu::StencilOperation::DecrementWrap,
        StencilAction::Invert => wgpu::StencilOperation::Invert,
    }
}

pub fn wgpu_write_mask(mask: ColorMask) -> wgpu::ColorWrites {
    let mut raw_mask = ColorWrites::empty();
    if mask.r {
        raw_mask |= ColorWrites::RED;
    }

    if mask.g {
        raw_mask |= ColorWrites::GREEN;
    }

    if mask.b {
        raw_mask |= ColorWrites::BLUE;
    }

    if mask.a {
        raw_mask |= ColorWrites::ALPHA;
    }

    raw_mask
}
