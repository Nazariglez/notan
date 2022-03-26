use crate::buffer::{VertexAttr, VertexInfo};
use crate::color::Color;
use crate::device::{DropManager, ResourceId};
use crate::{Device, ShaderSource};
use std::sync::Arc;

#[derive(Debug)]
struct PipelineIdRef {
    id: u64,
    drop_manager: Arc<DropManager>,
}

impl Drop for PipelineIdRef {
    fn drop(&mut self) {
        self.drop_manager.push(ResourceId::Pipeline(self.id));
    }
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    id: u64,
    _id_ref: Arc<PipelineIdRef>,
    stride: usize,
    pub options: PipelineOptions,
}

impl std::cmp::PartialEq for Pipeline {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.options == other.options
    }
}

impl Pipeline {
    pub(crate) fn new(
        id: u64,
        stride: usize,
        options: PipelineOptions,
        drop_manager: Arc<DropManager>,
    ) -> Self {
        let id_ref = Arc::new(PipelineIdRef { id, drop_manager });

        Self {
            id,
            _id_ref: id_ref,
            stride,
            options,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> u64 {
        self.id
    }

    #[inline(always)]
    pub fn stride(&self) -> usize {
        self.stride
    }

    #[inline(always)]
    pub fn offset(&self) -> usize {
        self.stride / 4
    }
}

enum ShaderKind<'b> {
    Raw {
        vertex: &'b [u8],
        fragment: &'b [u8],
    },

    Source {
        vertex: &'b ShaderSource<'b>,
        fragment: &'b ShaderSource<'b>,
    },
}

/// Pipeline builder pattern
pub struct PipelineBuilder<'a, 'b> {
    device: &'a mut Device,
    attrs: Vec<VertexAttr>,
    options: PipelineOptions,
    shaders: Option<ShaderKind<'b>>,
}

impl<'a, 'b> PipelineBuilder<'a, 'b> {
    pub fn new(device: &'a mut Device) -> Self {
        Self {
            device,
            attrs: vec![],
            options: Default::default(),
            shaders: None,
        }
    }

    /// Set the shaders from a ShaderSource object
    pub fn from(mut self, vertex: &'b ShaderSource, fragment: &'b ShaderSource) -> Self {
        self.shaders = Some(ShaderKind::Source { vertex, fragment });
        self
    }

    /// Set the shaders from a bytes slice
    #[allow(clippy::wrong_self_convention)]
    pub fn from_raw(mut self, vertex: &'b [u8], fragment: &'b [u8]) -> Self {
        self.shaders = Some(ShaderKind::Raw { vertex, fragment });
        self
    }

    /// Set the vertex structure info for a vertex buffer
    pub fn with_vertex_info(mut self, info: &VertexInfo) -> Self {
        self.attrs.extend(&info.attrs);
        self
    }

    /// Set the Color blending mode
    pub fn with_color_blend(mut self, color_blend: BlendMode) -> Self {
        self.options.color_blend = Some(color_blend);
        self
    }

    /// Set the alpha blending mode
    pub fn with_alpha_blend(mut self, alpha_blend: BlendMode) -> Self {
        self.options.alpha_blend = Some(alpha_blend);
        self
    }

    /// Set the Culling mode
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.options.cull_mode = cull_mode;
        self
    }

    /// Set the Depth Stencil options
    pub fn with_depth_stencil(mut self, depth_stencil: DepthStencil) -> Self {
        self.options.depth_stencil = depth_stencil;
        self
    }

    /// Set the Color Mask options
    pub fn with_color_mask(mut self, color_mask: ColorMask) -> Self {
        self.options.color_mask = color_mask;
        self
    }

    /// Set the Stencil options
    pub fn with_stencil(mut self, stencil: StencilOptions) -> Self {
        self.options.stencil = Some(stencil);
        self
    }

    /// Build the pipeline with the data set on the builder
    pub fn build(self) -> Result<Pipeline, String> {
        match self.shaders {
            Some(ShaderKind::Source { vertex, fragment }) => {
                self.device
                    .inner_create_pipeline(vertex, fragment, &self.attrs, self.options)
            }
            Some(ShaderKind::Raw { vertex, fragment }) => self
                .device
                .inner_create_pipeline_from_raw(vertex, fragment, &self.attrs, self.options),
            _ => Err("Vertex and Fragment shaders should be present".to_string()),
        }
    }
}

/// Blending factor computed
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlendFactor {
    Zero,
    One,
    SourceColor,
    InverseSourceColor,
    DestinationColor,
    InverseDestinationColor,
    SourceAlpha,
    InverseSourceAlpha,
    DestinationAlpha,
    InverseDestinationAlpha,
}

/// Blending equation used to combine source and destiny
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

/// Blending mode used to draw
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct BlendMode {
    pub src: BlendFactor,
    pub dst: BlendFactor,
    pub op: BlendOperation,
}

