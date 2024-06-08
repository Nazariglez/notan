#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct BlendMode {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

impl BlendMode {
    /// Replaces the destination pixel with the source pixel
    pub const REPLACE: Self = Self {
        color: BlendComponent::REPLACE,
        alpha: BlendComponent::REPLACE,
    };

    /// Blends the source over the destination, considering the source's alpha
    pub const NORMAL: Self = Self {
        color: BlendComponent {
            src: BlendFactor::SourceAlpha,
            dst: BlendFactor::InverseSourceAlpha,
            op: BlendOperation::Add,
        },
        alpha: BlendComponent::OVER,
    };

    /// Adds the source and destination pixels
    pub const ADD: Self = Self {
        color: BlendComponent {
            src: BlendFactor::SourceAlpha,
            dst: BlendFactor::One,
            op: BlendOperation::Add,
        },
        alpha: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::One,
            op: BlendOperation::Add,
        },
    };

    /// Combines the source and destination pixels to create a lighter image
    pub const SCREEN: Self = Self {
        color: BlendComponent {
            src: BlendFactor::SourceAlpha,
            dst: BlendFactor::InverseSourceColor,
            op: BlendOperation::Add,
        },
        alpha: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::InverseSourceAlpha,
            op: BlendOperation::Add,
        },
    };

    /// Multiplies the top and bottom layer pixels, resulting in a darker image
    pub const MULTIPLY: Self = Self {
        color: BlendComponent {
            src: BlendFactor::DestinationColor,
            dst: BlendFactor::InverseSourceAlpha,
            op: BlendOperation::Add,
        },
        alpha: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::InverseSourceAlpha,
            op: BlendOperation::Add,
        },
    };

    /// Normal mode with premultiplied alpha.
    pub const NORMAL_PM: Self = Self {
        color: BlendComponent::OVER,
        alpha: BlendComponent::OVER,
    };

    /// Add mode with premultiplied alpha.
    pub const ADD_PM: Self = Self {
        color: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::One,
            op: BlendOperation::Add,
        },
        alpha: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::One,
            op: BlendOperation::Add,
        },
    };

    /// Screen mode with premultiplied alpha.
    pub const SCREEN_PM: Self = Self {
        color: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::InverseSourceColor,
            op: BlendOperation::Add,
        },
        alpha: BlendComponent {
            src: BlendFactor::One,
            dst: BlendFactor::InverseSourceAlpha,
            op: BlendOperation::Add,
        },
    };
}

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

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlendOperation {
    #[default]
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct BlendComponent {
    pub src: BlendFactor,
    pub dst: BlendFactor,
    pub op: BlendOperation,
}

impl BlendComponent {
    pub const NONE: Self = Self {
        src: BlendFactor::Zero,
        dst: BlendFactor::Zero,
        op: BlendOperation::Add,
    };

    pub const REPLACE: Self = Self {
        src: BlendFactor::One,
        dst: BlendFactor::Zero,
        op: BlendOperation::Add,
    };

    pub const OVER: Self = Self {
        src: BlendFactor::One,
        dst: BlendFactor::InverseSourceAlpha,
        op: BlendOperation::Add,
    };
}

impl Default for BlendComponent {
    fn default() -> Self {
        Self::REPLACE
    }
}
