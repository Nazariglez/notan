use notan_graphics::prelude::*;

//TODO use Into<u32> and Into<Option<u32>>?
pub trait ToGlow {
    fn to_glow(&self) -> u32;
}

pub trait ToOptionalGlow {
    fn to_glow(&self) -> Option<u32>;
}

impl ToGlow for StencilAction {
    fn to_glow(&self) -> u32 {
        use StencilAction::*;
        match self {
            Keep => glow::KEEP,
            Zero => glow::ZERO,
            Replace => glow::REPLACE,
            Increment => glow::INCR,
            IncrementWrap => glow::INCR_WRAP,
            Decrement => glow::DECR,
            DecrementWrap => glow::DECR_WRAP,
            Invert => glow::INVERT,
        }
    }
}

impl ToGlow for BlendOperation {
    fn to_glow(&self) -> u32 {
        use BlendOperation::*;
        match self {
            Add => glow::FUNC_ADD,
            Subtract => glow::FUNC_SUBTRACT,
            ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            Max => glow::MAX,
            Min => glow::MIN,
        }
    }
}

impl ToGlow for BlendFactor {
    fn to_glow(&self) -> u32 {
        use BlendFactor::*;
        match self {
            Zero => glow::ZERO,
            One => glow::ONE,
            SourceAlpha => glow::SRC_ALPHA,
            SourceColor => glow::SRC_COLOR,
            InverseSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
            InverseSourceColor => glow::ONE_MINUS_SRC_COLOR,
            DestinationAlpha => glow::DST_ALPHA,
            DestinationColor => glow::SRC_COLOR,
            InverseDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
            InverseDestinationColor => glow::ONE_MINUS_DST_COLOR,
        }
    }
}

impl ToOptionalGlow for CompareMode {
    fn to_glow(&self) -> Option<u32> {
        use CompareMode::*;
        Some(match self {
            None => return Option::None,
            Less => glow::LESS,
            Equal => glow::EQUAL,
            LEqual => glow::LEQUAL,
            Greater => glow::GREATER,
            NotEqual => glow::NOTEQUAL,
            GEqual => glow::GEQUAL,
            Always => glow::ALWAYS,
        })
    }
}

impl ToOptionalGlow for CullMode {
    fn to_glow(&self) -> Option<u32> {
        use CullMode::*;
        Some(match self {
            None => return Option::None,
            Front => glow::FRONT,
            Back => glow::BACK,
        })
    }
}

impl ToGlow for DrawType {
    fn to_glow(&self) -> u32 {
        match self {
            DrawType::Static => glow::STATIC_DRAW,
            DrawType::Dynamic => glow::DYNAMIC_DRAW,
        }
    }
}

impl ToGlow for BufferUsage {
    fn to_glow(&self) -> u32 {
        match self {
            BufferUsage::Vertex => glow::ARRAY_BUFFER,
            BufferUsage::Index => glow::ELEMENT_ARRAY_BUFFER,
            BufferUsage::Uniform(_) => glow::UNIFORM_BUFFER,
        }
    }
}

impl ToGlow for VertexFormat {
    fn to_glow(&self) -> u32 {
        use VertexFormat::*;
        match &self {
            UInt8 | UInt8x2 | UInt8x3 | UInt8x4 => glow::UNSIGNED_BYTE,
            UInt8Norm | UInt8x2Norm | UInt8x3Norm | UInt8x4Norm => glow::UNSIGNED_BYTE,
            Float32 | Float32x2 | Float32x3 | Float32x4 => glow::FLOAT,
        }
    }
}

impl ToGlow for TextureWrap {
    fn to_glow(&self) -> u32 {
        use TextureWrap::*;
        match self {
            Clamp => glow::CLAMP_TO_EDGE,
            Repeat => glow::REPEAT,
        }
    }
}

impl ToGlow for TextureFilter {
    fn to_glow(&self) -> u32 {
        use TextureFilter::*;
        match self {
            Linear => glow::LINEAR,
            Nearest => glow::NEAREST,
        }
    }
}

impl ToGlow for DrawPrimitive {
    fn to_glow(&self) -> u32 {
        match self {
            DrawPrimitive::Triangles => glow::TRIANGLES,
            DrawPrimitive::TriangleStrip => glow::TRIANGLE_STRIP,
            DrawPrimitive::Lines => glow::LINES,
            DrawPrimitive::LineStrip => glow::LINE_STRIP,
        }
    }
}
