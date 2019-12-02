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
    InverseDestinationAlpha
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlendMode {
    src: BlendFactor,
    dst: BlendFactor,
}

impl BlendMode {
    pub const NONE:BlendMode = BlendMode { src: BlendFactor::One, dst: BlendFactor::Zero };
    pub const NORMAL:BlendMode = BlendMode { src: BlendFactor::SourceAlpha, dst: BlendFactor::InverseSourceAlpha };
    pub const ADD:BlendMode = BlendMode { src: BlendFactor::One, dst: BlendFactor::One };
    pub const MULTIPLY:BlendMode = BlendMode { src: BlendFactor::DestinationColor, dst: BlendFactor::InverseSourceAlpha };
    pub const SCREEN:BlendMode = BlendMode { src: BlendFactor::One, dst: BlendFactor::InverseSourceColor };
    pub const ERASE:BlendMode = BlendMode { src: BlendFactor::Zero, dst: BlendFactor::InverseSourceColor };

    pub fn new(source: BlendFactor, destination: BlendFactor) -> Self {
        Self {
            src: source,
            dst: destination,
        }
    }

    pub fn source(&self) -> BlendFactor {
        self.src
    }

    pub fn destination(&self) -> BlendFactor {
        self.dst
    }
}

impl From<BlendFactor> for u32 {
    fn from(factor: BlendFactor) -> Self {
        use BlendFactor::*;
        match factor {
            One => glow::ONE,
            Zero => glow::ZERO,
            SourceColor => glow::SRC_COLOR,
            InverseSourceColor => glow::ONE_MINUS_SRC_COLOR,
            DestinationColor => glow::DST_COLOR,
            InverseDestinationColor => glow::ONE_MINUS_DST_COLOR,
            SourceAlpha => glow::SRC_ALPHA,
            InverseSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
            DestinationAlpha => glow::DST_ALPHA,
            InverseDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
        }
    }
}