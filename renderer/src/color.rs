//! Color type.

use std::{
    fmt,
    str::{self, FromStr},
};

/// Blue color.
pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 0.7 };
/// Red color.
pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 0.7 };
/// Green color.
pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 0.7 };
/// Orange color.
pub const ORANGE: Color = Color { r: 1.0, g: 0.647, b: 0.0, a: 0.7 };
/// Purple color.
pub const PURPLE: Color = Color { r: 0.502, g: 0.0, b: 0.502, a: 0.7 };
/// Yellow color.
pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 0.7 };
/// Brown color.
pub const BROWN: Color = Color { r: 0.647, g: 0.165, b: 0.165, a: 0.7 };
/// Gray color.
pub const GRAY: Color = Color { r: 0.502, g: 0.502, b: 0.502, a: 0.7 };
/// Black color.
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.7 };
/// White color.
pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.7 };
/// Transparent color.
pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

/// Color.
#[derive(Debug, PartialEq)]
pub struct Color {
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
    /// Alfa.
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

impl FromStr for Color {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Color format: #RRGGBB or #RRGGBBAA
        // Example: #ff0000 - red

        // Parse 2 chars as Hex byte and map to 0..1 range
        let char_to_color = |h: char, l: char| -> Result<f32, fmt::Error> {
            let Some(h) = h.to_digit(16) else {
                return Err(fmt::Error);
            };
            let Some(l) = l.to_digit(16) else {
                return Err(fmt::Error);
            };
            let Ok(color) = u8::try_from(h << 4 | l) else {
                return Err(fmt::Error);
            };
            Ok(f32::from(color) / 255.0)
        };
        match *s.chars().collect::<Vec<_>>().as_slice() {
            ['#', rh, rl, gh, gl, bh, bl, ah, al] => Ok(Self {
                r: char_to_color(rh, rl)?,
                g: char_to_color(gh, gl)?,
                b: char_to_color(bh, bl)?,
                a: char_to_color(ah, al)?,
            }),
            ['#', rh, rl, gh, gl, bh, bl] => Ok(Self {
                r: char_to_color(rh, rl)?,
                g: char_to_color(gh, gl)?,
                b: char_to_color(bh, bl)?,
                a: 1.0,
            }),
            _ => Err(fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use std::str::FromStr;

    #[test]
    fn parse_colors() {
        assert_eq!(Color::from_str("#000000").unwrap(), Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 });
        assert_eq!(Color::from_str("#000000FF").unwrap(), Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 });
        assert_eq!(Color::from_str("#00000000").unwrap(), Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
        assert_eq!(Color::from_str("#FF0000").unwrap(), Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
        assert_eq!(Color::from_str("#00FF00").unwrap(), Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
        assert_eq!(Color::from_str("#0000FF").unwrap(), Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
    }
}
