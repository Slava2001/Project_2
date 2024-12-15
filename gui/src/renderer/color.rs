pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 0.7 };
pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 0.7 };
pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 0.7 };
pub const ORANGE: Color = Color { r: 1.0, g: 0.647, b: 0.0, a: 0.7 };
pub const PURPLE: Color = Color { r: 0.502, g: 0.0, b: 0.502, a: 0.7 };
pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 0.7 };
pub const BROWN: Color = Color { r: 0.647, g: 0.165, b: 0.165, a: 0.7 };
pub const GRAY: Color = Color { r: 0.502, g: 0.502, b: 0.502, a: 0.7 };
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.7 };
pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.7 };
pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<T: Copy + Into<f32>> From<[T; 4]> for Color {
    fn from(v: [T; 4]) -> Self {
        Self { r: v[0].into(), g: v[1].into(), b: v[2].into(), a: v[3].into() }
    }
}

impl<T: Copy + From<f32>> Into<[T; 4]> for Color {
    fn into(self) -> [T; 4] {
        [self.r.into(), self.g.into(), self.b.into(), self.a.into()]
    }
}

impl<T: Copy + From<f32>> Into<[T; 4]> for &Color {
    fn into(self) -> [T; 4] {
        [self.r.into(), self.g.into(), self.b.into(), self.a.into()]
    }
}

impl<T: Copy + Into<f32>> From<[T; 3]> for Color {
    fn from(v: [T; 3]) -> Self {
        Self { r: v[0].into(), g: v[1].into(), b: v[2].into(), a: 1.0 }
    }
}
