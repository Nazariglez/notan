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
