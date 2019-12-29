/// Represents a visual color
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Rgba(f32, f32, f32, f32),
    Rgb(f32, f32, f32),
    Hex(u32),
}

impl Color {
    pub const TRANSPARENT: Color = Color::Rgba(0.0, 0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::Rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::Rgba(0.0, 0.0, 0.0, 1.0);
    pub const RED: Color = Color::Rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::Rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::Rgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::Rgba(1.0, 1.0, 0.0, 1.0);
    pub const FUCHSIA: Color = Color::Rgba(1.0, 0.0, 1.0, 1.0);
    pub const SILVER: Color = Color::Rgba(0.753, 0.753, 0.753, 1.0);
    pub const GRAY: Color = Color::Rgba(0.5, 0.5, 0.5, 1.0);
    pub const OLIVE: Color = Color::Rgba(0.5, 0.5, 0.0, 1.0);
    pub const PURPLE: Color = Color::Rgba(0.5, 0.0, 0.5, 1.0);
    pub const MAROON: Color = Color::Rgba(0.5, 0.0, 0.0, 1.0);
    pub const AQUA: Color = Color::Rgba(0.0, 1.0, 1.0, 1.0);
    pub const TEAL: Color = Color::Rgba(0.0, 0.5, 0.5, 1.0);
    pub const NAVY: Color = Color::Rgba(0.0, 0.0, 0.5, 1.0);
    pub const ORANGE: Color = Color::Rgba(1.0, 0.647, 0.0, 1.0);
    pub const PINK: Color = Color::Rgba(1.0, 0.753, 0.796, 1.0);

    /// Converts the color to f32 values inside an array [red, green, blue, alpha]
    pub fn to_rgba(&self) -> [f32; 4] {
        use Color::*;
        match self {
            Rgba(r, g, b, a) => [*r, *g, *b, *a],
            Rgb(r, g, b) => [*r, *g, *b, 1.0],
            Hex(color) => hex_to_rgba(*color),
        }
    }

    /// Converts the color to a hexadecimal value
    pub fn to_hex(&self) -> u32 {
        match self {
            Color::Hex(val) => *val,
            _ => {
                let [r, g, b, a] = self.to_rgba();
                rgba_to_hex(r, g, b, a)
            }
        }
    }

    /// Converts a hexadecimal color to a rgba string value 0x000000ff -> #000000ff;
    pub fn to_hex_string(&self) -> String {
        match self {
            Color::Hex(val) => hex_to_string(*val),
            _ => {
                let [r, g, b, a] = self.to_rgba();
                hex_to_string(rgba_to_hex(r, g, b, a))
            }
        }
    }

    /// Returns the same color with the red passed
    pub fn with_red(&self, red: f32) -> Color {
        let [_, g, b, a] = self.to_rgba();
        rgba(red, g, b, a)
    }

    /// Returns the same color with the green passed
    pub fn with_green(&self, green: f32) -> Color {
        let [r, _, b, a] = self.to_rgba();
        rgba(r, green, b, a)
    }

    /// Returns the same color with the blue passed
    pub fn with_blue(&self, blue: f32) -> Color {
        let [r, g, _, a] = self.to_rgba();
        rgba(r, g, blue, a)
    }

    /// Returns the same color with the alpha passed
    pub fn with_alpha(&self, alpha: f32) -> Color {
        let [r, g, b, _] = self.to_rgba();
        rgba(r, g, b, alpha)
    }

    /// Returns the red value of this color
    pub fn red(&self) -> f32 {
        use Color::*;
        match self {
            Rgba(r, _, _, _) => *r,
            Rgb(r, _, _) => *r,
            Hex(hex) => ((*hex >> 24) & 0xFF) as f32 / 255.0,
        }
    }

    /// Returns the green value of this color
    pub fn green(&self) -> f32 {
        use Color::*;
        match self {
            Rgba(_, g, _, _) => *g,
            Rgb(_, g, _) => *g,
            Hex(hex) => ((*hex >> 16) & 0xFF) as f32 / 255.0,
        }
    }

    /// Returns the blue value of this color
    pub fn blue(&self) -> f32 {
        use Color::*;
        match self {
            Rgba(_, _, b, _) => *b,
            Rgb(_, _, b) => *b,
            Hex(hex) => ((*hex >> 8) & 0xFF) as f32 / 255.0,
        }
    }

    /// Returns the alpha value of this color
    pub fn alpha(&self) -> f32 {
        use Color::*;
        match self {
            Rgba(_, _, _, a) => *a,
            Rgb(_, _, _) => 1.0,
            Hex(hex) => (*hex & 0xFF) as f32 / 255.0,
        }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        hex(color)
    }
}

impl From<[f32; 4]> for Color {
    fn from(color: [f32; 4]) -> Self {
        rgba(color[0], color[1], color[2], color[3])
    }
}

/// Create a new color from rgba values
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::Rgba(r, g, b, a)
}

/// Create a new color from an hexadecimal value
pub fn hex(hex: u32) -> Color {
    let [r, g, b, a] = hex_to_rgba(hex);
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
pub fn hex_to_rgba(hex: u32) -> [f32; 4] {
    [
        ((hex >> 24) & 0xFF) as f32 / 255.0,
        ((hex >> 16) & 0xFF) as f32 / 255.0,
        ((hex >> 8) & 0xFF) as f32 / 255.0,
        (hex & 0xFF) as f32 / 255.0,
    ]
}

/// Converts a hexadecimal value to string
pub fn hex_to_string(hex: u32) -> String {
    format!("{:#X}", hex)
}
