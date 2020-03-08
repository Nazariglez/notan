use crate::Color;

/// BLending factor computed
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

/// Represents depth comparison
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DepthStencil {
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

/// Options to use with the render pipeline
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PipelineOptions {
    pub color_blend: Option<BlendMode>,
    pub alpha_blend: Option<BlendMode>,
    pub cull_mode: CullMode,
    pub depth_stencil: DepthStencil,
}

impl Default for PipelineOptions {
    fn default() -> Self {
        Self {
            depth_stencil: DepthStencil::None,
            cull_mode: CullMode::None,
            color_blend: None,
            alpha_blend: None,
        }
    }
}

/// Clear options to use at the beginning of the frame
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClearOptions {
    pub color: Option<Color>,
    pub depth: Option<f32>,
    pub stencil: Option<i32>,
}

impl ClearOptions {
    /// Create a new struct just with color
    pub fn new(color: Color) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
}

/// Represents the draw usage
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawUsage {
    Static,
    Dynamic,
}
