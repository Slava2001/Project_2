//! Color type

/// Blue color
pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 0.7 };
/// Red color
pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 0.7 };
/// Green color
pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 0.7 };
/// Orange color
pub const ORANGE: Color = Color { r: 1.0, g: 0.647, b: 0.0, a: 0.7 };
/// Purple color
pub const PURPLE: Color = Color { r: 0.502, g: 0.0, b: 0.502, a: 0.7 };
/// Yellow color
pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 0.7 };
/// Brown color
pub const BROWN: Color = Color { r: 0.647, g: 0.165, b: 0.165, a: 0.7 };
/// Gray color
pub const GRAY: Color = Color { r: 0.502, g: 0.502, b: 0.502, a: 0.7 };
/// Black color
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.7 };
/// White color
pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.7 };
/// Transparent color
pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

/// Color
pub struct Color {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
    /// Alfa
    pub a: f32,
}

impl<T: Copy + Into<f32>> From<[T; 4]> for Color {
    fn from(v: [T; 4]) -> Self {
        Self { r: v[0].into(), g: v[1].into(), b: v[2].into(), a: v[3].into() }
    }
}

impl<T: Copy + From<f32>> From<&Color> for [T; 4] {
    fn from(val: &Color) -> Self {
        [val.r.into(), val.g.into(), val.b.into(), val.a.into()]
    }
}

impl<T: Copy + Into<f32>> From<[T; 3]> for Color {
    fn from(v: [T; 3]) -> Self {
        Self { r: v[0].into(), g: v[1].into(), b: v[2].into(), a: 1.0 }
    }
}
