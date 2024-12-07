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