impl BlendMode {
    pub const NONE: BlendMode = BlendMode {
        src: BlendFactor::One,
        dst: BlendFactor::Zero,
        op: BlendOperation::Add,
    };
    pub const NORMAL: BlendMode = BlendMode {
        src: BlendFactor::SourceAlpha,
        dst: BlendFactor::InverseSourceAlpha,
        op: BlendOperation::Add,
    };
    pub const ADD: BlendMode = BlendMode {
        src: BlendFactor::One,
        dst: BlendFactor::One,
        op: BlendOperation::Add,
    };
    pub const MULTIPLY: BlendMode = BlendMode {
        src: BlendFactor::DestinationColor,
        dst: BlendFactor::InverseSourceAlpha,
        op: BlendOperation::Add,
    };
    pub const SCREEN: BlendMode = BlendMode {
        src: BlendFactor::One,
        dst: BlendFactor::InverseSourceColor,
        op: BlendOperation::Add,
    };
    pub const ERASE: BlendMode = BlendMode {
        src: BlendFactor::Zero,
        dst: BlendFactor::InverseSourceColor,
        op: BlendOperation::Add,
    };
    pub const OVER: BlendMode = BlendMode {
        src: BlendFactor::One,
        dst: BlendFactor::InverseSourceAlpha,
        op: BlendOperation::Add,
    };

    /// Creates a new blend mode using the ADD operation
    pub fn new(source: BlendFactor, destination: BlendFactor) -> Self {
        Self::with_operation(source, destination, BlendOperation::Add)
    }

    /// Creates a new blend mode
    pub fn with_operation(
        source: BlendFactor,
        destination: BlendFactor,
        operation: BlendOperation,
    ) -> Self {
        Self {
            src: source,
            dst: destination,
            op: operation,
        }
    }
}

/// Represents stencil and depth comparison
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CompareMode {
    None,
    Less,
    Equal,
    LEqual,
    Greater,
    NotEqual,
    GEqual,
    Always,
}

/// Represents face culling modes
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

/// Represents the color mask
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ColorMask {
    pub r: bool,
    pub g: bool,
    pub b: bool,
    pub a: bool,
}

impl Default for ColorMask {
    fn default() -> Self {
        Self {
            r: true,
            g: true,
            b: true,
            a: true,
        }
    }
}

impl ColorMask {
    pub const ALL: ColorMask = ColorMask {
        r: true,
        g: true,
        b: true,
        a: true,
    };

    pub const NONE: ColorMask = ColorMask {
        r: false,
        g: false,
        b: false,
        a: false,
    };
}

/// Represents the color mask
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DepthStencil {
    pub write: bool,
    pub compare: CompareMode,
}

impl Default for DepthStencil {
    fn default() -> Self {
        Self {
            write: true,
            compare: CompareMode::None, //Less?
        }
    }
}

/// Options to use with the render pipeline
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PipelineOptions {
    pub color_blend: Option<BlendMode>,
    pub alpha_blend: Option<BlendMode>,
    pub cull_mode: CullMode,
    pub depth_stencil: DepthStencil,
    pub color_mask: ColorMask,
    pub stencil: Option<StencilOptions>,
}

impl Default for PipelineOptions {
    fn default() -> Self {
        Self {
            depth_stencil: Default::default(),
            cull_mode: CullMode::None,
            color_blend: None,
            alpha_blend: None,
            color_mask: Default::default(),
            stencil: None,
        }
    }
}

/// Clear options to use at the beginning of the frame
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ClearOptions {
    pub color: Option<Color>,
    pub depth: Option<f32>,
    pub stencil: Option<i32>,
}

impl ClearOptions {
    /// Create a new struct just with color
    pub fn color(color: Color) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }

    pub fn none() -> Self {
        Self::default()
    }
}

/// Represents the draw usage
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawType {
    Static,
    Dynamic,
}

/// Represent's the stencil action
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StencilAction {
    Keep,
    Zero,
    Replace,
    Increment,
    IncrementWrap,
    Decrement,
    DecrementWrap,
    Invert,
}

/// Represents the stencil's option
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct StencilOptions {
    pub stencil_fail: StencilAction,
    pub depth_fail: StencilAction,
    pub pass: StencilAction,
    pub compare: CompareMode,
    pub read_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

impl Default for StencilOptions {
    fn default() -> Self {
        Self {
            stencil_fail: StencilAction::Keep,
            depth_fail: StencilAction::Keep,
            pass: StencilAction::Keep,
            compare: CompareMode::Always,
            read_mask: 0xff,
            write_mask: 0,
            reference: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawPrimitive {
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

impl Default for DrawPrimitive {
    fn default() -> Self {
        DrawPrimitive::Triangles
    }
}
