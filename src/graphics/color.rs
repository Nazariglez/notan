/// Represents a visual color
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Rgba(f32, f32, f32, f32),
    Rgb(f32, f32, f32),
    Hex(u32),
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl Color {
    //TODO Change rgba values from tuple to array? [f32;4]
    /// Converts the color to f32 values inside a tuple (red, green, blue, alpha)
    pub fn to_rgba(&self) -> (f32, f32, f32, f32) {
        use Color::*;

        match self {
            Rgba(r, g, b, a) => (*r, *g, *b, *a),
            Rgb(r, g, b) => (*r, *g, *b, 1.0),
            Hex(val) => hex_to_rgba(*val),
            Black => (0.0, 0.0, 0.0, 1.0),
            White => (1.0, 1.0, 1.0, 1.0),
            Red => (1.0, 0.0, 0.0, 1.0),
            Green => (0.0, 1.0, 0.0, 1.0),
            Blue => (0.0, 0.0, 1.0, 1.0), //TODO add more colors
        }
    }

    /// Converts the color to a hexadecimal value
    pub fn to_hex(&self) -> u32 {
        match self {
            Color::Hex(val) => *val,
            _ => {
                let (r, g, b, a) = self.to_rgba();
                rgba_to_hex(r, g, b, a)
            }
        }
    }

    /// Converts a hexadecimal color to a string value 0x000000ff -> #000000ff;
    pub fn to_hex_string(&self) -> String {
        match self {
            Color::Hex(val) => hex_to_string(*val),
            _ => {
                let (r, g, b, a) = self.to_rgba();
                hex_to_string(rgba_to_hex(r, g, b, a))
            }
        }
    }

    /// Returns the same color with the alpha passed
    pub fn with_alpha(&self, alpha: f32) -> Color {
        let (r, g, b, _) = self.to_rgba();
        rgba(r, g, b, alpha)
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        hex(color)
    }
}

/// Create a new color from rgba values
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::Rgba(r, g, b, a)
}

/// Create a new color from an hexadecimal value
pub fn hex(hex: u32) -> Color {
    let (r, g, b, a) = hex_to_rgba(hex);
    Color::Rgba(r, g, b, a)
}

/// Converts a rgba color values to a hexadecimal values
pub fn rgba_to_hex(r: f32, g: f32, b: f32, a: f32) -> u32 {
    (((r * 255.0) as u32) << 24)
        + (((g * 255.0) as u32) << 16)
        + (((b * 255.0) as u32) << 8)
        + (((a * 255.0) as u32) | 0)
}

/// Converts an hexadecimal value to a rgba values
pub fn hex_to_rgba(hex: u32) -> (f32, f32, f32, f32) {
    (
        ((hex >> 24) & 0xFF) as f32 / 255.0,
        ((hex >> 16) & 0xFF) as f32 / 255.0,
        ((hex >> 8) & 0xFF) as f32 / 255.0,
        (hex & 0xFF) as f32 / 255.0,
    )
}

/// Converts a hexadecimal value to string
pub fn hex_to_string(hex: u32) -> String {
    format!("{:#X}", hex)
}
