/// Represents a visual color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color([f32; 4]);

impl Color {
    pub const TRANSPARENT: Color = Color([0.0, 0.0, 0.0, 0.0]);
    pub const WHITE: Color = Color([1.0, 1.0, 1.0, 1.0]);
    pub const BLACK: Color = Color([0.0, 0.0, 0.0, 1.0]);
    pub const RED: Color = Color([1.0, 0.0, 0.0, 1.0]);
    pub const GREEN: Color = Color([0.0, 1.0, 0.0, 1.0]);
    pub const BLUE: Color = Color([0.0, 0.0, 1.0, 1.0]);
    pub const YELLOW: Color = Color([1.0, 1.0, 0.0, 1.0]);
    pub const MAGENTA: Color = Color([1.0, 0.0, 1.0, 1.0]);
    pub const SILVER: Color = Color([0.753, 0.753, 0.753, 1.0]);
    pub const GRAY: Color = Color([0.5, 0.5, 0.5, 1.0]);
    pub const OLIVE: Color = Color([0.5, 0.5, 0.0, 1.0]);
    pub const PURPLE: Color = Color([0.5, 0.0, 0.5, 1.0]);
    pub const MAROON: Color = Color([0.5, 0.0, 0.0, 1.0]);
    pub const AQUA: Color = Color([0.0, 1.0, 1.0, 1.0]);
    pub const TEAL: Color = Color([0.0, 0.5, 0.5, 1.0]);
    pub const NAVY: Color = Color([0.0, 0.0, 0.5, 1.0]);
    pub const ORANGE: Color = Color([1.0, 0.647, 0.0, 1.0]);
    pub const PINK: Color = Color([1.0, 0.753, 0.796, 1.0]);

    /// Create a new color from red, green, blue and alpha values
    pub fn new(r:f32, g:f32, b:f32, a:f32) -> Self {
        Self([r, g, b, a])
    }

    /// Create a new color from red, green, blue and alpha values
    pub fn from_rgba(r:f32, g:f32, b:f32, a:f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// Create a new color from red, green and blue values
    pub fn from_rgb(r:f32, g:f32, b:f32) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    /// Create a new color from hexadecimal number like 0x000000ff (0xRRGGBBAA)
    pub fn from_hex(hex: u32) -> Self {
        Self(hex_to_rgba(hex))
    }

    /// Returns the same color with the red passed
    pub fn with_red(&self, red: f32) -> Color {
        Self([red, self.green(), self.blue(), self.alpha()])
    }

    /// Returns the same color with the green passed
    pub fn with_green(&self, green: f32) -> Color {
        Self([self.red(), green, self.blue(), self.alpha()])
    }

    /// Returns the same color with the blue passed
    pub fn with_blue(&self, blue: f32) -> Color {
        Self([self.red(), self.green(), blue, self.alpha()])
    }

    /// Returns the same color with the alpha passed
    pub fn with_alpha(&self, alpha: f32) -> Color {
        Self([self.red(), self.green(), self.blue(), alpha])
    }

    /// Returns the colors as slice
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }

    /// Return the red value
    #[inline]
    pub fn red(&self) -> f32 {
        self.0[0]
    }

    /// Returns the green value
    #[inline]
    pub fn green(&self) -> f32 {
        self.0[1]
    }

    /// Returns the blue value
    #[inline]
    pub fn blue(&self) -> f32 {
        self.0[2]
    }

    /// Returns the alpha value
    #[inline]
    pub fn alpha(&self) -> f32 {
        self.0[3]
    }

    /// Returns an array with the r, g, b, a values
    pub fn to_rgba(&self) -> [f32; 4] {
        self.0
    }

    /// Returns an array with the r, g, b values
    pub fn to_rgb(&self) -> [f32; 3] {
        [self.red(), self.green(), self.blue()]
    }

    /// Returns the hexadecimal representation of the color like 0xRRGGBBAA
    pub fn to_hex(&self) -> u32 {
        rgba_to_hex(self.red(), self.green(), self.blue(), self.alpha())
    }

    /// Returns the hexadecimal representantion of the colos as string like #RRGGBBAA
    pub fn to_hex_string(&self) -> String {
        hex_to_string(self.to_hex())
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        c.0
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Color::from_hex(color)
    }
}

impl From<[f32; 4]> for Color {
    fn from(color: [f32; 4]) -> Self {
        Color::new(color[0], color[1], color[2], color[3])
    }
}

impl From<[f32; 3]> for Color {
    fn from(color: [f32; 3]) -> Self {
        Color::from_rgb(color[0], color[1], color[2])
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color {{ r: {}, g: {}, b: {}, a: {}}}", self.red(), self.green(), self.blue(), self.alpha())
    }
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
