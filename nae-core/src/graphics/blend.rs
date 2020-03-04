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


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max
}

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

    pub fn new(source: BlendFactor, destination: BlendFactor) -> Self {
        Self::with_operation(source, destination, BlendOperation::Add)
    }

    pub fn with_operation(source: BlendFactor, destination: BlendFactor, operation: BlendOperation) -> Self {
        Self {
            src: source,
            dst: destination,
            op: operation,
        }
    }
}

